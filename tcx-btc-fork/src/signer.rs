use tcx_chain::{Keystore, TransactionSigner};

use bitcoin::{OutPoint, Script, Transaction, TxIn, TxOut};
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::{sha256d, Hash};

use crate::bip143_with_forkid::SighashComponentsWithForkId;
use crate::Result;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use std::str::FromStr;

use crate::address::BtcForkAddress;
use tcx_primitive::{
    Bip32DeterministicPublicKey, Derive, DerivePath, PrivateKey, PublicKey,
    TypedDeterministicPublicKey, DeterministicPublicKey, FromHex
};

use crate::transaction::{BtcForkSignedTxOutput, BtcForkTxInput, Utxo};
use bitcoin::util::bip143::SighashComponents;
use bitcoin_hashes::hash160;
use bitcoin_hashes::hex::FromHex as HashFromHex;
use bitcoin_hashes::hex::ToHex as HashToHex;
use std::marker::PhantomData;
use tcx_chain::Address;
use tcx_constants::CoinInfo;

const DUST: u64 = 546;
const SIGHASH_ALL: u8 = 0x01;

pub trait ScriptPubKeyComponent {
    fn address_script_like(target_addr: &str, pub_key: &bitcoin::PublicKey) -> Result<Script>;
    fn address_script_pub_key(target_addr: &str) -> Result<Script>;
}

pub struct BitcoinForkSinger<S: ScriptPubKeyComponent + Address, T: BitcoinTransactionSignComponent>
{
    pub tx_input: BtcForkTxInput,
    pub coin_info: CoinInfo,
    pub _marker_s: PhantomData<S>,
    pub _marker_t: PhantomData<T>,
}

impl<S: ScriptPubKeyComponent + Address, T: BitcoinTransactionSignComponent>
    TransactionSigner<BitcoinForkSinger<S, T>, BtcForkSignedTxOutput> for Keystore
{
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &BitcoinForkSinger<S, T>,
    ) -> Result<BtcForkSignedTxOutput> {
        let change_address = if self.determinable() {
            let dpk = self.find_deterministic_public_key(symbol, address)?;
            tx.change_address(&dpk)?
        } else {
            S::address_script_pub_key(&address)?
        };

        let mut sks = vec![];

        for x in tx.tx_input.unspents.iter() {
            if x.derived_path.len() > 0 {
                sks.push(
                    self.find_private_key_by_path(symbol, address, &x.derived_path)?
                        .as_secp256k1()?
                        .clone(),
                );
            } else {
                sks.push(
                    self.find_private_key(symbol, &x.address)?
                        .as_secp256k1()?
                        .clone(),
                );
            }
        }

        tx.sign_transaction(&sks, change_address)
    }
}

impl<S: ScriptPubKeyComponent + Address, T: BitcoinTransactionSignComponent>
    BitcoinForkSinger<S, T>
{
    pub fn new(input: BtcForkTxInput, coin: CoinInfo) -> Self {
        BitcoinForkSinger::<S, T> {
            tx_input: input,
            coin_info: coin,
            _marker_s: PhantomData,
            _marker_t: PhantomData,
        }
    }

    fn receive_script_pubkey(&self) -> Result<Script> {
        S::address_script_pub_key(&self.tx_input.to)
    }

    fn change_address(&self, dpk: &TypedDeterministicPublicKey) -> Result<Script> {
        if !self.tx_input.change_address.is_empty() {
            S::address_script_pub_key(&self.tx_input.change_address)
        } else {
            let from = &self.tx_input.unspents.first().expect("first_utxo").address;
            // todo: address is error
            let _change_path = format!("1/{}", &self.tx_input.change_address_index);
            let pub_key = dpk.public_key().as_secp256k1()?.0;
            S::address_script_like(&from, &pub_key)
        }
    }

    pub fn derive_pub_key_at_path(xpub: &str, child_path: &str) -> Result<bitcoin::PublicKey> {
        let epk = Bip32DeterministicPublicKey::from_hex(xpub)?;

        let index_ext_pub_key = epk.derive(DerivePath::from_str(child_path)?.into_iter())?;

        Ok(index_ext_pub_key.public_key().0)
    }

    fn tx_outs(&self, change_script_pubkey: Script) -> Result<Vec<TxOut>> {
        let mut total_amount = 0;

        for unspent in &self.tx_input.unspents {
            total_amount += unspent.amount;
        }

        ensure!(
            total_amount >= (self.tx_input.amount + self.tx_input.fee),
            "total amount must ge amount + fee"
        );

        let mut tx_outs: Vec<TxOut> = vec![];

        let receive_script_pubkey = self.receive_script_pubkey()?;
        let receiver_tx_out = TxOut {
            value: self.tx_input.amount as u64,
            script_pubkey: receive_script_pubkey,
        };
        tx_outs.push(receiver_tx_out);
        let change_amount = total_amount - self.tx_input.amount - self.tx_input.fee;

        if change_amount >= DUST as i64 {
            let change_tx_out = TxOut {
                value: change_amount as u64,
                script_pubkey: change_script_pubkey,
            };
            tx_outs.push(change_tx_out);
        }
        Ok(tx_outs)
    }

    fn tx_inputs(&self) -> Vec<TxIn> {
        let mut tx_inputs: Vec<TxIn> = vec![];

        for unspent in &self.tx_input.unspents {
            tx_inputs.push(TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.tx_hash).expect("tx_hash"),
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFF_FFFF,
                witness: vec![],
            });
        }
        tx_inputs
    }

    pub fn sign_transaction(
        &self,
        keys: &[impl PrivateKey],
        change_addr_pubkey: Script,
    ) -> Result<BtcForkSignedTxOutput> {
        let tx_outs = self.tx_outs(change_addr_pubkey)?;
        let tx_inputs = self.tx_inputs();
        let tx = Transaction {
            version: T::tx_version(),
            lock_time: 0,
            input: tx_inputs,
            output: tx_outs,
        };

        let signed_tx = T::sign_inputs(&tx, &self.tx_input.unspents, &keys)?;
        let tx_bytes = serialize(&signed_tx);

        Ok(BtcForkSignedTxOutput {
            signature: tx_bytes.to_hex(),
            tx_hash: signed_tx.txid().into_inner().to_hex(),
        })
    }
}

