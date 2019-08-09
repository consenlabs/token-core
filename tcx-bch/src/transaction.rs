use tcx_chain::{TxSignResult, Secp256k1Curve};

use bitcoin::{Address as BtcAddress, OutPoint, Script, Transaction, TxIn, TxOut};
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::Hash;

use bitcoin::network::constants::Network;

use crate::bip143_with_forkid::SighashComponentsWithForkId;
use crate::Result;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::address::BchAddress;
use tcx_chain::curve::{PrivateKey, Secp256k1PublicKey};

use tcx_chain::bips::get_account_path;
use tcx_chain::curve::PublicKey;
use bitcoin::util::bip32::ExtendedPubKey;

const DUST: u64 = 546;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Utxo {
    pub tx_hash: String,
    pub vout: i32,
    pub amount: i64,
    pub address: String,
    pub script_pub_key: String,
    pub derived_path: String,
    pub sequence: i64,
}

pub struct BitcoinCashTransaction {
    pub to: String,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub memo: String,
    pub fee: i64,
    pub change_idx: u32,
}

impl BitcoinCashTransaction {
    pub fn collect_prv_keys_paths(&self, path: &str) -> Result<Vec<String>> {
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

    fn network(&self) -> Network {
        let unspent = self.unspents.first().expect("empty_unspents");
        match BchAddress::is_main_net(&unspent.address) {
            true => Network::Bitcoin,
            _ => Network::Testnet,
        }
    }

    fn sign_hash(&self, pri_key: &impl PrivateKey, hash: &[u8]) -> Result<Script> {
        let signature_bytes = pri_key.sign(&hash)?;
        let raw_bytes: Vec<u8> = vec![0x41];
        let sig_bytes: Vec<u8> = [signature_bytes, raw_bytes].concat();
        let pub_key_bytes = pri_key.public_key().to_bytes();
        Ok(Builder::new()
            .push_slice(&sig_bytes)
            .push_slice(&pub_key_bytes)
            .into_script())
    }

    fn address_from_priv_key(&self, priv_key: &impl PrivateKey) -> Result<BtcAddress> {
        let pub_key = priv_key.public_key();
        let secp_pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let change_addr = BtcAddress::p2pkh(&secp_pub_key, self.network());
        Ok(change_addr)
    }

    fn change_addr(&self, xpub: &str) -> Result<BtcAddress> {
        let change_path = format!("0/{}", &self.change_idx);
        let pub_key = Secp256k1Curve::derive_pub_key_at_path(&xpub, &change_path)?;
        Ok(BtcAddress::p2pkh(&pub_key, self.network()))
    }

    fn tx_outs(&self, change_addr: &BtcAddress) -> Result<Vec<TxOut>> {
        let mut total_amount = 0;

        for unspent in &self.unspents {
            total_amount += unspent.amount;
        }

        ensure!(
            total_amount >= (self.amount + self.fee),
            "total amount must ge amount + fee"
        );

        let mut tx_outs: Vec<TxOut> = vec![];
        let receiver_addr = BtcAddress::from_str(&self.to)?;
        let receiver_tx_out = TxOut {
            value: self.amount as u64,
            script_pubkey: receiver_addr.script_pubkey(),
        };
        tx_outs.push(receiver_tx_out);
        let change_amount = total_amount - self.amount - self.fee;

        if change_amount > DUST as i64 {
            let change_tx_out = TxOut {
                value: change_amount as u64,
                script_pubkey: change_addr.script_pubkey(),
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
                    txid: Hash256::from_hex(&unspent.tx_hash).unwrap(),
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF,
                witness: vec![],
            });
        }
        tx_inputs
    }

    fn script_sigs(&self, tx: &Transaction, shc: &SighashComponentsWithForkId, prv_keys: &[impl PrivateKey]) -> Result<Vec<Script>> {
        let mut script_sigs: Vec<Script> = vec![];
        for i in 0..tx.input.len() {
            let tx_in = &tx.input[i];
            let unspent = &self.unspents[i];
            let script_bytes: Vec<u8> = FromHex::from_hex(&unspent.script_pub_key).unwrap();
            let script = Builder::from(script_bytes).into_script();
            let shc_hash =
                shc.sighash_all(tx_in, &script, unspent.amount as u64, 0x01 | 0x40);
            let prv_key = &prv_keys[i];
            script_sigs.push(self.sign_hash(prv_key, &shc_hash.into_inner())?);
        }
        Ok(script_sigs)
    }

