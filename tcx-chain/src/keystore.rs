use bip39::{Language, Mnemonic, Seed};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use tcx_crypto::{Crypto, Pbkdf2Params};

use crate::bips;
use crate::bips::DerivationInfo;
use crate::curve::{CurveType, PrivateKey, PublicKey, Secp256k1Curve};
use crate::Result;

/// Source to remember which format it comes from
///
/// NOTE: Identity related type is only for imToken App v2.x
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    fn is_valid(address: &str) -> bool;
    // Incompatible between the trait `Address:PubKey is not implemented for `&<impl curve::PrivateKey as curve::PrivateKey>::PublicKey`
    fn from_public_key(public_key: &impl PublicKey) -> Result<String>;
    // fn from_data(data: &[u8]) -> Box<dyn Address>;

    fn extended_public_key_version() -> [u8; 4] {
        // default use btc mainnet
        [0x04, 0x88, 0xb2, 0x1e]
    }
    fn extended_private_key_version() -> [u8; 4] {
        // default use btc mainnet
        [0x04, 0x88, 0xad, 0xe4]
    }

    fn extended_public_key(derivation_info: &DerivationInfo) -> String {
        derivation_info.encode_with_network(Self::extended_public_key_version())
    }

    fn extended_private_key(derivation_info: &DerivationInfo) -> String {
        derivation_info.encode_with_network(Self::extended_private_key_version())
    }
}

/// Blockchain basic config
///
/// NOTE: Unique key field is `symbol`
pub struct CoinInfo {
    pub symbol: String,
    pub derivation_path: String,
    pub curve: CurveType,
}

/// Account that presents one blockchain wallet on a keystore
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn from(coin_info: &CoinInfo, seed: &Seed) -> Result<Self>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyExtra {}

impl Extra for EmptyExtra {
    fn from(_coin_info: &CoinInfo, _seed: &Seed) -> Result<Self> {
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
    #[serde(alias = "imTokenMeta")]
    pub meta: Metadata,
}

impl HdKeystore {
    pub const VERSION: i32 = 11000i32;

    /// Derive account from a mnemonic phase
    pub fn mnemonic_to_account<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        mnemonic: &str,
    ) -> Result<(Account, E)> {
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)
            .map_err(|_| format_err!("invalid_mnemonic"))?;
        let seed = bip39::Seed::new(&mnemonic, &"");
        Self::derive_account_from_coin::<A, E>(coin_info, &seed)
    }

    pub fn derive_account_from_coin<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        seed: &Seed,
    ) -> Result<(Account, E)> {
        let paths = vec![coin_info.derivation_path.clone()];
        let keys = Self::key_at_paths_with_seed(coin_info.curve, &paths, &seed)?;
        let key = keys.first().ok_or(format_err!("derivate_failed"))?;
        let pub_key = key.public_key();
        let address = A::from_public_key(&pub_key)?;

        let extra = E::from(coin_info, seed)?;
        let acc = Account {
            address,
            derivation_path: coin_info.derivation_path.to_string(),
            curve: coin_info.curve,
            coin: coin_info.symbol.to_string(),
            extra: serde_json::to_value(extra.clone()).expect("extra_error"),
        };
        Ok((acc, extra))
    }

    pub fn new(password: &str, meta: Metadata) -> HdKeystore {
        let mnemonic = bips::generate_mnemonic();
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

    pub fn from_private_key(private_key: &str, password: &str, account: Account) -> HdKeystore {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, private_key.as_bytes());
        let meta = Metadata::default();
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 11000,
            key_type: KeyType::PrivateKey,
            crypto,
            active_accounts: vec![account],
            meta,
        }
    }

    pub fn mnemonic(&self, password: &str) -> Result<String> {
        if self.key_type != KeyType::Mnemonic {
            return Err(format_err!("{}", "invalid_key_type"));
        }
        let mnemonic_bytes = self.crypto.decrypt(password)?;
        let mnemonic = String::from_utf8(mnemonic_bytes)?;
        Ok(mnemonic)
    }

    pub fn seed(&self, password: &str) -> Result<Seed> {
        let mnemonic_str = self.mnemonic(password)?;
        let mnemonic = Mnemonic::from_phrase(mnemonic_str, Language::English)
            .map_err(|_| format_err!("invalid_mnemonic"))?;
        Ok(bip39::Seed::new(&mnemonic, &""))
    }

    fn key_at_paths_with_seed(
        curve: CurveType,
        paths: &[impl AsRef<str>],
        seed: &Seed,
    ) -> Result<Vec<impl PrivateKey>> {
        match curve {
            CurveType::SECP256k1 => Secp256k1Curve::key_at_paths_with_seed(paths, seed),
            _ => Err(format_err!("{}", "unsupport_curve")),
        }
    }

    /// Derive a private key at a specific path, it's coin independent
    pub fn key_at_paths(
        &self,
        symbol: &str,
        paths: &[impl AsRef<str>],
        password: &str,
    ) -> Result<Vec<impl PrivateKey>> {
        let acc = self
            .account(symbol)
            .ok_or(format_err!("{}", "account_not_found"))?;
        let seed = self.seed(password)?;
        Ok(Self::key_at_paths_with_seed(acc.curve, paths, &seed)?)
    }

    /// Derive an account on a specific coin
    pub fn derive_coin<A: Address, E: Extra>(
        &mut self,
        coin_info: &CoinInfo,
        password: &str,
    ) -> Result<(&Account, E)> {
        let seed = self.seed(password)?;
        let (account, extra) = Self::derive_account_from_coin::<A, E>(coin_info, &seed)?;
        self.active_accounts.push(account);
        Ok((self.active_accounts.last().unwrap(), extra))
    }

    /// Find an account by coin symbol
    pub fn account(&self, symbol: &str) -> Option<&Account> {
        self.active_accounts.iter().find(|acc| acc.coin == symbol)
    }

    // TODO: rename to `to_json`
    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Load a json to create HD keystore instance
    pub fn load(json: &str) -> Result<HdKeystore> {
        let ret: HdKeystore = serde_json::from_str(json)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";

    #[test]
    pub fn test_defualt_metadata() {
        let md = Metadata::default();
        assert_eq!(md.name, "Unknown");
    }

    #[test]
    pub fn test_derive_account() {
        let hdks = HdKeystore::new("insecure", Metadata::default());
        assert_eq!(0, hdks.active_accounts.len());
    }

    #[test]
    pub fn restore_keystore() {
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
            "coin": "BTC",
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
        let keystore: HdKeystore = serde_json::from_str(&json).unwrap();
        assert_eq!(keystore.active_accounts.len(), 2);
    }
}