pub trait BitcoinTransactionSignComponent {
    fn sign_inputs(
        tx: &Transaction,
        unspents: &[Utxo],
        keys: &[impl PrivateKey],
    ) -> Result<Transaction>;
    fn tx_version() -> u32;

    fn sign_hash_and_pub_key(
        pri_key: &impl PrivateKey,
        hash: &[u8],
        sign_hash: u8,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let signature_bytes = pri_key.sign(&hash)?;
        let raw_bytes: Vec<u8> = vec![sign_hash];
        let sig_bytes: Vec<u8> = [signature_bytes, raw_bytes].concat();
        let pub_key = pri_key.public_key();
        let pub_key_bytes = pub_key.to_bytes();
        Ok((sig_bytes, pub_key_bytes.to_vec()))
    }
}

pub struct SegWitTransactionSignComponent {}

impl SegWitTransactionSignComponent {
    fn witness_sign(
        tx: &Transaction,
        unspents: &[Utxo],
        keys: &[impl PrivateKey],
    ) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        let shc = SighashComponents::new(&tx);
        for i in 0..tx.input.len() {
            let tx_in = &tx.input[i];
            let unspent = &unspents[i];
            let pub_key = &keys[i].public_key();
            let pub_key_bytes = pub_key.to_bytes();
            let pub_key_hash = hash160::Hash::hash(&pub_key_bytes).into_inner();
            let script_hex = format!("76a914{}88ac", hex::encode(pub_key_hash));
            let script = Script::from(hex::decode(script_hex)?);
            let hash = shc.sighash_all(tx_in, &script, unspent.amount as u64);

            let prv_key = &keys[i];
            witnesses.push(Self::sign_hash_and_pub_key(
                prv_key,
                &hash.into_inner(),
                SIGHASH_ALL,
            )?);
        }
        Ok(witnesses)
    }
}

impl BitcoinTransactionSignComponent for SegWitTransactionSignComponent {
    fn sign_inputs(
        tx: &Transaction,
        unspents: &[Utxo],
        keys: &[impl PrivateKey],
    ) -> Result<Transaction> {
        let _sig_hash_components = SighashComponentsWithForkId::new(&tx);
        let witnesses: Vec<(Vec<u8>, Vec<u8>)> = Self::witness_sign(tx, unspents, keys)?;
        let input_with_sigs = tx
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let pub_key = &keys[i].public_key();
                let pub_key_bytes = pub_key.to_bytes();
                let hash = hash160::Hash::hash(&pub_key_bytes).into_inner();
                let hex = format!("160014{}", hex::encode(&hash));

                TxIn {
                    script_sig: Script::from(hex::decode(hex).expect("script_sig")),
                    witness: vec![witnesses[i].0.clone(), witnesses[i].1.clone()],
                    ..*txin
                }
            })
            .collect();
        Ok(Transaction {
            version: Self::tx_version(),
            lock_time: tx.lock_time,
            input: input_with_sigs,
            output: tx.output.clone(),
        })
    }

    fn tx_version() -> u32 {
        2
    }
}

