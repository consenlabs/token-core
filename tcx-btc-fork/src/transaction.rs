use tcx_chain::{HdKeystore, TransactionSigner, TxSignResult};

use bitcoin::{OutPoint, Script, Transaction, TxIn, TxOut};
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::{sha256d, Hash};
use tcx_chain::Transaction as TraitTransaction;

use crate::bip143_with_forkid::SighashComponentsWithForkId;
use crate::Result;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::address::BtcForkAddress;
//use tcx_chain::curve::PrivateKey;
use tcx_primitive::{ArbitraryNetworkExtendedPubKey, Pair};

use crate::ExtendedPubKeyExtra;
use bitcoin::util::bip143::SighashComponents;
use bitcoin_hashes::hash160;
use secp256k1::Secp256k1;
use std::marker::PhantomData;
use tcx_chain::bips::{get_account_path, relative_path_to_child_nums};
//use tcx_chain::curve::PublicKey;
use tcx_chain::keystore::Address;
use tcx_primitive::Public;
//use serde::

const DUST: u64 = 546;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Utxo {
    pub tx_hash: String,
    pub vout: i32,
    #[serde(with = "string")]
    pub amount: i64,
    pub address: String,
    pub script_pub_key: String,
    pub derived_path: String,
    #[serde(default)]
    pub sequence: i64,
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub trait ScriptPubKeyComponent {
    fn address_like(target_addr: &str, pub_key: &bitcoin::PublicKey) -> Result<Script>;
    fn address_script_pub_key(target_addr: &str) -> Result<Script>;
}

#[derive(Serialize)]
pub struct BitcoinForkTransaction<
    S: ScriptPubKeyComponent + Address,
    T: BitcoinTransactionSignComponent,
> {
    pub to: String,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub memo: String,
    pub fee: i64,
    pub change_idx: Option<u32>,
    pub change_address: Option<String>,
    pub coin: String,
    #[serde(skip_serializing)]
    pub _marker_s: PhantomData<S>,
    #[serde(skip_serializing)]
    pub _marker_t: PhantomData<T>,
}

impl<S: ScriptPubKeyComponent + Address, T: BitcoinTransactionSignComponent> TraitTransaction
    for BitcoinForkTransaction<S, T>
{
}

impl<S: ScriptPubKeyComponent + Address, T: BitcoinTransactionSignComponent>
    TransactionSigner<BitcoinForkTransaction<S, T>, TxSignResult> for HdKeystore
{
    fn sign_transaction(
        &self,
        tx: &BitcoinForkTransaction<S, T>,
        password: Option<&str>,
    ) -> Result<TxSignResult> {
        let account = self
            .account(tx.coin.to_uppercase().as_str())
            .ok_or(format_err!("account_not_found"))?;
        let path = &account.derivation_path;
        let extra = ExtendedPubKeyExtra::<S>::from(account.extra.clone());

        let paths = tx.collect_prv_keys_paths(path)?;

        tcx_ensure!(password.is_some(), tcx_crypto::Error::InvalidPassword);
        let priv_keys =
            &self.key_at_paths(tx.coin.to_uppercase().as_str(), &paths, password.unwrap())?;

        let xpub = extra.xpub()?;
        let change_addr = tx.change_address(&xpub)?;
        tx.sign_transaction(&priv_keys, change_addr)
    }
}

impl<S: ScriptPubKeyComponent + Address, T: BitcoinTransactionSignComponent>
    BitcoinForkTransaction<S, T>
{
    pub fn new(
        to: String,
        amount: i64,
        unspents: Vec<Utxo>,
        fee: i64,
        change_idx: Option<u32>,
        change_address: Option<String>,
        coin: String,
    ) -> Self {
        BitcoinForkTransaction::<S, T> {
            to,
            amount,
            unspents,
            memo: "".to_string(),
            fee,
            change_idx,
            change_address,
            coin,
            _marker_s: PhantomData,
            _marker_t: PhantomData,
        }
    }

    fn collect_prv_keys_paths(&self, path: &str) -> Result<Vec<String>> {
        let mut paths: Vec<String> = vec![];
        let account_path = get_account_path(path)?;

        for unspent in &self.unspents {
            let derived_path = unspent.derived_path.trim();
            let path_with_space = derived_path.replace("/", " ");

            let path_idxs: Vec<&str> = path_with_space.split(" ").collect();
            ensure!(path_idxs.len() == 2, "derived path must be x/x");

            paths.push(format!("{}/{}", account_path, derived_path));
        }
        Ok(paths)
    }

    fn receive_script_pubkey(&self) -> Result<Script> {
        S::address_script_pub_key(&self.to)
    }

    fn change_address(&self, xpub: &str) -> Result<Script> {
        if let Some(change_addr) = &self.change_address {
            S::address_script_pub_key(change_addr)
        } else {
            let from = &self.unspents.first().expect("first_utxo").address;
            let change_path = format!("0/{}", &self.change_idx.expect("change_idx"));
            let pub_key = Self::derive_pub_key_at_path(&xpub, &change_path)?;
            S::address_like(&from, &pub_key)
        }
    }

    pub fn derive_pub_key_at_path(xpub: &str, child_path: &str) -> Result<bitcoin::PublicKey> {
        let ext_pub_key = ArbitraryNetworkExtendedPubKey::from_str(xpub)?;
        let s = Secp256k1::new();
        let child_nums = relative_path_to_child_nums(child_path)?;
        let index_ext_pub_key = ext_pub_key.extended_pub_key.derive_pub(&s, &child_nums)?;
        Ok(index_ext_pub_key.public_key)
    }

    fn tx_outs(&self, change_script_pubkey: Script) -> Result<Vec<TxOut>> {
        let mut total_amount = 0;

        for unspent in &self.unspents {
            total_amount += unspent.amount;
        }

        ensure!(
            total_amount >= (self.amount + self.fee),
            "total amount must ge amount + fee"
        );

        let mut tx_outs: Vec<TxOut> = vec![];

        let receive_script_pubkey = self.receive_script_pubkey()?;
        let receiver_tx_out = TxOut {
            value: self.amount as u64,
            script_pubkey: receive_script_pubkey,
        };
        tx_outs.push(receiver_tx_out);
        let change_amount = total_amount - self.amount - self.fee;

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

        for unspent in &self.unspents {
            tx_inputs.push(TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.tx_hash).expect("tx_hash"),
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF,
                witness: vec![],
            });
        }
        tx_inputs
    }

    pub fn sign_transaction(
        &self,
        prv_keys: &[impl Pair],
        change_addr_pubkey: Script,
    ) -> Result<TxSignResult> {
        let tx_outs = self.tx_outs(change_addr_pubkey)?;
        let tx_inputs = self.tx_inputs();
        let tx = Transaction {
            version: T::tx_version(),
            lock_time: 0,
            input: tx_inputs,
            output: tx_outs,
        };

        let signed_tx = T::sign_inputs(&tx, &self.unspents, &prv_keys)?;
        let tx_bytes = serialize(&signed_tx);

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: signed_tx.txid().into_inner().to_hex(),
            wtx_id: "".to_string(),
        })
    }
}

