use super::Result;
use std::time::{SystemTime, UNIX_EPOCH};

mod guard;
mod hd;
mod private;

use serde::{Deserialize, Serialize};

use tcx_constants::{CoinInfo, CurveType};

pub use self::{
    guard::KeystoreGuard, hd::key_hash_from_mnemonic, hd::HdKeystore, private::PrivateKeystore,
};
use crate::signer::ChainSigner;
use tcx_crypto::{Crypto, Pbkdf2Params};
use tcx_primitive::{
    DeterministicPrivateKey, PrivateKey, TypedDeterministicPublicKey, TypedPrivateKey,
    TypedPublicKey,
};

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

pub enum Keystore {
    PrivateKey(PrivateKeystore),
    Hd(HdKeystore),
}

impl Keystore {
    pub fn from_private_key(private_key: &str, password: &str) -> Keystore {
        Keystore::PrivateKey(PrivateKeystore::from_private_key(
            private_key,
            password,
            Source::Wif,
        ))
    }

    pub fn id(&self) -> String {
        self.store().id.to_string()
    }

    pub fn set_id(&mut self, id: &str) {
        self.store_mut().id = id.to_string()
    }

    fn store(&self) -> &Store {
        match self {
            Keystore::PrivateKey(ks) => ks.store(),
            Keystore::Hd(ks) => ks.store(),
        }
    }

    fn store_mut(&mut self) -> &mut Store {
        match self {
            Keystore::PrivateKey(ks) => ks.store_mut(),
            Keystore::Hd(ks) => ks.store_mut(),
        }
    }

    pub fn meta(&self) -> Metadata {
        self.store().meta.clone()
    }

    pub fn key_hash(&self) -> String {
        self.store().key_hash.to_string()
    }

    pub fn unlock_by_password(&mut self, password: &str) -> Result<()> {
        match self {
            Keystore::PrivateKey(ks) => ks.unlock_by_password(password),
            Keystore::Hd(ks) => ks.unlock_by_password(password),
        }
    }

    pub fn determinable(&self) -> bool {
        match self {
            Keystore::PrivateKey(_) => false,
            Keystore::Hd(_) => true,
        }
    }

    pub fn export(&self) -> Result<String> {
        match self {
            Keystore::PrivateKey(pk_store) => pk_store.private_key(),
            Keystore::Hd(hd_store) => hd_store.mnemonic(),
        }
    }

    pub fn lock(&mut self) {
        match self {
            Keystore::PrivateKey(ks) => ks.lock(),
            Keystore::Hd(ks) => ks.lock(),
        }
    }

    pub fn derive_coin<A: Address>(&mut self, coin_info: &CoinInfo) -> Result<&Account> {
        match self {
            Keystore::PrivateKey(ks) => ks.derive_coin::<A>(coin_info),
            Keystore::Hd(ks) => ks.derive_coin::<A>(coin_info),
        }
    }

    pub fn find_private_key(&mut self, symbol: &str, address: &str) -> Result<TypedPrivateKey> {
        match self {
            Keystore::PrivateKey(ks) => ks.find_private_key(address),
            Keystore::Hd(ks) => ks.find_private_key(symbol, address),
        }
    }