pub struct LegacyTransactionSignComponent<H: SignHasher> {
    _maker: PhantomData<H>,
}

pub trait SignHasher {
    fn sign_hash(tx: &Transaction, index: usize, unspent: &Utxo) -> Result<(sha256d::Hash, u32)>;
}

pub struct LegacySignHasher {}

impl SignHasher for LegacySignHasher {
    fn sign_hash(tx: &Transaction, index: usize, unspent: &Utxo) -> Result<(sha256d::Hash, u32)> {
        let addr = BtcForkAddress::from_str(&unspent.address)?;
        let script = addr.script_pubkey();
        let hash = tx.signature_hash(index, &script, u32::from(SIGHASH_ALL));
        Ok((hash, u32::from(SIGHASH_ALL)))
    }
}

impl<H: SignHasher> LegacyTransactionSignComponent<H> {
    fn script_sigs_sign(
        tx: &Transaction,
        unspents: &[Utxo],
        keys: &[impl PrivateKey],
    ) -> Result<Vec<Script>> {
        let mut script_sigs: Vec<Script> = vec![];

        for i in 0..tx.input.len() {
            let unspent = &unspents[i];
            let (hash, hash_type) = H::sign_hash(&tx, i, &unspent)?;
            let prv_key = &keys[i];
            let script_sig_and_pub_key =
                Self::sign_hash_and_pub_key(prv_key, &hash.into_inner(), hash_type as u8)?;
            let script = Builder::new()
                .push_slice(&script_sig_and_pub_key.0)
                .push_slice(&script_sig_and_pub_key.1)
                .into_script();
            script_sigs.push(script);
        }
        Ok(script_sigs)
    }
}

impl<H: SignHasher> BitcoinTransactionSignComponent for LegacyTransactionSignComponent<H> {
    fn sign_inputs(
        tx: &Transaction,
        unspents: &[Utxo],
        keys: &[impl PrivateKey],
    ) -> Result<Transaction> {
        let sign_scripts = Self::script_sigs_sign(&tx, unspents, &keys)?;
        let input_with_sigs = tx
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| TxIn {
                script_sig: sign_scripts[i].clone(),
                witness: vec![],
                ..*txin
            })
            .collect();
        Ok(Transaction {
            version: Self::tx_version(),
            lock_time: tx.lock_time,
            input: input_with_sigs,
            output: tx.output.clone(),
        })
    }

    fn tx_version() -> u32 {
        1
    }
}

pub type BtcForkTransaction =
    BitcoinForkSinger<BtcForkAddress, LegacyTransactionSignComponent<LegacySignHasher>>;

pub type BtcForkSegWitTransaction =
    BitcoinForkSinger<BtcForkAddress, SegWitTransactionSignComponent>;

#[cfg(test)]
mod tests {
    use super::*;

    use tcx_constants::coin_info::coin_info_from_param;
    use tcx_primitive::Secp256k1PrivateKey;

