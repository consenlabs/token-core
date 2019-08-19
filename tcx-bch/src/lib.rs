use bip39::Seed;

pub mod address;
pub mod bip143_with_forkid;
pub mod hard_wallet_keystore;
pub mod transaction;

use core::result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tcx_chain::curve::{CurveType, Secp256k1Curve};
use tcx_chain::keystore::Address;
use tcx_chain::keystore::{CoinInfo, Extra};

#[macro_use]
extern crate failure;

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

pub type Result<T> = result::Result<T, failure::Error>;

pub use address::{BchAddress, BchTestNetAddress};
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey};
use secp256k1::Secp256k1;
use serde_json::Value;
use tcx_crypto::aes::cbc::encrypt_pkcs7;
use tcx_crypto::aes::ctr::encrypt_nopadding;
pub use transaction::{BitcoinCashTransaction, Utxo};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "bch_convert_to_legacy_address_failed# address: {}", _0)]
    ConvertToLegacyAddressFailed(String),
    #[fail(display = "bch_convert_to_cash_address_failed# address: {}", _0)]
    ConvertToCashAddressFailed(String),
    #[fail(display = "construct_bch_address_failed# address: {}", _0)]
    ConstructBchAddressFailed(String),
}

const SYMBOL: &'static str = "BCH";
const PATH: &'static str = "m/44'/145'/0'/0/0";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAddress {
    pub address: String,
    #[serde(rename = "type")]
    pub addr_type: String,
    pub derived_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedPubKeyExtra {
    pub xpub: String,
}

impl Extra for ExtendedPubKeyExtra {
    fn from(coin_info: &CoinInfo, seed: &Seed) -> Result<Self> {
        ensure!(
            coin_info.curve == CurveType::SECP256k1,
            "BCH must be at secp256k1"
        );
        let derivation_info = Secp256k1Curve::extended_pub_key(&coin_info.derivation_path, &seed)?;
        let xpub = address::BchAddress::extended_public_key(&derivation_info);
        Ok(ExtendedPubKeyExtra { xpub })
    }
}

impl ExtendedPubKeyExtra {
    pub fn enc_xpub(&self, key: &str, iv: &str) -> Result<String> {
        let key_bytes = hex::decode(key)?;
        let iv_bytes = hex::decode(iv)?;
        let encrypted = encrypt_pkcs7(&self.xpub.as_bytes(), &key_bytes, &iv_bytes);
        Ok(base64::encode(&encrypted))
    }

    pub fn calc_external_address<A: Address>(&self, idx: i64) -> Result<ExternalAddress> {
        let extended_pub_key = ExtendedPubKey::from_str(&self.xpub)?;
        let s = Secp256k1::new();
        let index_pub = extended_pub_key.derive_pub(
            &s,
            &vec![
                ChildNumber::from_normal_idx(0).unwrap(),
                ChildNumber::from_normal_idx(idx as u32).unwrap(),
            ],
        )?;
        let address = A::from_public_key(&index_pub.public_key)?;
        Ok(ExternalAddress {
            address,
            addr_type: "EXTERNAL".to_string(),
            derived_path: format!("0/{}", idx).to_string(),
        })
    }
}

impl From<Value> for ExtendedPubKeyExtra {
    fn from(v: Value) -> Self {
        serde_json::from_value::<Self>(v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::address::BchAddress;
    use crate::ExtendedPubKeyExtra;
    use serde_json::Value;
    use tcx_chain::curve::CurveType;
    use tcx_chain::keystore::CoinInfo;
    use tcx_chain::{HdKeystore, Metadata};

    const PASSWORD: &str = "Insecure Password";
    const BIP_PATH: &str = "m/44'/145'/0'";
    const MNEMONIC: &str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    fn bch_create() {
        let mut meta = Metadata::default();
        meta.name = "CreateTest".to_string();

        let mut keystore = HdKeystore::new("Insecure Password", meta);

        //        let coin = BchCoin::<Secp256k1Curve, BchAddress>::append_account(&mut keystore, PASSWORD, BIP_PATH);
        let bch_coin = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };
        let _ = keystore
            .derive_coin::<BchAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD)
            .unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();
        assert!(!address.is_empty());
        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BCH", coin);
    }

    #[test]
    fn bch_recover() {
        let mut meta = Metadata::default();
        meta.name = "RecoverTest".to_string();

        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let bch_coin = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };

        let _ = keystore
            .derive_coin::<BchAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD)
            .unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();

        assert_eq!(
            "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885",
            address
        );

        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BCH", coin);

        let extra = account["extra"].as_object().expect("extra");
        let xpub = extra["xpub"].as_str().expect("xpub");
        assert_eq!("xpub6Bmkv3mmRZZWoFSBdj9vDMqR2PCPSP6DEj8u3bBuv44g3Ncnro6cPVqZAw6wTEcxHQuodkuJG4EmAinqrrRXGsN3HHnRRMtAvzfYTiBATV1", xpub)
    }
}
