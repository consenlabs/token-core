use super::Result;
use std::time::{SystemTime, UNIX_EPOCH};

mod guard;
mod hd;
mod private;

use serde::{Deserialize, Serialize};

use tcx_constants::{CoinInfo, CurveType};

pub use self::{
    guard::KeystoreGuard, hd::key_hash_from_mnemonic, hd::HdKeystore,
    private::key_hash_from_private_key, private::PrivateKeystore,
};

use crate::signer::ChainSigner;
use tcx_crypto::{Crypto, Pbkdf2Params};
use tcx_primitive::{TypedDeterministicPublicKey, TypedPrivateKey, TypedPublicKey};

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
    #[fail(display = "mnemonic_invalid")]
    MnemonicInvalid,
    #[fail(display = "mnemonic_word_invalid")]
    MnemonicWordInvalid,
    #[fail(display = "mnemonic_length_invalid")]
    MnemonicLengthInvalid,
    #[fail(display = "mnemonic_checksum_invalid")]
    MnemonicChecksumInvalid,
    #[fail(display = "account_not_found")]
    AccountNotFound,
    #[fail(display = "can_not_derive_key")]
    CannotDeriveKey,
    #[fail(display = "keystore_locked")]
    KeystoreLocked,
    #[fail(display = "invalid_version")]
    InvalidVersion,
}

