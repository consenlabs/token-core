use bip39::Seed;

pub mod address;
pub mod bip143_with_forkid;
pub mod transaction;

use core::result;
use serde::{Deserialize, Serialize};
use std::iter::IntoIterator;
use std::str::FromStr;
use tcx_chain::keystore::Address;
use tcx_chain::keystore::Extra;

#[macro_use]
extern crate failure;

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

#[macro_use]
extern crate tcx_chain;

pub type Result<T> = result::Result<T, failure::Error>;

use tcx_crypto::aes::cbc::{decrypt_pkcs7, encrypt_pkcs7};
use tcx_primitive::derive::get_account_path;

pub use transaction::{BitcoinForkTransaction, BtcForkSegWitTransaction, BtcForkTransaction, Utxo};

pub use address::{BtcForkAddress, PubKeyScript};
use serde::export::PhantomData;
use serde_json::Value;
use tcx_constants::{CoinInfo, CurveType};
use tcx_primitive::Pair;
use tcx_primitive::{ArbitraryNetworkExtendedPubKey, Derive};
use tcx_primitive::{DerivePath, Secp256k1Pair};
pub use transaction::ScriptPubKeyComponent;

#[derive(Fail, Debug)]
pub enum Error {
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
        let xpub_key = account_pair.extended_pub_key()?;
        let xpub = BtcForkAddress::extended_public_key(&xpub_key, coin_info)?;
        ExtendedPubKeyExtra::from_xpub(&xpub, &coin_info.symbol)
    }

    fn from_private_key(_coin_info: &CoinInfo, _prv_key: &str) -> Result<Self> {
        unimplemented!()
    }
}

impl<T: Address> ExtendedPubKeyExtra<T> {
    pub fn _enc_xpub(xpub: &str, key: &str, iv: &str) -> Result<String> {
        let key_bytes = hex::decode(key)?;
        let iv_bytes = hex::decode(iv)?;
        let encrypted = encrypt_pkcs7(&xpub.as_bytes(), &key_bytes, &iv_bytes)?;
        Ok(base64::encode(&encrypted))
    }

    fn _calc_external_address(xpub: &str, idx: i64, coin: &str) -> Result<ExternalAddress> {
        let extended_pub_key = ArbitraryNetworkExtendedPubKey::from_str(&xpub)?;
        let child_path = format!("{}/{}", 0, idx as u32);
        let index_pub = extended_pub_key.derive(&child_path)?;
        let address = T::from_public_key(&index_pub.public_key().to_bytes(), Some(coin))?;
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
        let enc_xpub = Self::_enc_xpub(&xpub, &*key, &*iv)?;
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
mod tests {
    use crate::BtcForkExtra;
    use bip39::{Language, Mnemonic, Seed};
    use tcx_chain::keystore::Extra;
    use tcx_constants::{CoinInfo, CurveType};

    #[test]
    pub fn extra_test() {
        let coin_info = CoinInfo {
            symbol: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        };
        let extra = BtcForkExtra::from_xpub("tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R", "LITECOIN").unwrap();
        assert_eq!("GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==", extra.enc_xpub);
        assert_eq!(
            "LNp88kijfnFKGcp1aPdnMkpfMycw1v7KdQ",
            extra.external_address.address
        );
        let xpub = extra.xpub().unwrap();
        assert_eq!("tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R", xpub);

        let mnemonic = Mnemonic::from_phrase(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            Language::English,
        )
        .unwrap();
        let seed = Seed::new(&mnemonic, "");
        let extra = BtcForkExtra::new(&coin_info, &seed).unwrap();
        assert_eq!(extra.enc_xpub, "MwDMFXVWDEuWvBogeW1v/MOMFDnGnnflm2JAPvJaJZO4HXp8fCsWETA7u8MzOW3KaPksglpUHLN3xkDr2QWMEQq0TewFZoZ3KsjmLW0KGMRN7XQKqo/omkSEsPfalVnp9Zxm2lpxVmIacqvlernVSg==");
        assert_eq!(extra.xpub().unwrap(), "xpub6D3MqTwuLWB5veAfhDjPu1oHfS6L1imVbf22zQFWJW9EtnSmYYqiGMGkW1MCsT2HmkW872tefMY9deewW6DGd8zE7RcXVv8wKhZnbJeidjT");

        let next_receive_address = extra.calc_external_address(2, "LITECOIN").unwrap();
        assert_eq!(
            "LYdJgidYoP6kvDkyyczPcLn78vghCFfKpe",
            next_receive_address.address
        );
    }
}
