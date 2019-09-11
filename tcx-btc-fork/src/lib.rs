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

#[macro_use]
extern crate tcx_chain;
#[macro_use]
extern crate serde;

pub type Result<T> = result::Result<T, failure::Error>;

use crate::address::BtcForkAddress;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey};
use secp256k1::Secp256k1;
use serde_json::Value;
use tcx_chain::bips::get_account_path;
use tcx_crypto::aes::cbc::{decrypt_pkcs7, encrypt_pkcs7};
use tcx_crypto::aes::ctr::encrypt_nopadding;
pub use transaction::{
    BchTransaction, BitcoinForkTransaction, BtcForkSegWitTransaction, BtcForkTransaction, Utxo,
};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "bch_convert_to_legacy_address_failed# address: {}", _0)]
    ConvertToLegacyAddressFailed(String),
    #[fail(display = "bch_convert_to_cash_address_failed# address: {}", _0)]
    ConvertToCashAddressFailed(String),
    #[fail(display = "construct_bch_address_failed# address: {}", _0)]
    ConstructBchAddressFailed(String),
    #[fail(display = "decrypt_xpub_error")]
    DecryptXPubError,
    #[fail(display = "unsupported_chain")]
    UnsupportedChain,
}

const SYMBOL: &'static str = "BCH";
const PATH: &'static str = "m/44'/145'/0'/0/0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    #[serde(rename = "encXPub")]
    pub enc_xpub: String,
    pub external_address: ExternalAddress,
}

impl Extra for ExtendedPubKeyExtra {
    fn new(coin_info: &CoinInfo, seed: &Seed) -> Result<Self> {
        ensure!(
            coin_info.curve == CurveType::SECP256k1,
            "BCH must be at secp256k1"
        );
        let account_path = get_account_path(&coin_info.derivation_path)?;
        let derivation_info = Secp256k1Curve::extended_pub_key(&account_path, &seed)?;
        let xpub = BtcForkAddress::extended_public_key(&derivation_info, coin_info)?;
        ExtendedPubKeyExtra::from_xpub(&xpub, &coin_info.symbol)
    }
}

impl ExtendedPubKeyExtra {
    pub fn enc_xpub(xpub: &str, key: &str, iv: &str) -> Result<String> {
        let key_bytes = hex::decode(key)?;
        let iv_bytes = hex::decode(iv)?;
        let encrypted = encrypt_pkcs7(&xpub.as_bytes(), &key_bytes, &iv_bytes)?;
        Ok(base64::encode(&encrypted))
    }

    fn _calc_external_address<A: Address>(
        xpub: &str,
        idx: i64,
        coin: &str,
    ) -> Result<ExternalAddress> {
        let extended_pub_key = ExtendedPubKey::from_str(&xpub)?;
        let s = Secp256k1::new();
        let index_pub = extended_pub_key.derive_pub(
            &s,
            &vec![
                ChildNumber::from_normal_idx(0).unwrap(),
                ChildNumber::from_normal_idx(idx as u32).unwrap(),
            ],
        )?;
        let address = A::from_public_key(&index_pub.public_key, Some(coin))?;
        Ok(ExternalAddress {
            address,
            addr_type: "EXTERNAL".to_string(),
            derived_path: format!("0/{}", idx).to_string(),
        })
    }

    pub fn calc_external_address<A: Address>(
        &self,
        idx: i64,
        coin: &str,
    ) -> Result<ExternalAddress> {
        let xpub = self.xpub()?;
        Self::_calc_external_address::<A>(&xpub, idx, coin)
    }

    pub fn from_xpub(xpub: &str, coin: &str) -> Result<Self> {
        let key = tcx_crypto::XPUB_COMMON_KEY_128.read().unwrap();
        let iv = tcx_crypto::XPUB_COMMON_IV.read().unwrap();
        let enc_xpub = ExtendedPubKeyExtra::enc_xpub(&xpub, &*key, &*iv)?;
        let external_address =
            ExtendedPubKeyExtra::_calc_external_address::<BtcForkAddress>(&xpub, 1i64, coin)?;
        Ok(ExtendedPubKeyExtra {
            enc_xpub,
            external_address,
        })
    }

    pub fn xpub(&self) -> Result<String> {
        let key = tcx_crypto::XPUB_COMMON_KEY_128.read().unwrap();
        let iv = tcx_crypto::XPUB_COMMON_IV.read().unwrap();
        let xpub_bytes = base64::decode(&self.enc_xpub)?;
        let key_bytes = hex::decode(&*key)?;
        let iv_bytes = hex::decode(&*iv)?;
        let decrypted = decrypt_pkcs7(&xpub_bytes, &key_bytes, &iv_bytes)?;
        String::from_utf8(decrypted).map_err(|_| Error::DecryptXPubError.into())
    }
}

impl From<Value> for ExtendedPubKeyExtra {
    fn from(v: Value) -> Self {
        serde_json::from_value::<Self>(v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::address::BtcForkAddress;
    use crate::{ExtendedPubKeyExtra, ExternalAddress};
    use bch_addr::Converter;
    use bitcoin::util::misc::hex_bytes;
    use serde_json::Value;
    use std::str::FromStr;
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
            .derive_coin::<BtcForkAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD)
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
            .derive_coin::<BtcForkAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD)
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
        let enc_xpub = extra["encXPub"].as_str().expect("enc_xpub");
        assert_eq!(enc_xpub, "wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==")
    }

    #[test]
    fn extra_test() {
        let ex = ExtendedPubKeyExtra::from_xpub("tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R", "bch").unwrap();

        assert_eq!(ex.enc_xpub, "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==");

        //        let addr = ex.calc_external_address::<BchAddress>(1i64).unwrap();
        let expected = r#"
        {
            "address": "bitcoincash:qqn4as4zx0jmy02rlgv700umavxt8xtpzu5gcetg92",
            "type": "EXTERNAL",
            "derivedPath": "0/1"
        }
        "#;

        assert_eq!(
            serde_json::to_value(ex.external_address).unwrap(),
            serde_json::Value::from_str(expected).unwrap()
        );
    }

    #[test]
    fn address_converter() {
        let converter = Converter::new();
        let ret = converter
            .to_legacy_addr("bitcoincash:qq2ug6v04ht22n0daxxzl0rzlvsmzwcdwuymj77ymy")
            .unwrap();
        assert_eq!(ret, "12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g")
    }
}