fn transform_mnemonic_error(err: failure::Error) -> Error {
    let err = err.downcast::<bip39::ErrorKind>().unwrap();
    match err {
        bip39::ErrorKind::InvalidChecksum => Error::MnemonicChecksumInvalid,
        bip39::ErrorKind::InvalidWord => Error::MnemonicWordInvalid,
        bip39::ErrorKind::InvalidWordLength(_) => Error::MnemonicLengthInvalid,
        _ => Error::MnemonicInvalid,
    }
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

    fn is_valid(address: &str, coin: &CoinInfo) -> bool;
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

    pub fn from_mnemonic(mnemonic: &str, password: &str, metadata: Metadata) -> Result<Keystore> {
        Ok(Keystore::Hd(HdKeystore::from_mnemonic(
            mnemonic, password, metadata,
        )?))
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

    pub fn is_locked(&self) -> bool {
        match self {
            Keystore::PrivateKey(ks) => ks.is_locked(),
            Keystore::Hd(ks) => ks.is_locked(),
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

    pub fn derive_coin<A: Address>(&mut self, coin_info: &CoinInfo) -> Result<Account> {
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
            Keystore::PrivateKey(ks) => ks.account(symbol, address),
            Keystore::Hd(ks) => ks.account(symbol, address),
        }
    }

    pub fn accounts(&self) -> &[Account] {
        match self {
            Keystore::PrivateKey(ks) => ks.store().active_accounts.as_slice(),
            Keystore::Hd(ks) => ks.store().active_accounts.as_slice(),
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

        private_key.sign(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::keystore::Keystore::{Hd, PrivateKey};
    use crate::{ChainSigner, HdKeystore, Keystore, Metadata, PrivateKeystore, Source};
    use serde_json::Value;
    use std::str::FromStr;

    use crate::keystore::metadata_default_source;
    use tcx_constants::{TEST_MNEMONIC, TEST_PASSWORD};
    use tcx_primitive::{Ss58Codec, ToHex};

    static HD_KEYSTORE_JSON: &'static str = r#"
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

    static PK_KEYSTORE_JSON: &'static str = r#"
    {"id":"89e6fc5d-ac9a-46ab-b53f-342a80f3d28b","version":11001,"keyHash":"4fc213ddcb6fa44a2e2f4c83d67502f88464e6ee","crypto":{"cipher":"aes-128-ctr","cipherparams":{"iv":"c0ecc72839f8a02cc37eb7b0dd0b93ba"},"ciphertext":"1239e5807e19f95d86567f81c162c69a5f4564ea17f487669a277334f4dcc7dc","kdf":"pbkdf2","kdfparams":{"c":1024,"prf":"hmac-sha256","dklen":32,"salt":"3c9df9eb95a014c77bbc8b9a06f4f14e0d08170dea71189c7cf377a3b2099404"},"mac":"909a6bfe1ad031901e80927b847a8fa8407fdcde56cfa374f7a732fb3b3a882d"},"activeAccounts":[{"address":"TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG","derivationPath":"","curve":"SECP256k1","coin":"TRON","network":"","segWit":"","extPubKey":""}],"imTokenMeta":{"name":"Unknown","passwordHint":"","timestamp":1576733295,"source":"PRIVATE"}}
    "#;

    static OLD_KEYSTORE_JSON: &'static str = r#"
    {
  "crypto": {
    "cipher": "aes-128-ctr",
    "cipherparams": {
      "iv": "437ef8c8553df9910ad117ecec5b8c05"
    },
    "ciphertext": "acabec2bd6fab27d867ebabe0ded9c64c85aebd294d29ecf537e563474ebb931522dbb977e0644830516550255edde02c507863cb083b55f2f0f759c2f8a885a81a6518237e7b65b7cf3e912fb36e42a13a7b2df3d401e5ff778a412a6d4c5516645770c4b12f2e30551542c699eef",
    "kdf": "pbkdf2",
    "kdfparams": {
      "c": 65535,
      "dklen": 32,
      "prf": "hmac-sha256",
      "salt": "33c8f2d27fe994a1e7d51108c7811cdaa2b821cc6760ed760954b4b67a1bcd8c"
    },
    "mac": "6b86a18f4ba9f3f428e256e72a3d832dcf0cd1cb820ec61e413a64d83b012059"
  },
  "id": "02a55ab6-554a-4e78-bc26-6a7acced7e5e",
  "version": 44,
  "address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
  "encMnemonic": {
    "encStr": "840fad94f4bf4128f629bc1dec731d156283cc4099e3c7659a3bf382031443fcdce6debaaef444393c446d2b4007064c010f6a442b3ad0ff0851c1bd638ba251afa92d3106457bd78c49",
    "nonce": "4d691a7f0cb6396e96e8dc3e4f35dccd"
  },
  "info": {
    "curve": "spec256k1",
    "purpuse": "sign"
  },
  "mnemonicPath": "m/44'/1'/0'",
  "xpub": "tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R",
  "imTokenMeta": {
    "backup": [],
    "chainType": "BITCOIN",
    "network": "TESTNET",
    "name": "BTC",
    "passwordHint": "",
    "source": "RECOVERED_IDENTITY",
    "walletType": "HD",
    "timestamp": 1519611221,
    "segWit": "NONE"
  }
}
    "#;

    #[test]
    fn test_json() {
        let keystore: Keystore = Keystore::from_json(HD_KEYSTORE_JSON).unwrap();
        assert_eq!(1, keystore.accounts().len());
        assert_eq!(
            "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
            keystore.accounts().first().unwrap().address
        );

        assert_eq!(
            Value::from_str(&keystore.to_json()).unwrap(),
            Value::from_str(HD_KEYSTORE_JSON).unwrap()
        );

        let keystore: Keystore = Keystore::from_json(PK_KEYSTORE_JSON).unwrap();
        assert_eq!(1, keystore.accounts().len());
        assert_eq!(
            "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG",
            keystore.accounts().first().unwrap().address
        );

        assert_eq!(
            Value::from_str(&keystore.to_json()).unwrap(),
            Value::from_str(PK_KEYSTORE_JSON).unwrap()
        );

        let ret = Keystore::from_json(OLD_KEYSTORE_JSON);
        assert!(ret.is_err());
    }

    #[test]
    fn test_sign_hash() {
        let msg = hex::decode("645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76")
            .unwrap();

        let mut keystore: Keystore = Keystore::from_json(HD_KEYSTORE_JSON).unwrap();
        let ret = keystore.sign_hash(
            &msg,
            "BITCOINCASH",
            "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
            Some("0/2"),
        );
        assert!(ret.is_err());
        assert_eq!(format!("{}", ret.err().unwrap()), "keystore_locked");
        let _ = keystore.unlock_by_password(TEST_PASSWORD);
        let ret = keystore
            .sign_hash(
                &msg,
                "BITCOINCASH",
                "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
                Some("0/2"),
            )
            .unwrap();
        assert_eq!("3045022100a5c14ac7fd46f9f0c951b86d9586595270266ab09b49bf79fc27ebae7866256002206a7d7841fb740ee190c94dcd156228fc820f5ff5ba8c07748b220d07c51d247a", hex::encode(ret));

        let mut keystore: Keystore = Keystore::from_json(PK_KEYSTORE_JSON).unwrap();
        let ret = keystore.sign_hash(&msg, "TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG", None);
        assert!(ret.is_err());
        assert_eq!(format!("{}", ret.err().unwrap()), "keystore_locked");
        keystore.unlock_by_password("imtoken1");
        let msg = hex::decode("645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76")
            .unwrap();
        let ret = keystore
            .sign_hash(&msg, "TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG", None)
            .unwrap();
        assert_eq!("30450221008d4920cb3a5a46a3f76845e823c9531f4a882eac4ffd61bfeaa29646999a83d302205c4c5537816911a8b0eb5f0e7ea09839c37e9e22bace8404d23d064c84d403d5", hex::encode(ret));
    }

    #[test]
    fn test_keystore_non_sensitive() {
        let mut keystore = Keystore::from_json(HD_KEYSTORE_JSON).unwrap();
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
        let mut keystore = Keystore::from_json(HD_KEYSTORE_JSON).unwrap();

        let export_ret = keystore.export();
        assert!(export_ret.is_err());
        assert_eq!(format!("{}", export_ret.err().unwrap()), "keystore_locked");
        let unlocked_ret = keystore.unlock_by_password("WRONG PASSWORD");
        assert!(unlocked_ret.is_err());
        assert_eq!(
            format!("{}", unlocked_ret.err().unwrap()),
            "password_incorrect"
        );

        assert!(keystore.verify_password(TEST_PASSWORD));
        assert!(!keystore.verify_password("WRONG PASSWORD"));
        keystore.unlock_by_password(TEST_PASSWORD).unwrap();
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
    fn test_hd_find_key() {
        let mut keystore = Keystore::from_json(HD_KEYSTORE_JSON).unwrap();
        keystore.unlock_by_password(TEST_PASSWORD).unwrap();
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

        let acc = keystore.account("BITCOINCASH", "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2");
        assert!(acc.is_none());

        let acc = keystore.account("BITCOINCASH", "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r");
        assert!(acc.is_some());
    }

    #[test]
    fn test_pk_find_key() {
        let mut keystore = Keystore::from_json(PK_KEYSTORE_JSON).unwrap();
        keystore.unlock_by_password("imtoken1").unwrap();
        let pk =
            keystore.find_private_key("BITCOINCASH", "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y21");
        assert!(pk.is_err());

        let pk = keystore
            .find_private_key("TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG")
            .unwrap();
        assert_eq!(
            hex::encode(pk.to_bytes()),
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6"
        );

        let pk = keystore
            .find_private_key_by_path("TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG", "")
            .unwrap();
        assert_eq!(
            hex::encode(pk.to_bytes()),
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6"
        );

        let acc = keystore.account("TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLsh");
        assert!(acc.is_none());

        let acc = keystore.account("TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG");
        assert!(acc.is_some());

        let ret =
            keystore.find_deterministic_public_key("TRON", "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG");
        assert!(ret.is_err())
    }

    #[test]
    fn test_create() {
        let hd_store = HdKeystore::new(TEST_PASSWORD, Metadata::default());
        let keystore = Hd(hd_store);
        assert_eq!(0, keystore.accounts().len());
        assert!(keystore.determinable());

        let hd_store =
            HdKeystore::from_mnemonic(TEST_MNEMONIC, TEST_PASSWORD, Metadata::default()).unwrap();
        let keystore = Hd(hd_store);
        assert_eq!(0, keystore.accounts().len());
        assert!(keystore.determinable());
        assert_eq!(
            keystore.key_hash(),
            "512115eca3ae86646aeb06861d551e403b543509"
        );

        let pk_store = PrivateKeystore::from_private_key(
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            TEST_PASSWORD,
            Source::Private,
        );
        let keystore = PrivateKey(pk_store);
        assert_eq!(0, keystore.accounts().len());
        assert!(!keystore.determinable());

        let ret = HdKeystore::from_mnemonic(
            format!("{} hello", TEST_MNEMONIC).as_str(),
            TEST_PASSWORD,
            Metadata::default(),
        );
        assert!(ret.is_err())
    }

    #[test]
    fn test_default_source() {
        assert_eq!(Source::Mnemonic, metadata_default_source());
    }
}