pub trait BitcoinTransactionSignComponent {
    fn sign_inputs(
        tx: &Transaction,
        unspents: &[Utxo],
        prv_keys: &[impl Pair],
    ) -> Result<Transaction>;
    fn tx_version() -> u32;

    fn sign_hash_and_pub_key(
        pri_key: &impl Pair,
        hash: &[u8],
        sign_hash: u8,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let signature_bytes = pri_key.sign(&hash)?;
        let raw_bytes: Vec<u8> = vec![sign_hash];
        let sig_bytes: Vec<u8> = [signature_bytes, raw_bytes].concat();
        let pub_key = pri_key.public();
        let pub_key_bytes = pub_key.as_slice();
        Ok((sig_bytes, pub_key_bytes.to_vec()))
    }
}

pub struct SegWitTransactionSignComponent {}

impl SegWitTransactionSignComponent {
    fn witness_sign(
        tx: &Transaction,
        unspents: &[Utxo],
        prv_keys: &[impl Pair],
    ) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut witnesses: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        let shc = SighashComponents::new(&tx);
        for i in 0..tx.input.len() {
            let tx_in = &tx.input[i];
            let unspent = &unspents[i];
            let pub_key = &prv_keys[i].public_key();
            let pub_key_hash = hash160::Hash::hash(&pub_key.as_slice()).into_inner();
            let script_hex = format!("76a914{}88ac", hex::encode(pub_key_hash));
            let script = Script::from(hex::decode(script_hex)?);
            let hash = shc.sighash_all(tx_in, &script, unspent.amount as u64);

            let prv_key = &prv_keys[i];
            witnesses.push(Self::sign_hash_and_pub_key(
                prv_key,
                &hash.into_inner(),
                0x01,
            )?);
        }
        Ok(witnesses)
    }
}

impl BitcoinTransactionSignComponent for SegWitTransactionSignComponent {
    fn sign_inputs(
        tx: &Transaction,
        unspents: &[Utxo],
        prv_keys: &[impl Pair],
    ) -> Result<Transaction> {
        let _sig_hash_components = SighashComponentsWithForkId::new(&tx);
        let witnesses: Vec<(Vec<u8>, Vec<u8>)> = Self::witness_sign(tx, unspents, prv_keys)?;
        let input_with_sigs = tx
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let pub_key = &prv_keys[i].public_key();
                let hash = hash160::Hash::hash(&pub_key.as_slice()).into_inner();
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
        let hash = tx.signature_hash(index, &script, 0x01);
        Ok((hash, 0x01))
    }
}

