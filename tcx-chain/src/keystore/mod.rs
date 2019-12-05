use super::Result;
use std::time::{SystemTime, UNIX_EPOCH};

mod guard;
mod hd;
mod private;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tcx_constants::{CoinInfo, CurveType};

pub use self::{guard::KeystoreGuard, hd::HdKeystore, private::PrivateKeystore};
use tcx_crypto::{Crypto, Pbkdf2Params};
use tcx_primitive::{DeterministicPrivateKey, PrivateKey, TypedDeterministicPrivateKey, TypedPrivateKey, TypedPublicKey, TypedDeterministicPublicKey};
use crate::signer::ChainSigner;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Store {
    pub id: String,
    pub version: i64,
    pub key_hash: String,
    pub crypto: Crypto<Pbkdf2Params>,
    pub active_accounts: Vec<Account>,

    #[serde(rename = "imTokenMeta")]
    pub meta: Metadata,
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
    #[fail(display = "invalid_version")]
    InvalidVersion,
}

/// Account that presents one blockchain wallet on a keystore
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub derivation_path: String,
    pub curve: CurveType,
    pub coin: String,
    pub network: String,
    pub seg_wit: String,
    pub ext_pub_key: String,
}

/// Chain address interface, for encapsulate derivation
pub trait Address {
    // Incompatible between the trait `Address:PubKey is not implemented for `&<impl curve::PrivateKey as curve::PrivateKey>::PublicKey`
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String>;

    fn is_valid(address: &str) -> bool;
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

pub enum Keystore {
    PrivateKey(PrivateKeystore),
    Hd(HdKeystore),
}

impl Keystore {
    pub fn unlock_by_password(&mut self, password: &str) -> Result<()> {
        match self {
            Keystore::PrivateKey(ks) => ks.unlock_by_password(password),
            Keystore::Hd(ks) => ks.unlock_by_password(password),
        }
    }

    pub fn determinable(&self) -> bool {
        match self {
            Keystore::PrivateKey(_) => false,
            Keystore::Hd(ks) => true,
        }
    }

    pub fn lock(&mut self) {
        match self {
            Keystore::PrivateKey(ks) => ks.lock(),
            Keystore::Hd(ks) => ks.lock(),
        }
    }

    pub fn derive_coin<A: Address, E: Extra>(&mut self, coin_info: &CoinInfo) -> Result<&Account> {
        match self {
            Keystore::PrivateKey(ks) => ks.derive_coin::<A, E>(coin_info),
            Keystore::Hd(ks) => ks.derive_coin::<A, E>(coin_info),
        }
    }

    pub fn find_private_key(&mut self, symbol: &str, address: &str) -> Result<TypedPrivateKey> {
        match self {
            Keystore::PrivateKey(ks) => ks.find_private_key(address),
            Keystore::Hd(ks) => ks.find_private_key(symbol, address),
        }
    }

    pub fn find_private_key_by_path(&mut self, symbol: &str, address:&str, path: &str) -> Result<TypedPrivateKey> {
        match self {
            Keystore::Hd(ks) => ks.find_private_key_by_path(symbol, address, path),
            Keystore::PrivateKey(ks) => ks.find_private_key(address),
        }
    }

    /*
    pub fn find_public_key(&mut self, symbol: &str, address:&str) -> Result<TypedPublicKey> {
        match self {
            Keystore::Hd(ks) => ks.find_public_key(symbol, address),
            _ => Err(Error::CannotDeriveKey.into()),
        }
    }
    */

    pub fn find_deterministic_public_key(&mut self, symbol: &str, address: &str) -> Result<TypedDeterministicPublicKey> {
        match self {
            Keystore::Hd(ks) => ks.find_deterministic_public_key(symbol, address),
            _ => Err(Error::CannotDeriveKey.into()),
        }
    }

    pub fn account(&self, symbol: &str, address: &str) -> Option<&Account> {
        match self {
            Keystore::PrivateKey(ks) => ks.account(address),
            Keystore::Hd(ks) => ks.account(symbol, address),
        }
    }

