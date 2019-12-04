use bip39::{Language, Mnemonic};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use super::guard::KeystoreGuard;
use super::Address;
use super::Result;
use super::{Account, Extra};
use super::{Error, Metadata, Source};

use crate::keystore::Keystore;
use core::{fmt, result};
use serde_json::{Map, Value};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tcx_constants::{CoinInfo, CurveType};
use tcx_crypto::{Crypto, Pbkdf2Params};
use tcx_primitive::{
    generate_mnemonic, Bip32DeterministicPrivateKey, Bip32DeterministicPublicKey, Derive,
    DerivePath, DeterministicPrivateKey, DeterministicPublicKey, PrivateKey, PublicKey,
    TypedPrivateKey,
};

/// Primary keystore type to store a root seed for deriving multi chain accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdKeystore {
    pub id: String,
    pub version: i64,
    pub crypto: Crypto<Pbkdf2Params>,
    pub active_accounts: Vec<Account>,

    #[serde(rename = "imTokenMeta")]
    pub meta: Metadata,

    #[serde(skip_serializing)]
    seed: Option<Vec<u8>>,
}

impl HdKeystore {
    pub const VERSION: i64 = 11000i64;

    pub fn unlock_by_password(&mut self, password: &str) -> Result<()> {
        self.seed = Some(self.decrypt_seed(password)?);

        Ok(())
    }

    pub fn lock(&mut self) {
        self.seed = None;
    }

    pub fn find_private_key(&self, address: &str) -> Result<TypedPrivateKey> {
        unimplemented!()
    }

    pub fn new(password: &str, meta: Metadata) -> HdKeystore {
        let mnemonic = generate_mnemonic();
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: Self::VERSION,
            crypto,
            active_accounts: vec![],
            meta,
            seed: None,
        }
    }

    pub fn from_mnemonic(mnemonic: &str, password: &str, meta: Metadata) -> HdKeystore {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: Self::VERSION,
            crypto,
            active_accounts: vec![],
            meta,
            seed: None,
        }
    }

    pub fn get_private_key(&self, path: &str) -> Result<impl PrivateKey> {
        tcx_ensure!(self.seed.is_some(), Error::KeystoreLocked);

        let seed = self.seed.as_ref().unwrap().as_slice();

        let esk = Bip32DeterministicPrivateKey::from_seed(seed)?;
        let p = DerivePath::from_str(path)?.into_iter();

        let sk = esk.derive(p)?;

        Ok(sk.private_key())
    }

    /// Derive account from a mnemonic phase
    pub fn mnemonic_to_account<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        mnemonic: &str,
    ) -> Result<Account> {
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)
            .map_err(|_| Error::InvalidMnemonic)?;
        let seed = bip39::Seed::new(&mnemonic, &"");
        Self::derive_account_from_coin::<A, E>(coin_info, seed.as_bytes())
    }

    pub fn derive_account_from_coin<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        seed: &[u8],
    ) -> Result<Account> {
        let paths = vec![coin_info.derivation_path.clone()];
        let keys = Self::key_at_paths_with_seed(coin_info.curve, &paths, &seed)?;
        let key = keys.first().ok_or_else(|| format_err!("derivate_failed"))?;
        let pub_key = key.private_key().public_key();
        let bytes = pub_key.to_bytes();
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

    pub fn mnemonic(&self, password: &str) -> Result<String> {
        let mnemonic_bytes = self.crypto.decrypt(password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes)?;
        Ok(mnemonic)
    }

    pub fn decrypt_seed(&self, password: &str) -> Result<Vec<u8>> {
        let mnemonic_str = self.mnemonic(password)?;
        let mnemonic = Mnemonic::from_phrase(mnemonic_str, Language::English)
            .map_err(|_| Error::InvalidMnemonic)?;
        Ok(bip39::Seed::new(&mnemonic, &"").as_bytes().to_vec())
    }

    fn key_at_paths_with_seed(
        curve: CurveType,
        paths: &[impl AsRef<str>],
        seed: &[u8],
    ) -> Result<Vec<Bip32DeterministicPrivateKey>> {
        match curve {
            CurveType::SECP256k1 => {
                let private_key = Bip32DeterministicPrivateKey::from_seed(seed)
                    .map_err(|_| Error::CanNotDerivePairFromSeed)?;
                let children: Result<Vec<Bip32DeterministicPrivateKey>> = paths
                    .iter()
                    .map(|path| {
                        let path = DerivePath::from_str(path.as_ref())
                            .map_err(|_| Error::CannotDeriveKey)?;
                        let child_private_key = private_key
                            .derive(path.into_iter())
                            .map_err(|_| Error::CannotDeriveKey)?;
                        Ok(child_private_key)
                    })
                    .collect();

                children
            }

            _ => Err(Error::UnsupportedCurve.into()),
        }
    }

    /// Derive a private key at a specific path, it's coin independent
    pub fn key_at_paths(
        &self,
        symbol: &str,
        paths: &[impl AsRef<str>],
    ) -> Result<Vec<Bip32DeterministicPrivateKey>> {
        let acc = self.account(symbol).ok_or(Error::AccountNotFound)?;

        tcx_ensure!(self.seed.is_some(), Error::KeystoreLocked);

        let seed = self.seed.as_ref().unwrap().as_slice();

        Ok(Self::key_at_paths_with_seed(acc.curve, paths, seed)?)
    }

    /// Derive an account on a specific coin
    pub fn derive_coin<A: Address, E: Extra>(&mut self, coin_info: &CoinInfo) -> Result<&Account> {
        tcx_ensure!(self.seed.is_some(), Error::KeystoreLocked);

        let seed = self.seed.as_ref().unwrap().as_slice();

        let account = Self::derive_account_from_coin::<A, E>(coin_info, seed)?;

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
    use crate::keystore::{metadata_default_time, EmptyExtra};
    use bitcoin_hashes::hex::ToHex;
    use serde_json::Map;
    use tcx_primitive::{PublicKey, TypedPublicKey};

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

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
        fn from_public_key(_pk: &[u8], _coin: Option<&str>) -> Result<String> {
            Ok("mock_address".to_string())
        }

        fn is_valid(address: &str) -> bool {
            true
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

        let mnemonic = keystore.mnemonic(PASSWORD).unwrap();
        assert_eq!(mnemonic, MNEMONIC);

        let expected_seed = "ee3fce3ccf05a2b58c851e321077a63ee2113235112a16fc783dc16279ff818a549ff735ac4406c624235db2d37108e34c6cbe853cbe09eb9e2369e6dd1c5aaa";

        let seed = keystore.decrypt_seed(PASSWORD).unwrap();
        assert_eq!(seed.to_hex(), expected_seed);

        let wrong_password_err = keystore.mnemonic("WrongPassword").err().unwrap();
        assert_eq!(format!("{}", wrong_password_err), "password_incorrect");

        let wrong_password_err = keystore.decrypt_seed("WrongPassword").err().unwrap();
        assert_eq!(format!("{}", wrong_password_err), "password_incorrect");
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
        let _ = keystore.unlock_by_password(PASSWORD);

        let acc = keystore
            .derive_coin::<MockAddress, EmptyExtra>(&coin_info)
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

        let _ = keystore.unlock_by_password(PASSWORD);
        let private_keys = keystore.key_at_paths("BITCOIN", &paths).unwrap();
        let pub_keys = private_keys
            .iter()
            .map(|epk| epk.private_key().public_key().to_bytes().to_hex())
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
