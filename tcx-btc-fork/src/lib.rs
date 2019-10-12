use bip39::Seed;

pub mod address;
pub mod bip143_with_forkid;
pub mod hard_wallet_keystore;
pub mod transaction;

use core::result;
use serde::{Deserialize, Serialize};
use std::iter::IntoIterator;
use std::str::FromStr;
//use tcx_chain::curve::CurveType;
use tcx_chain::keystore::Address;
use tcx_chain::keystore::{CoinInfo, Extra};

#[macro_use]
extern crate failure;

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

#[macro_use]
extern crate tcx_chain;

pub type Result<T> = result::Result<T, failure::Error>;

use crate::address::BtcForkAddress;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey};
use secp256k1::Secp256k1;
use serde_json::Value;
use tcx_chain::bips::get_account_path;
use tcx_crypto::aes::cbc::{decrypt_pkcs7, encrypt_pkcs7};

pub use transaction::{BitcoinForkTransaction, BtcForkSegWitTransaction, BtcForkTransaction, Utxo};

pub use address::PubKeyScript;
use serde::export::PhantomData;
use tcx_primitive::Derive;
use tcx_primitive::Pair;
use tcx_primitive::{CurveType, DerivePath, Secp256k1Pair};
pub use transaction::ScriptPubKeyComponent;

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
pub struct ExtendedPubKeyExtra<T: Address> {
    #[serde(rename = "encXPub")]
    pub enc_xpub: String,
    pub external_address: ExternalAddress,
    #[serde(skip)]
    _maker_t: PhantomData<T>,
}

pub type BtcForkExtra = ExtendedPubKeyExtra<BtcForkAddress>;

impl<T: Address> Extra for ExtendedPubKeyExtra<T>
where
    T: std::clone::Clone,
{
    fn new(coin_info: &CoinInfo, seed: &Seed) -> Result<Self> {
        ensure!(
            coin_info.curve == CurveType::SECP256k1,
            "BCH must be at secp256k1"
        );
        let account_path = get_account_path(&coin_info.derivation_path)?;
        let pair = Secp256k1Pair::from_seed_slice(seed.as_bytes())?;
        let derive_path = DerivePath::from_str(&account_path)?;
        let account_pair = pair.derive(derive_path.into_iter())?;
        let derivation_info = pair.extended_pub_key()?;
        //        let derivation_info = Secp256k1Curve::extended_pub_key(&account_path, &seed)?;
        let xpub = BtcForkAddress::extended_public_key(&derivation_info, coin_info)?;
        ExtendedPubKeyExtra::from_xpub(&xpub, &coin_info.symbol)
    }
}

impl<T: Address> ExtendedPubKeyExtra<T> {
    pub fn enc_xpub(xpub: &str, key: &str, iv: &str) -> Result<String> {
        let key_bytes = hex::decode(key)?;
        let iv_bytes = hex::decode(iv)?;
        let encrypted = encrypt_pkcs7(&xpub.as_bytes(), &key_bytes, &iv_bytes)?;
        Ok(base64::encode(&encrypted))
    }

    fn _calc_external_address(xpub: &str, idx: i64, coin: &str) -> Result<ExternalAddress> {
        let extended_pub_key = ExtendedPubKey::from_str(&xpub)?;
        let s = Secp256k1::new();
        let index_pub = extended_pub_key.derive_pub(
            &s,
            &vec![
                ChildNumber::from_normal_idx(0).unwrap(),
                ChildNumber::from_normal_idx(idx as u32).unwrap(),
            ],
        )?;
        let address = T::from_public_key(&index_pub.public_key.to_bytes(), Some(coin))?;
        Ok(ExternalAddress {
            address,
            addr_type: "EXTERNAL".to_string(),
            derived_path: format!("0/{}", idx).to_string(),
        })
    }

    pub fn calc_external_address(&self, idx: i64, coin: &str) -> Result<ExternalAddress> {
        let xpub = self.xpub()?;
        Self::_calc_external_address(&xpub, idx, coin)
    }

    pub fn from_xpub(xpub: &str, coin: &str) -> Result<Self> {
        let key = tcx_crypto::XPUB_COMMON_KEY_128.read().unwrap();
        let iv = tcx_crypto::XPUB_COMMON_IV.read().unwrap();
        let enc_xpub = Self::enc_xpub(&xpub, &*key, &*iv)?;
        let external_address = Self::_calc_external_address(&xpub, 1i64, coin)?;
        Ok(ExtendedPubKeyExtra::<T> {
            enc_xpub,
            external_address,
            _maker_t: PhantomData,
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

impl<T: Address> From<Value> for ExtendedPubKeyExtra<T> {
    fn from(v: Value) -> Self {
        serde_json::from_value::<Self>(v).unwrap()
    }
}

#[cfg(test)]
mod tests {}