    #[test]
    fn test_sign_ltc() {
        let unspents = vec![Utxo {
            tx_hash: "a477af6b2667c29670467e4e0728b685ee07b240235771862318e29ddbe58458".to_string(),
            vout: 0,
            amount: 1000000,
            address: "mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1".to_string(),
            script_pub_key: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac".to_string(),
            derived_path: "0/0".to_string(),
            sequence: 0,
        }];
        let tx_input = BtcForkTxInput {
            to: "mrU9pEmAx26HcbKVrABvgL7AwA5fjNFoDc".to_string(),
            amount: 500000,
            unspents,
            fee: 100000,
            change_address_index: 1u32,
            change_address: "".to_string(),
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let coin_info = coin_info_from_param("LITECOIN", "TESTNET", "NONE").unwrap();
        let tran =
            BitcoinForkSinger::<BtcForkAddress, LegacyTransactionSignComponent<LegacySignHasher>> {
                tx_input,
                coin_info,
                _marker_s: PhantomData,
                _marker_t: PhantomData,
            };

        let prv_key =
            Secp256k1PrivateKey::from_wif("cSBnVM4xvxarwGQuAfQFwqDg9k5tErHUHzgWsEfD4zdwUasvqRVY")
                .unwrap();
        let change_addr = BtcForkAddress::from_str("mgBCJAsvzgT2qNNeXsoECg2uPKrUsZ76up").unwrap();
        let expected = tran
            .sign_transaction(&vec![prv_key], change_addr.script_pubkey())
            .unwrap();
        assert_eq!(expected.signature, "01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006a473044022029063983b2537e4aa15ee838874269a6ba6f5280297f92deb5cd56d2b2db7e8202207e1581f73024a48fce1100ed36a1a48f6783026736de39a4dd40a1ccc75f651101210223078d2942df62c45621d209fab84ea9a7a23346201b7727b9b45a29c4e76f5effffffff0220a10700000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac801a0600000000001976a914073b7eae2823efa349e3b9155b8a735526463a0f88ac00000000");
    }

    #[test]
    fn test_sign_ltc_change_address() {
        let unspents = vec![Utxo {
            tx_hash: "a477af6b2667c29670467e4e0728b685ee07b240235771862318e29ddbe58458".to_string(),
            vout: 0,
            amount: 1000000,
            address: "mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1".to_string(),
            script_pub_key: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac".to_string(),
            derived_path: "0/0".to_string(),
            sequence: 0,
        }];
        let tx_input = BtcForkTxInput {
            to: "mrU9pEmAx26HcbKVrABvgL7AwA5fjNFoDc".to_string(),
            amount: 500000,
            unspents,
            fee: 100000,
            change_address_index: 0,
            change_address: "mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1".to_string(),
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let coin_info = coin_info_from_param("LITECOIN", "TESTNET", "NONE").unwrap();
        let tran =
            BitcoinForkSinger::<BtcForkAddress, LegacyTransactionSignComponent<LegacySignHasher>> {
                tx_input,
                coin_info,
                _marker_s: PhantomData,
                _marker_t: PhantomData,
            };

        let prv_key =
            Secp256k1PrivateKey::from_wif("cSBnVM4xvxarwGQuAfQFwqDg9k5tErHUHzgWsEfD4zdwUasvqRVY")
                .unwrap();
        let change_addr = BtcForkAddress::from_str("mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1").unwrap();
        let actual = tran
            .sign_transaction(&vec![prv_key], change_addr.script_pubkey())
            .unwrap();
        assert_eq!(actual.signature, "01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006b483045022100eefdd6cace70ee64d6a29bca5f52c338b2b3ecf6e6c7b222818c9bba60f094fb022053535e23a77afc7255c18ae8c6e6bf0f8b6e3f552d08519455714cbe59e489cf01210223078d2942df62c45621d209fab84ea9a7a23346201b7727b9b45a29c4e76f5effffffff0220a10700000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac801a0600000000001976a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac00000000");
    }

    #[test]
    fn test_sign_segwit_ltc() {
        let unspents = vec![Utxo {
            tx_hash: "e868b66e75376add2154acb558cf45ff7b723f255e2aca794da1548eb945ba8b".to_string(),
            vout: 1,
            amount: 19850000,
            address: "MV3hqxhhcGxCdeLXpZKRCabtUApRXixgid".to_string(),
            script_pub_key: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac".to_string(),
            derived_path: "1/0".to_string(),
            sequence: 0,
        }];
        let tx_input = BtcForkTxInput {
            to: "M7xo1Mi1gULZSwgvu7VVEvrwMRqngmFkVd".to_string(),
            amount: 19800000,
            unspents,
            fee: 50000,
            change_address_index: 1u32,
            change_address: "".to_string(),
            network: "".to_string(),
            seg_wit: "".to_string(),
        };
        let coin_info = coin_info_from_param("LITECOIN", "MAINNET", "NONE").unwrap();
        let tran = BitcoinForkSinger::<BtcForkAddress, SegWitTransactionSignComponent> {
            tx_input,
            coin_info,
            _marker_s: PhantomData,
            _marker_t: PhantomData,
        };

        let pair = Secp256k1PrivateKey::from_slice(
            &hex::decode("f3731f49d830c109e054522df01a9378383814af5b01a9cd150511f12db39e6e")
                .unwrap(),
        )
        .unwrap();

        let change_addr = BtcForkAddress::from_str("MV3hqxhhcGxCdeLXpZKRCabtUApRXixgid").unwrap();
        let expected = tran
            .sign_transaction(&vec![pair], change_addr.script_pubkey())
            .unwrap();
        assert_eq!(expected.signature, "020000000001018bba45b98e54a14d79ca2a5e253f727bff45cf58b5ac5421dd6a37756eb668e801000000171600147b03478d2f7c984179084baa38f790ed1d37629bffffffff01c01f2e010000000017a91400aff21f24bc08af58e41e4186d8492a10b84f9e8702483045022100d0cc3d94c7b7b34fdcc2adc4fd3f735560407581afd6caa11c8d04b963a048a00220777d98e0122fe97206875f49556a401dfc449739ec30e44cb9ed9b92a0b3ff1b01210209c629c64829ec2e99703600ee86c7161a9ed13213e714726210274c29cf780900000000");
    }
}
