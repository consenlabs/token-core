use tcx_chain::curve::{Curve, Secp256k1Curve};
use tcx_chain::{Coin, HdKeystore, Account};
use crate::errors::Result;
use bitcoin::network::constants::Network;
use bitcoin::{Address, PublicKey, PrivateKey};
use tcx_chain::keystore::KeyType;
use secp256k1::{SecretKey, Secp256k1};
use bitcoin_hashes::hex::ToHex;
use serde_json::Value;
use crate::bch_transaction::{Utxo, BitcoinCashTransaction};
use std::str::FromStr;

pub struct BchCoin<'a> {
    curve: Secp256k1Curve,
    derivation_path: String,
    keystore: &'a HdKeystore,
}

impl<'a> BchCoin<'a> {
    const SYMBOL: &'static str = "BCH";
    fn load(keystore: &HdKeystore) -> BchCoin {
        BchCoin {
            curve: Secp256k1Curve::new(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            keystore,
        }
    }
}


impl<'a> BchCoin<'a> {
    fn append_account(&self, password: &str) -> Result<Account> {
        let seed = self.keystore.seed(password)?;
        let main_key = self.curve.key_at_path(&self.derivation_path, &seed)?;
        let address = self.derive_address(&main_key)?;
        let xpub = self.curve.extended_pub_key(&self.derivation_path, &seed)?;
//        pub address: String,
//        pub derivation_path: String,
//        pub extended_public_key: String,
//        pub coin: String,
//        #[serde(skip_deserializing)]
//        pub extra: String,
        Ok(Account {
            derivation_path: self.derivation_path.clone(),
            extended_public_key: xpub,
            coin: BchCoin::SYMBOL.to_string(),
            address,
            extra: "".to_string(),
        })
    }

    fn key(&self, password: &str) -> Result<Vec<u8>> {
        let seed = self.keystore.seed(password)?;
        Ok(self.curve.key_at_path(&self.derivation_path, &seed)?)
    }

    fn derive_address(&self, prv_key: &[u8]) -> Result<String> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(prv_key)?;
        let pub_key = PublicKey {
            compressed: true,
            key: secp256k1::PublicKey::from_secret_key(&secp, &secret_key)
        };
        // todo network
        Ok(Address::p2wpkh(&pub_key, Network::Bitcoin).to_string())
    }

    fn extended_private_key(&self, password: &str) -> Result<String>{
        let seed = self.keystore.seed(password)?;
        Ok(self.curve.extended_prv_key(&self.derivation_path, &seed)?)
    }

    fn extended_public_key(&self) -> String {
        let mut iter = self.keystore.active_accounts.iter();
        match iter.find(|a| a.coin == BchCoin::SYMBOL && a.derivation_path == self.derivation_path) {
            Some(acc) => acc.extended_public_key.to_owned(),
            _ => "".to_string()
        }
    }

    fn sign_transaction(&self, json: &str) -> Result<String> {
        let v: Value = serde_json::from_str(json).unwrap();
        let unspents: Vec<Utxo> = serde_json::from_value(v["outputs"].clone()).unwrap();
        let internal_used = v["internalUsed"].as_i64().unwrap();
        let change_idx = internal_used + 1;
        let to = v["to"].as_str().unwrap();
        let amount = v["amount"].as_str().unwrap().parse::<i64>().unwrap();
        let fee = v["fee"].as_str().unwrap().parse::<i64>().unwrap();
        let password = v["password"].as_str().unwrap();
        let chain_id = v["chainId"].as_str().unwrap();
        let xprv = self.extended_private_key(password)?;
        let bch_tran = BitcoinCashTransaction {
            to: to.to_owned(),
            amount,
            unspents,
            memo: "".to_string(),
            fee,
            change_idx: change_idx as u32,
        };

        let ret = bch_tran.sign_transaction(chain_id, &xprv)?;
        Ok(serde_json::to_string(&ret)?)
    }
}

struct BchAddress {

}

impl BchAddress {
    fn is_valid(addr: &str) -> bool {
        Address::from_str(addr).is_ok()
    }

    fn from_public_key(pub_key: &[u8]) -> Result<String> {
        let pub_key = PublicKey::from_slice(pub_key)?;
        Ok(Address::p2wpkh(&pub_key, Network::Bitcoin).to_string())
    }

}




