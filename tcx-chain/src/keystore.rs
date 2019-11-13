use bip39::{Language, Mnemonic, Seed};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use tcx_crypto::{Crypto, Pbkdf2Params};
use tcx_primitive::{derive, Derive, DerivePath, Pair, Public, Secp256k1Pair};

use crate::Error;
use crate::Result;
use core::{fmt, result};
use serde_json::{Map, Value};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tcx_constants::{CoinInfo, CurveType};

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

/// Chain address interface, for encapsulate derivation
pub trait Address {
    // Incompatible between the trait `Address:PubKey is not implemented for `&<impl curve::PrivateKey as curve::PrivateKey>::PublicKey`
    fn from_public_key(public_key: &[u8], coin: Option<&str>) -> Result<String>;

    fn from_private_key(private_key: &str, coin: Option<&str>) -> Result<String>;
}

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

/// Encoding more information to account data with variant chain, like xpub for UTXO account base chain.
pub trait Extra: Sized + serde::Serialize + Clone {
    fn new(coin_info: &CoinInfo, seed: &Seed) -> Result<Self>;
    fn from_private_key(coin_info: &CoinInfo, prv_key: &str) -> Result<Self>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyExtra {}

impl Extra for EmptyExtra {
    fn new(_coin_info: &CoinInfo, _seed: &Seed) -> Result<Self> {
        Ok(EmptyExtra {})
    }
    fn from_private_key(_coin_info: &CoinInfo, _prv_key: &str) -> Result<Self> {
        Ok(EmptyExtra {})
    }
}

/// Keystore type
///
/// NOTE: mnemonic for HD wallet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyType {
    PrivateKey,
    Mnemonic,
}

/// Primary keystore type to store a root seed for deriving multi chain accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdKeystore {
    pub id: String,
    pub version: i32,
    pub key_type: KeyType,
    pub crypto: Crypto<Pbkdf2Params>,
    pub active_accounts: Vec<Account>,
    #[serde(rename = "imTokenMeta")]
    pub meta: Metadata,
}

impl HdKeystore {
    pub const VERSION: i32 = 11000i32;