impl<H: SignHasher> LegacyTransactionSignComponent<H> {
    fn script_sigs_sign(
        tx: &Transaction,
        unspents: &[Utxo],
        prv_keys: &[impl Pair],
    ) -> Result<Vec<Script>> {
        let mut script_sigs: Vec<Script> = vec![];

        for i in 0..tx.input.len() {
            let unspent = &unspents[i];
            let (hash, hash_type) = H::sign_hash(&tx, i, &unspent)?;
            let prv_key = &prv_keys[i];
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
        prv_keys: &[impl Pair],
    ) -> Result<Transaction> {
        let sign_scripts = Self::script_sigs_sign(&tx, unspents, &prv_keys)?;
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
    BitcoinForkTransaction<BtcForkAddress, LegacyTransactionSignComponent<LegacySignHasher>>;

pub type BtcForkSegWitTransaction =
    BitcoinForkTransaction<BtcForkAddress, SegWitTransactionSignComponent>;

#[cfg(test)]
mod tests {
    use super::*;

    use bitcoin::network::constants::Network;
    use secp256k1::SecretKey;

    use tcx_chain::Secp256k1PrivateKey;

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
        let tran = BitcoinForkTransaction::<
            BtcForkAddress,
            LegacyTransactionSignComponent<LegacySignHasher>,
        > {
            to: "mrU9pEmAx26HcbKVrABvgL7AwA5fjNFoDc".to_string(),
            amount: 500000,
            unspents,
            memo: "".to_string(),
            fee: 100000,
            change_idx: Some(1u32),
            change_address: None,
            coin: "LITECOIN-TESTNET".to_string(),
            _marker_s: PhantomData,
            _marker_t: PhantomData,
        };

        let prv_key =
            Secp256k1PrivateKey::from_wif("cSBnVM4xvxarwGQuAfQFwqDg9k5tErHUHzgWsEfD4zdwUasvqRVY")
                .unwrap();
        let change_addr = BtcForkAddress::from_str("mgBCJAsvzgT2qNNeXsoECg2uPKrUsZ76up").unwrap();
        //        let sign_ret = keystore.sign_transaction(&tran, Some(&PASSWORD)).unwrap();
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
        let tran = BitcoinForkTransaction::<
            BtcForkAddress,
            LegacyTransactionSignComponent<LegacySignHasher>,
        > {
            to: "mrU9pEmAx26HcbKVrABvgL7AwA5fjNFoDc".to_string(),
            amount: 500000,
            unspents,
            memo: "".to_string(),
            fee: 100000,
            change_idx: None,
            change_address: Some("mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1".to_string()),
            coin: "LITECOIN-TESTNET".to_string(),
            _marker_s: PhantomData,
            _marker_t: PhantomData,
        };

        let prv_key =
            Secp256k1PrivateKey::from_wif("cSBnVM4xvxarwGQuAfQFwqDg9k5tErHUHzgWsEfD4zdwUasvqRVY")
                .unwrap();
        let change_addr = BtcForkAddress::from_str("mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1").unwrap();
        //        let sign_ret = keystore.sign_transaction(&tran, Some(&PASSWORD)).unwrap();
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
        let tran = BitcoinForkTransaction::<BtcForkAddress, SegWitTransactionSignComponent> {
            to: "M7xo1Mi1gULZSwgvu7VVEvrwMRqngmFkVd".to_string(),
            amount: 19800000,
            unspents,
            memo: "".to_string(),
            fee: 50000,
            change_idx: Some(1u32),
            change_address: None,
            coin: "LITECOIN".to_string(),
            _marker_s: PhantomData,
            _marker_t: PhantomData,
        };
        //
        let prv_key = Secp256k1PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: SecretKey::from_slice(
                &hex::decode("f3731f49d830c109e054522df01a9378383814af5b01a9cd150511f12db39e6e")
                    .unwrap(),
            )
            .unwrap(),
        };
        let change_addr = BtcForkAddress::from_str("MV3hqxhhcGxCdeLXpZKRCabtUApRXixgid").unwrap();
        let expected = tran
            .sign_transaction(&vec![prv_key], change_addr.script_pubkey())
            .unwrap();
        assert_eq!(expected.signature, "020000000001018bba45b98e54a14d79ca2a5e253f727bff45cf58b5ac5421dd6a37756eb668e801000000171600147b03478d2f7c984179084baa38f790ed1d37629bffffffff01c01f2e010000000017a91400aff21f24bc08af58e41e4186d8492a10b84f9e8702483045022100d0cc3d94c7b7b34fdcc2adc4fd3f735560407581afd6caa11c8d04b963a048a00220777d98e0122fe97206875f49556a401dfc449739ec30e44cb9ed9b92a0b3ff1b01210209c629c64829ec2e99703600ee86c7161a9ed13213e714726210274c29cf780900000000");
    }
}