    pub fn find_private_key_by_path(
        &mut self,
        symbol: &str,
        address: &str,
        path: &str,
    ) -> Result<TypedPrivateKey> {
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

    pub fn find_deterministic_public_key(
        &mut self,
        symbol: &str,
        address: &str,
    ) -> Result<TypedDeterministicPublicKey> {
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

    pub fn accounts(&self) -> &[Account] {
        match self {
            Keystore::PrivateKey(ks) => ks.store().active_accounts.as_slice(),
            Keystore::Hd(ks) => ks.store().active_accounts.as_slice(),
        }
    }

    pub fn must_find_account(&self, symbol: &str, address: &str) -> Result<&Account> {
        match self {
            Keystore::PrivateKey(ks) => ks.account(address).ok_or(Error::AccountNotFound.into()),
            Keystore::Hd(ks) => ks
                .account(symbol, address)
                .ok_or(Error::AccountNotFound.into()),
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
    fn sign_recoverable_hash(
        &mut self,
        data: &[u8],
        symbol: &str,
        address: &str,
        path: Option<&str>,
    ) -> Result<Vec<u8>> {
        let private_key = if path.is_some() {
            self.find_private_key_by_path(symbol, address, path.unwrap())?
        } else {
            self.find_private_key(symbol, address)?
        };

        private_key.sign_recoverable(data)
    }

    fn sign_hash(
        &mut self,
        data: &[u8],
        symbol: &str,
        address: &str,
        path: Option<&str>,
    ) -> Result<Vec<u8>> {
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
    use crate::keystore::Keystore::{Hd, PrivateKey};
    use crate::{HdKeystore, Keystore, Metadata, PrivateKeystore, Source};
    use serde_json::Value;
    use std::str::FromStr;

    use tcx_primitive::{Ss58Codec, ToHex};

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    const MNEMONIC: &str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    static KEYSTORE_JSON: &'static str = r#"
        {
    "id": "7719d1e3-3f67-439f-a18e-d9ae413e00e1",
    "version": 11000,
    "keyHash": "efbe00a55ddd4c5350e295a9533d28f93cac001bfdad8cf4275140461ea03e9e",
    "crypto": {
        "cipher": "aes-128-ctr",
        "cipherparams": {
            "iv": "6006bd4e828f2f93dca31e36590ca4c9"
        },
        "ciphertext": "b06b82b8cda0bc72761177b312dfd46318248ad8473b6c97d46c44aedf6a283f44f0267dd03f210dcddf4ea1a34f85b0b02533dd9c37ce2276cb087af3e43f2a76b968e17c816ca8ea5c",
        "kdf": "pbkdf2",
        "kdfparams": {
            "c": 10240,
            "prf": "hmac-sha256",
            "dklen": 32,
            "salt": "5d85aaf812a613f810cc1cda18d35f46c013f5e537629e25372969f5f87402cd"
        },
        "mac": "56af7c5faf0a791cbb4911c4c20070156e4ad0a03f8253b2a2fb005a68d7a026"
    },
    "activeAccounts": [
        {
            "address": "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
            "derivationPath": "m/44'/145'/0'/0/0",
            "curve": "SECP256k1",
            "coin": "BITCOINCASH",
            "network": "MAINNET",
            "segWit": "NONE",
            "extPubKey": "031064f6a580000000251d72997d4cf931a7e6819f7da37725166100fc7dae9ca6afc3f8fd8a3d3a7f0303f2f84851514bf2f40a46b5bb9dbf4e5913fbacde1a96968cda08f9fd882caa"
        }
    ],
    "imTokenMeta": {
        "name": "test-wallet",
        "passwordHint": "imtoken",
        "timestamp": 1575605134,
        "source": "MNEMONIC"
    }
}
"#;

    #[test]
    fn test_json() {
        let keystore: Keystore = Keystore::from_json(KEYSTORE_JSON).unwrap();
        assert_eq!(1, keystore.accounts().len());
        assert_eq!(
            "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
            keystore.accounts().first().unwrap().address
        );

        assert_eq!(
            Value::from_str(&keystore.to_json()).unwrap(),
            Value::from_str(KEYSTORE_JSON).unwrap()
        );
    }

    #[test]
    fn test_keystore_non_sensitive() {
        let mut keystore = Keystore::from_json(KEYSTORE_JSON).unwrap();
        assert_eq!(keystore.id(), "7719d1e3-3f67-439f-a18e-d9ae413e00e1");
        keystore.set_id("test_set_id");
        assert_eq!("test_set_id", keystore.id());
        assert_eq!("test-wallet", keystore.meta().name);
        assert!(keystore.determinable());
        assert_eq!(
            "efbe00a55ddd4c5350e295a9533d28f93cac001bfdad8cf4275140461ea03e9e",
            keystore.key_hash()
        );
    }

    #[test]
    fn test_keystore_unlock() {
        let mut keystore = Keystore::from_json(KEYSTORE_JSON).unwrap();

        let export_ret = keystore.export();
        assert!(export_ret.is_err());
        assert_eq!(format!("{}", export_ret.err().unwrap()), "keystore_locked");
        let unlocked_ret = keystore.unlock_by_password("WRONG PASSWORD");
        assert!(unlocked_ret.is_err());
        assert_eq!(
            format!("{}", unlocked_ret.err().unwrap()),
            "password_incorrect"
        );

        assert!(keystore.verify_password(PASSWORD));
        assert!(!keystore.verify_password("WRONG PASSWORD"));
        keystore.unlock_by_password(PASSWORD).unwrap();
        assert_eq!(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            keystore.export().unwrap()
        );

        keystore.lock();
        let export_ret = keystore.export();
        assert!(export_ret.is_err());
        assert_eq!(format!("{}", export_ret.err().unwrap()), "keystore_locked");
    }

    #[test]
    fn test_find_key() {
        let mut keystore = Keystore::from_json(KEYSTORE_JSON).unwrap();
        keystore.unlock_by_password(PASSWORD);
        let pk =
            keystore.find_private_key("BITCOINCASH", "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y21");
        assert!(pk.is_err());

        let pk =
            keystore.find_private_key("BITCOINCASH", "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r");
        assert_eq!(
            pk.unwrap()
                .as_secp256k1()
                .unwrap()
                .to_ss58check_with_version(&[0x80]),
            "L39VXyorp19JfsEJfbD7Tfr4pBEX93RJuVXW7E13C51ZYAhUWbYa"
        );

        let pk = keystore.find_private_key_by_path(
            "BITCOINCASH",
            "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
            "0/0",
        );
        assert_eq!(
            pk.unwrap()
                .as_secp256k1()
                .unwrap()
                .to_ss58check_with_version(&[0x80]),
            "L39VXyorp19JfsEJfbD7Tfr4pBEX93RJuVXW7E13C51ZYAhUWbYa"
        );

        let public_key = keystore
            .find_deterministic_public_key(
                "BITCOINCASH",
                "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
            )
            .unwrap();
        assert_eq!(public_key.to_hex(), "031064f6a580000000251d72997d4cf931a7e6819f7da37725166100fc7dae9ca6afc3f8fd8a3d3a7f0303f2f84851514bf2f40a46b5bb9dbf4e5913fbacde1a96968cda08f9fd882caa");
    }

    #[test]
    fn test_create() {
        let hd_store = HdKeystore::new(PASSWORD, Metadata::default());
        let keystore = Hd(hd_store);
        assert_eq!(0, keystore.accounts().len());
        assert!(keystore.determinable());

        let hd_store = HdKeystore::from_mnemonic(MNEMONIC, PASSWORD, Metadata::default());
        let keystore = Hd(hd_store);
        assert_eq!(0, keystore.accounts().len());
        assert!(keystore.determinable());
        assert_eq!(
            keystore.key_hash(),
            "512115eca3ae86646aeb06861d551e403b54350968ad9a247aadd5e7dace9d33"
        );

        let pk_store = PrivateKeystore::from_private_key(
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            PASSWORD,
            Source::Private,
        );
        let keystore = PrivateKey(pk_store);
        assert_eq!(0, keystore.accounts().len());
        assert!(!keystore.determinable());
    }
}