    /// Derive account from a mnemonic phase
    pub fn mnemonic_to_account<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        mnemonic: &str,
    ) -> Result<Account> {
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)
            .map_err(|_| Error::InvalidMnemonic)?;
        let seed = bip39::Seed::new(&mnemonic, &"");
        Self::derive_account_from_coin::<A, E>(coin_info, &seed)
    }

    fn derive_account_from_coin<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        seed: &Seed,
    ) -> Result<Account> {
        let paths = vec![coin_info.derivation_path.clone()];
        let keys = Self::key_pair_at_paths_with_seed(coin_info.curve, &paths, &seed)?;
        let key = keys.first().ok_or_else(|| format_err!("derivate_failed"))?;
        let pub_key = key.public_key();
        let bytes = pub_key.to_bytes()?;
        let address = A::from_public_key(&bytes, Some(&coin_info.symbol))?;

        let extra = E::new(coin_info, seed)?;
        let acc = Account {
            address,
            derivation_path: coin_info.derivation_path.to_string(),
            curve: coin_info.curve,
            coin: coin_info.symbol.to_string(),
            extra: serde_json::to_value(extra.clone()).expect("extra_error"),
        };
        Ok(acc)
    }

    pub fn new(password: &str, meta: Metadata) -> HdKeystore {
        let mnemonic = derive::generate_mnemonic();
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 11000,
            key_type: KeyType::Mnemonic,
            crypto,
            active_accounts: vec![],
            meta,
        }
    }

    pub fn from_mnemonic(mnemonic: &str, password: &str, meta: Metadata) -> HdKeystore {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 11000,
            key_type: KeyType::Mnemonic,
            crypto,
            active_accounts: vec![],
            meta,
        }
    }

    pub fn from_private_key(private_key: &str, password: &str, source: Source) -> HdKeystore {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, private_key.as_bytes());
        let mut meta = Metadata::default();
        meta.source = source;
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 11000,
            key_type: KeyType::PrivateKey,
            crypto,
            active_accounts: vec![],
            meta,
        }
    }
    pub fn private_key_to_account<A: Address, E: Extra>(
        coin: &CoinInfo,
        private_key: &str,
    ) -> Result<Account> {
        let extra = E::from_private_key(coin, private_key)?;
        let addr = A::from_private_key(private_key, Some(&coin.symbol))?;
        let acc = Account {
            address: addr,
            derivation_path: "".to_string(),
            curve: coin.curve,
            coin: coin.symbol.to_owned(),
            extra: serde_json::to_value(extra.clone()).expect("extra_error"),
        };
        Ok(acc)
    }

    pub fn mnemonic(&self, password: &str) -> Result<String> {
        tcx_ensure!(self.key_type == KeyType::Mnemonic, Error::InvalidKeyType);
        let mnemonic_bytes = self.crypto.decrypt(password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes)?;
        Ok(mnemonic)
    }

    pub fn private_key(&self, password: &str) -> Result<String> {
        tcx_ensure!(self.key_type == KeyType::PrivateKey, Error::InvalidKeyType);
        let bytes = self.crypto.decrypt(password)?;
        let priv_key = String::from_utf8(bytes)?;
        Ok(priv_key)
    }

    pub fn seed(&self, password: &str) -> Result<Seed> {
        let mnemonic_str = self.mnemonic(password)?;
        let mnemonic = Mnemonic::from_phrase(mnemonic_str, Language::English)
            .map_err(|_| Error::InvalidMnemonic)?;
        Ok(bip39::Seed::new(&mnemonic, &""))
    }

    fn key_pair_at_paths_with_seed(
        curve: CurveType,
        paths: &[impl AsRef<str>],
        seed: &Seed,
    ) -> Result<Vec<impl Pair>> {
        match curve {
            CurveType::SECP256k1 => {
                let pair = Secp256k1Pair::from_seed_slice(seed.as_bytes())
                    .map_err(|_| Error::CanNotDerivePairFromSeed)?;
                let pairs: Result<Vec<Secp256k1Pair>> = paths
                    .iter()
                    .map(|path| {
                        let path = DerivePath::from_str(path.as_ref())
                            .map_err(|_| Error::CannotDeriveKey)?;
                        let child_pair = pair
                            .derive(path.into_iter())
                            .map_err(|_| Error::CannotDeriveKey)?;
                        Ok(child_pair)
                    })
                    .collect();
                pairs
            }

            _ => Err(Error::UnsupportedCurve.into()),
        }
    }

    pub fn get_pair<T: Pair>(&self, path: &str, password: &str) -> Result<T> {
        let seed = self.seed(password)?;

        match T::from_seed_slice(seed.as_bytes()) {
            Ok(r) => {
                if let Ok(p) = DerivePath::from_str(path) {
                    if let Ok(ret) = r.derive(p.into_iter()) {
                        return Ok(ret);
                    }
                }

                Err(Error::CanNotDerivePairFromSeed.into())
            }
            _ => Err(Error::CanNotDerivePairFromSeed.into()),
        }
    }

    /// Derive a private key at a specific path, it's coin independent
    pub fn key_pair_at_paths(
        &self,
        symbol: &str,
        paths: &[impl AsRef<str>],
        password: &str,
    ) -> Result<Vec<impl Pair>> {
        let acc = self.account(symbol).ok_or(Error::AccountNotFound)?;
        let seed = self.seed(password)?;
        Ok(Self::key_pair_at_paths_with_seed(acc.curve, paths, &seed)?)
    }

    /// Derive an account on a specific coin
    pub fn derive_coin<A: Address, E: Extra>(
        &mut self,
        coin_info: &CoinInfo,
        password: &str,
    ) -> Result<&Account> {
        let account = if self.meta.source != Source::Wif && self.meta.source != Source::Private {
            let seed = self.seed(password)?;
            Self::derive_account_from_coin::<A, E>(coin_info, &seed)?
        } else {
            let priv_key = self.private_key(password)?;
            Self::private_key_to_account::<A, E>(coin_info, &priv_key)?
        };
        self.active_accounts.push(account);
        Ok(self.active_accounts.last().unwrap())
    }

    /// Find an account by coin symbol
    pub fn account(&self, symbol: &str) -> Option<&Account> {
        self.active_accounts.iter().find(|acc| acc.coin == symbol)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        self.crypto.verify_password(password)
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Load a json to create HD keystore instance
    pub fn load(json: &str) -> Result<HdKeystore> {
        let ret: HdKeystore = serde_json::from_str(json)?;
        Ok(ret)
    }
}

