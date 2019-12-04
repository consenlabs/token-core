use super::Result;
use std::time::{SystemTime, UNIX_EPOCH};

mod guard;
mod hd;
mod private;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tcx_constants::{CoinInfo, CurveType};

pub use self::{guard::KeystoreGuard, hd::HdKeystore, private::PrivateKeystore};
use tcx_primitive::{DeterministicPrivateKey, PrivateKey, TypedPrivateKey, TypedPublicKey};

/// Account that presents one blockchain wallet on a keystore
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub derivation_path: String,
    pub curve: CurveType,
    pub coin: String,
    pub extra: Value,
}

/// Chain address interface, for encapsulate derivation
pub trait Address {
    // Incompatible between the trait `Address:PubKey is not implemented for `&<impl curve::PrivateKey as curve::PrivateKey>::PublicKey`
    fn from_public_key(bytes: &[u8], coin: Option<&str>) -> Result<String>;
}

/// Encoding more information to account data with variant chain, like xpub for UTXO account base chain.
pub trait Extra: Sized + serde::Serialize + Clone {
    fn new(coin_info: &CoinInfo, seed: &[u8]) -> Result<Self>;
    fn from_private_key(coin_info: &CoinInfo, prv_key: &str) -> Result<Self>;
}

/// Source to remember which format it comes from
///
/// NOTE: Identity related type is only for imToken App v2.x
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    Wif,
    Private,
    Keystore,
    Mnemonic,
    NewIdentity,
    RecoveredIdentity,
}

/// Metadata of keystore, for presenting wallet data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    pub password_hint: String,
    #[serde(default = "metadata_default_time")]
    pub timestamp: i64,
    #[serde(default = "metadata_default_source")]
    pub source: Source,
}

fn metadata_default_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("get timestamp");
    since_the_epoch.as_secs() as i64
}

fn metadata_default_source() -> Source {
    Source::Mnemonic
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            name: String::from("Unknown"),
            password_hint: String::new(),
            timestamp: metadata_default_time(),
            source: Source::Mnemonic,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyExtra {}

impl Extra for EmptyExtra {
    fn new(_coin_info: &CoinInfo, _seed: &[u8]) -> Result<Self> {
        Ok(EmptyExtra {})
    }
    fn from_private_key(_coin_info: &CoinInfo, _prv_key: &str) -> Result<Self> {
        Ok(EmptyExtra {})
    }
}

pub trait Keystore {
    fn unlock_by_password(&mut self, password: &str) -> Result<()>;
    fn lock(&mut self);
    fn find_private_key(&self, address: &str) -> Result<TypedPrivateKey>;
}

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_mnemonic")]
    InvalidMnemonic,
    #[fail(display = "invalid_key_type")]
    InvalidKeyType,
    #[fail(display = "invalid_secp256k1_public_key")]
    InvalidSecp256k1PublicKey,
    #[fail(display = "unsupported_curve")]
    UnsupportedCurve,
    #[fail(display = "account_not_found")]
    AccountNotFound,
    #[fail(display = "can_not_derive_pair_from_seed")]
    CanNotDerivePairFromSeed,
    #[fail(display = "can_not_derive_key")]
    CannotDeriveKey,
    #[fail(display = "keystore_locked")]
    KeystoreLocked,
}
