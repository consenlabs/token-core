use tcx_chain::{TxSignResult, TransactionSinger, Metadata, Source, HdKeystore};
use secp256k1::{Secp256k1, Message};
use bitcoin_hashes::Hash;
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::hex::FromHex;
use bitcoin::{Address as BtcAddress, TxOut, TxIn, OutPoint, Script, Transaction};
use bitcoin::{PublicKey as BtcPublicKey};
use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ExtendedPrivKey, ChildNumber};
use crate::bip143_with_forkid::SighashComponentsWithForkId;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use std::str::FromStr;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tcx_chain::keystore::Address;
use serde::export::PhantomData;
use tcx_chain::curve::PrivateKey;
use crate::bch_coin::BchAddress;
use crate::bch_coin::BchTestNetAddress;
use tcx_chain::curve::PublicKey;

const DUST: u64 = 546;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
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
    fn collect_prv_keys_paths(&self, path: &str) -> Result<Vec<String>> {
        let mut paths: Vec<String> = vec![];
        paths.push(format!("{}/0/{}", path, &self.change_idx));
        for unspent in &self.unspents {
            let derived_path = unspent.derived_path.trim();
            let path_with_space = derived_path.replace("/", " ");

            let path_idxs: Vec<&str> = path_with_space.split(" ").collect();
            ensure!(path_idxs.len() == 2, "derived path must be x/x");

            paths.push(format!("{}/{}", path, derived_path));
        }
        Ok(paths)
    }

    fn network(&self) -> Network {
        let unspent = self.unspents.first().expect("empty_unspents");
        match BchAddress::is_main_net(&unspent.address) {
            true => Network::Bitcoin,
            _ => Network::Testnet
        }
    }

    fn sign_hash(&self, pri_key: &impl PrivateKey, hash: &[u8]) -> Result<Script> {
        let signature_bytes = pri_key.sign(&hash)?;
        let raw_bytes: Vec<u8> = vec![0x41];
        let sig_bytes: Vec<u8> = [signature_bytes, raw_bytes].concat();
        let pub_key_bytes = pri_key.public_key().to_bytes();
        Ok(Builder::new().push_slice(&sig_bytes).push_slice(&pub_key_bytes).into_script())
    }

    pub fn sign_transaction(&self, prv_keys: &[impl PrivateKey]) -> Result<TxSignResult> {
        let mut total_amount = 0;

        for unspent in &self.unspents {
            total_amount += unspent.amount;
        }

        ensure!(total_amount >= (self.amount + self.fee), "total amount must ge amount + fee");

        let change_addr_prv_key = prv_keys.first().ok_or(format_err!("get_change_addr_prv_key_failed"))?;
        let change_addr_pub_key = change_addr_prv_key.public_key();
        let pub_key = BtcPublicKey::from_slice(&change_addr_pub_key.to_compressed())?;
        let change_addr = BtcAddress::p2pkh(&pub_key, self.network());

        let mut tx_outs: Vec<TxOut> = vec![];
        let receiver_addr = BtcAddress::from_str(&self.to)?;
        let receiver_tx_out = TxOut {
            value: self.amount as u64,
            script_pubkey: receiver_addr.script_pubkey(),
        };
        tx_outs.push(receiver_tx_out);
        let change_amount = (total_amount - self.amount - self.fee);

        if change_amount > DUST as i64 {
            let change_tx_out = TxOut {
                value: change_amount as u64,
                script_pubkey: change_addr.script_pubkey(),
            };
            tx_outs.push(change_tx_out);
        }

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


        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: tx_inputs,
            output: tx_outs,
        };

        let sig_hash_components = SighashComponentsWithForkId::new(&tx);

        let mut script_sigs: Vec<Script> = vec![];
        for i in 0..tx.input.len() {
            let tx_in = &tx.input[i];
            let unspent = &self.unspents[i];
            let script_bytes: Vec<u8> = FromHex::from_hex(&unspent.script_pub_key).unwrap();
            let script = Builder::from(script_bytes).into_script();
            let shc_hash = sig_hash_components.sighash_all(tx_in, &script, unspent.amount as u64, 0x01 | 0x40);
            let prv_key = &prv_keys[i];
            script_sigs.push(self.sign_hash(prv_key, &shc_hash.into_inner())?);
        }

        let signed_tx = Transaction {
            version: tx.version,
            lock_time: tx.lock_time,
            input: tx.input.iter().enumerate().map(|(i, txin)| TxIn { script_sig: script_sigs[i].clone(), witness: vec![], ..*txin }).collect(),
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

pub struct BitcoinCashSinger {}

impl TransactionSinger for BitcoinCashSinger {
    fn sign_transaction(&self, json: &str, keystore: &HdKeystore, password: &str) -> Result<String> {
        let v: Value = serde_json::from_str(json).expect("sign_transaction_json");
        let unspents: Vec<Utxo> = serde_json::from_value(v["outputs"].clone()).expect("outputs");
        let internal_used = v["internalUsed"].as_i64().expect("internalUsed");
        let change_idx = internal_used + 1;
        let to = v["to"].as_str().expect("to");
        let amount = v["amount"].as_str().expect("amount").parse::<i64>().unwrap();
        let fee = v["fee"].as_str().expect("fee").parse::<i64>().unwrap();
        let password = v["password"].as_str().expect("password");
        let account = keystore.account(&"BCH").ok_or(format_err!("account_not_found"))?;
        let path = &account.derivation_path;

        let bch_tran = BitcoinCashTransaction {
            to: to.to_owned(),
            amount,
            unspents,
            memo: "".to_string(),
            fee,
            change_idx: change_idx as u32,
        };
        let paths = bch_tran.collect_prv_keys_paths(path)?;
        let priv_keys = &keystore.key_at_paths("BCH", &paths, password)?;

        let ret = bch_tran.sign_transaction(&priv_keys)?;
        Ok(serde_json::to_string(&ret)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Language;
    //    use crate::hd_mnemonic_keystore::HdMnemonicKeystore;
    use tcx_chain::HdKeystore;
    use tcx_chain::keystore::CoinInfo;
    use tcx_chain::curve::{CurveType, PublicKeyType};

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static BCH_MAIN_PATH: &'static str = "m/44'/145'/0'";
    static WIF: &'static str = "L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy";

    //
    #[test]
    pub fn bch_signer() {
        let meta = Metadata::default();
        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let coin_info = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: "m/44'/145'/0'".to_string(),
            curve: CurveType::SECP256k1,
            pub_key_type: PublicKeyType::SECP256k1,
        };
        let _ = keystore.derive_coin::<BchAddress>(&coin_info, &PASSWORD);
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

        let singer = BitcoinCashSinger {};
        let paths = tran.collect_prv_keys_paths(&coin_info.derivation_path).unwrap();
        let priv_keys = keystore.key_at_paths("BCH", &paths, &PASSWORD).unwrap();
        let sign_ret = tran.sign_transaction( &priv_keys).unwrap();
        // todo: not a real test data, it's works at WIF: L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy
        assert_eq!(sign_ret.signature, "01000000018689302ea03ef5dd56fb7940a867f9240fa811eddeb0fa4c87ad9ff3728f5e11000000006b483045022100c9df637109b43c88f4c3d68c2ace39fe454b9e239779adaceb273a2e5cc3494e02204fdc62c9792adb46e9f056eea6147f6776193bec380de86ef2959a77a226588841210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff01983a0000000000001976a914ad618cf4333b3b248f9744e8e81db2964d0ae39788ac00000000");
    }
}