fn merge_value(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_value(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

impl Display for HdKeystore {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        let mut pw = Map::new();
        pw.insert("id".to_string(), json!(&self.id.to_string()));
        pw.insert("name".to_string(), json!(&self.meta.name));
        pw.insert("passwordHint".to_string(), json!(&self.meta.password_hint));
        pw.insert("createdAt".to_string(), json!(&self.meta.timestamp));
        pw.insert("source".to_string(), json!(&self.meta.source));

        if !&self.active_accounts.is_empty() {
            if self.active_accounts.len() > 1usize {
                panic!("Only one account in token 2.5");
            }
            let acc = &self
                .active_accounts
                .first()
                .expect("get first account from hdkeystore");
            pw.insert("address".to_string(), json!(acc.address.to_string()));
            let coin_split: Vec<&str> = acc.coin.split('-').collect();
            coin_split.iter().enumerate().for_each(|(i, s)| {
                if i == 0 {
                    pw.insert("chainType".to_string(), json!(s));
                } else if vec!["NONE", "P2WPKH"].contains(s) {
                    pw.insert("segWit".to_string(), json!(s));
                }
            });
            let mut obj = Value::Object(pw);
            if let Some(extra) = acc.extra.as_object() {
                merge_value(&mut obj, &Value::Object(extra.clone()))
            }
            write!(
                f,
                "{}",
                serde_json::to_string(&obj).expect("present err when convert to json")
            )
        } else {
            write!(
                f,
                "{}",
                serde_json::to_string(&pw).expect("present err when convert to json")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin_hashes::hex::ToHex;
    use serde_json::Map;
    use tcx_primitive::Public;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    pub fn keystore_json_test() {
        let json = r#"
        {
    "id": "41923f0c-427b-4e5f-a55c-a6a30d2ee0a5",
    "version": 11000,
    "keyType": "MNEMONIC",
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
        let keystore: HdKeystore = HdKeystore::load(json).unwrap();
        assert_eq!(keystore.active_accounts.len(), 2);
        assert_eq!(
            Value::from_str(&keystore.json()).unwrap(),
            Value::from_str(json).unwrap()
        );
    }

    #[test]
    pub fn default_meta_test() {
        let meta = Metadata::default();
        let expected = Metadata {
            name: String::from("Unknown"),
            password_hint: String::new(),
            timestamp: metadata_default_time(),
            source: Source::Mnemonic,
        };

        assert_eq!(meta.name, expected.name);
        assert_eq!(meta.password_hint, expected.password_hint);
        assert_eq!(meta.source, expected.source);
    }

    struct MockAddress {}
    impl Address for MockAddress {
        fn from_public_key(_public_key: &[u8], _coin: Option<&str>) -> Result<String> {
            Ok("mock_address".to_string())
        }

        fn from_private_key(private_key: &str, coin: Option<&str>) -> Result<String> {
            Ok("mock_address".to_string())
        }
    }

    #[test]
    pub fn mnemonic_to_account_test() {
        let coin_info = CoinInfo {
            symbol: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        };
        let account =
            HdKeystore::mnemonic_to_account::<MockAddress, EmptyExtra>(&coin_info, MNEMONIC)
                .unwrap();
        let expected = Account {
            address: "mock_address".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            coin: "BITCOINCASH".to_string(),
            extra: Value::Object(Map::new()),
        };
        assert_eq!(account, expected);
    }

    #[test]
    pub fn new_keystore() {
        let keystore = HdKeystore::new(PASSWORD, Metadata::default());
        assert_eq!(keystore.version, 11000);
        assert_ne!(keystore.id, "");
        assert_eq!(keystore.active_accounts.len(), 0);
        assert_eq!(keystore.key_type, KeyType::Mnemonic);
    }

    #[test]
    pub fn from_mnemonic_test() {
        let keystore = HdKeystore::from_mnemonic(MNEMONIC, PASSWORD, Metadata::default());
        assert_eq!(keystore.version, 11000);
        assert_ne!(keystore.id, "");
        let decrypted_bytes = keystore.crypto.decrypt(PASSWORD).unwrap();
        let decrypted_mnemonic = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(decrypted_mnemonic, MNEMONIC);
        assert_eq!(keystore.active_accounts.len(), 0);
        assert_eq!(keystore.key_type, KeyType::Mnemonic);

        let mnemonic = keystore.mnemonic(PASSWORD).unwrap();
        assert_eq!(mnemonic, MNEMONIC);

        let expected_seed = "ee3fce3ccf05a2b58c851e321077a63ee2113235112a16fc783dc16279ff818a549ff735ac4406c624235db2d37108e34c6cbe853cbe09eb9e2369e6dd1c5aaa";
        let seed = keystore.seed(PASSWORD).unwrap();
        assert_eq!(seed.to_hex(), expected_seed);

        let wrong_password_err = keystore.mnemonic("WrongPassword").err().unwrap();
        assert_eq!(format!("{}", wrong_password_err), "password_incorrect");

        let wrong_password_err = keystore.seed("WrongPassword").err().unwrap();
        assert_eq!(format!("{}", wrong_password_err), "password_incorrect");
    }

    #[test]
    pub fn from_private_key_test() {
        let keystore = HdKeystore::from_private_key(
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            PASSWORD,
            Source::Private,
        );
        assert_eq!(keystore.version, 11000);
        assert_ne!(keystore.id, "");
        assert_eq!(keystore.active_accounts.len(), 0);
        assert_eq!(keystore.key_type, KeyType::PrivateKey);
    }
    //
    //    #[test]
    //    pub fn get_pair_test() {
    //        let keystore = HdKeystore::from_mnemonic(MNEMONIC, PASSWORD, Metadata::default());
    //        let pair = keystore
    //            .get_pair::<tcx_primitive::key::secp256k1::Pair>("m/44'/0'/0'", PASSWORD)
    //            .unwrap();
    //        let xpub = pair.to_string();
    //        assert_eq!(xpub, "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ");
    //    }

    #[test]
    pub fn derive_key_at_paths_test() {
        let mut keystore = HdKeystore::from_mnemonic(MNEMONIC, PASSWORD, Metadata::default());
        let coin_info = CoinInfo {
            symbol: "BITCOIN".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        };
        let acc = keystore
            .derive_coin::<MockAddress, EmptyExtra>(&coin_info, PASSWORD)
            .unwrap();
        let expected = Account {
            address: "mock_address".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            coin: "BITCOIN".to_string(),
            extra: Value::Object(Map::new()),
        };

        assert_eq!(acc, &expected);
        assert_eq!(keystore.account("BITCOIN").unwrap(), &expected);
        assert_eq!(keystore.active_accounts.len(), 1);

        let paths = vec![
            "m/44'/0'/0'/0/0",
            "m/44'/0'/0'/0/1",
            "m/44'/0'/0'/1/0",
            "m/44'/0'/0'/1/1",
        ];
        let pairs = keystore
            .key_pair_at_paths("BITCOIN", &paths, PASSWORD)
            .unwrap();
        let pub_keys = pairs
            .iter()
            .map(|pair| pair.public_key().to_bytes().unwrap().to_hex())
            .collect::<Vec<String>>();
        let expected_pub_keys = vec![
            "026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868",
            "024fb7df3961e08f01025e434ea19708a4317d2fe59775cddd38df6e8a2d30697d",
            "0352470ace48f25b01b9c341e3b0e033fc32a203fb7a81a0453f97d94eca819a35",
            "022f4c38f7bbaa00fc886db62f975b34201c2bfed146e98973caf03268941801db",
        ];
        assert_eq!(pub_keys, expected_pub_keys);
    }
}