    pub fn sign_transaction(&self, prv_keys: &[impl PrivateKey], xpub: &str) -> Result<TxSignResult> {

//        let change_addr_prv_key = prv_keys
//            .first()
//            .ok_or(format_err!("get_change_addr_prv_key_failed"))?;
//        let change_addr = self.address_from_priv_key(change_addr_prv_key)?;
        let change_addr = self.change_addr(xpub)?;
        let tx_outs = self.tx_outs(&change_addr)?;
        let tx_inputs = self.tx_inputs();

        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: tx_inputs,
            output: tx_outs,
        };

        let sig_hash_components = SighashComponentsWithForkId::new(&tx);

        let script_sigs: Vec<Script> = self.script_sigs(&tx, &sig_hash_components, &prv_keys)?;

        let signed_tx = Transaction {
            version: tx.version,
            lock_time: tx.lock_time,
            input: tx
                .input
                .iter()
                .enumerate()
                .map(|(i, txin)| TxIn {
                    script_sig: script_sigs[i].clone(),
                    witness: vec![],
                    ..*txin
                })
                .collect(),
            output: tx.output.clone(),
        };

        let tx_bytes = serialize(&signed_tx);

        Ok(TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: signed_tx.txid().into_inner().to_hex(),
            wtx_id: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExtendedPubKeyExtra;
    use tcx_chain::curve::CurveType;
    use tcx_chain::keystore::CoinInfo;
    use tcx_chain::{HdKeystore, Metadata};

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static BCH_MAIN_PATH: &'static str = "m/44'/145'/0'";


    //
    #[test]
    pub fn bch_signer() {
        let meta = Metadata::default();
        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let coin_info = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        };
        let _ = keystore.derive_coin::<BchAddress, ExtendedPubKeyExtra>(&coin_info, &PASSWORD);
        let unspents = vec![Utxo {
            tx_hash: "115e8f72f39fad874cfab0deed11a80f24f967a84079fb56ddf53ea02e308986".to_string(),
            vout: 0,
            amount: 50000,
            address: "17XBj6iFEsf8kzDMGQk5ghZipxX49VXuaV".to_string(),
            script_pub_key: "76a91447862fe165e6121af80d5dde1ecb478ed170565b88ac".to_string(),
            derived_path: "0/1".to_string(),
            sequence: 0,
        }];
        let tran = BitcoinCashTransaction {
            to: "1Gokm82v6DmtwKEB8AiVhm82hyFSsEvBDK".to_string(),
            amount: 15000,
            unspents,
            memo: "".to_string(),
            fee: 35000,
            change_idx: 0,
        };

        let paths = tran
            .collect_prv_keys_paths(&coin_info.derivation_path)
            .unwrap();
        let priv_keys = keystore.key_at_paths("BCH", &paths, &PASSWORD).unwrap();
        let acc = keystore.account("BCH").unwrap();
        let extra = ExtendedPubKeyExtra::from(acc.extra.clone());

        let sign_ret = tran.sign_transaction(&priv_keys, &extra.xpub).unwrap();
        // todo: not a real test data, it's works at WIF: L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy
        assert_eq!(sign_ret.signature, "01000000018689302ea03ef5dd56fb7940a867f9240fa811eddeb0fa4c87ad9ff3728f5e11000000006b483045022100be283eb3c936fbdc9159d7067cf3bf44b40c5fc790e6f06368c404a6c1962ebb022071741ed6e1d034f300d177582c870934d4b155d0eb40e6eda99b3e95323a4666412102cc987e200a13c771d9c840cd08db93debf4d4443cec3e084a4cde2aad4cfa77dffffffff01983a0000000000001976a914ad618cf4333b3b248f9744e8e81db2964d0ae39788ac00000000");
    }
}