    pub fn must_find_account(&self, symbol: &str, address: &str) -> Result<&Account> {
        match self {
            Keystore::PrivateKey(ks) => ks.account(address).ok_or(Error::AccountNotFound.into()),
            Keystore::Hd(ks) => ks.account(symbol, address).ok_or(Error::AccountNotFound.into()),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        match self {
            Keystore::PrivateKey(ks) => ks.verify_password(password),
            Keystore::Hd(ks) => ks.verify_password(password),
        }
    }

    pub fn from_json(json: &str) -> Result<Keystore> {
        let store: Store = serde_json::from_str(json)?;

        match store.version {
            HdKeystore::VERSION => Ok(Keystore::Hd(HdKeystore::from_store(store))),
            PrivateKeystore::VERSION => {
                Ok(Keystore::PrivateKey(PrivateKeystore::from_store(store)))
            }

            _ => Err(Error::InvalidVersion.into()),
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            Keystore::PrivateKey(ks) => serde_json::to_string(ks.store()).unwrap(),
            Keystore::Hd(ks) => serde_json::to_string(ks.store()).unwrap(),
        }
    }
}

impl ChainSigner for Keystore {
    fn sign_recoverable_hash(&mut self, data: &[u8], symbol: &str, address: &str, path: Option<&str>) -> Result<Vec<u8>> {
        let private_key = if path.is_some() {
            self.find_private_key_by_path(symbol, address, path.unwrap())?
        } else {
            self.find_private_key(symbol, address)?
        };

        private_key.sign_recoverable(data)
    }

    fn sign_hash(&mut self, data: &[u8], symbol: &str, address: &str, path: Option<&str>) -> Result<Vec<u8>> {
        let private_key = if path.is_some() {
            self.find_private_key_by_path(symbol, address, path.unwrap())?
        } else {
            self.find_private_key(symbol, address)?
        };

        private_key.sign_recoverable(data)
    }
}


#[cfg(test)]
mod tests {
    use crate::Keystore;
    use serde_json::Value;
    use std::str::FromStr;

    fn from_json() {
        let json = r#"
        {
    "id": "41923f0c-427b-4e5f-a55c-a6a30d2ee0a5",
    "version": 11000,
    "crypto": {
        "cipher": "aes-128-ctr",
        "cipherparams": {
            "iv": "9374c8d7b04f9a7649a80142a83873e6"
        },
        "ciphertext": "8ce8dcc303dc02c2de0d8bad566c44b152543b61f12961c1bdcab08a7d83424f19d5beaf7251e28ddf00ccf5e3f3358ecc3eb10b1761bf1cd3b108806f6ff34158102602c6cdd6adceb09eb2db8c3244",
        "kdf": "pbkdf2",
        "kdfparams": {
          "c": 65535,
          "dklen": 32,
          "prf": "hmac-sha256",
          "salt": "33c8f2d27fe994a1e7d51108c7811cdaa2b821cc6760ed760954b4b67a1bcd8c"
        },
        "mac": "6b86a18f4ba9f3f428e256e72a3d832dcf0cd1cb820ec61e413a64d83b012059"
    },
    "activeAccounts": [
        {
            "address": "bc1q32nssyaw5ph0skae5nja0asmw2y2a6qw8f0p38",
            "derivationPath": "m/84'/0'/0'/0/0",
            "curve": "SECP256k1",
            "coin": "BITCOIN",
            "extra": {}
        },
        {
            "address": "tokencorex66",
            "derivationPath": "m/84'/0'/0'/0/0",
            "curve": "SECP256k1",
            "coin": "EOS",
            "extra": [
                {
                "encPrivate": {
                    "encStr": "8657459f1ad4b7b8d2db4850b9072dab1da6d08cf248070068dc910df73c1dc5",
                    "nonce": "cb64438515ef2565b7d0d1a036297bbd"
                },
                "publicKey": "EOS8W4CoVEhTj6RHhazfw6wqtrHGk4kE4fYb2VzCexAk81SjPU1mL"
                }
            ]
        }
    ],
    "imTokenMeta": {
        "name": "Multi Chain Keystore",
        "passwordHint": "",
        "source": "MNEMONIC",
        "timestamp": 1519611221
    }
}
"#;
        let keystore: Keystore = Keystore::from_json(json).unwrap();

        assert_eq!(
            Value::from_str(&keystore.to_json()).unwrap(),
            Value::from_str(json).unwrap()
        );
    }
}
