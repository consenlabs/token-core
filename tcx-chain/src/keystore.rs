use bitcoin::network::constants::Network;
use bitcoin::util::address::Address as BtcAddress;
use secp256k1::Secp256k1;
use bitcoin::PrivateKey as BtcPrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language, Seed};
use std::str::FromStr;
use bitcoin_hashes::hex::{ToHex, FromHex};
use serde::{Deserialize, Serialize};
use tcx_crypto::{Crypto, Pbkdf2Params, EncPair};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use failure::Error;
use crate::Result;
use crate::bips;
use crate::bips::DerivationInfo;
use crate::curve::{CurveType, PublicKeyType, PrivateKey, PublicKey, Secp256k1Curve, Secp256k1PrivateKey};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    Wif,
    Private,
    Keystore,
    Mnemonic,
    NewIdentity,
    RecoveredIdentity,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    pub password_hint: String,
    #[serde(default = "metadata_default_time")]
    pub timestamp: i64,
    #[serde(default = "metadata_default_source")]
    pub source: Source,

}

fn metadata_empty_str() -> String {
    "".to_owned()
}

fn metadata_default_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("get timestamp");
    since_the_epoch.as_secs() as i64
}

fn metadata_default_source() -> Source {
    Source::Mnemonic
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            name: String::from("BCH"),
            password_hint: String::new(),
            timestamp: metadata_default_time(),
            source: Source::Mnemonic,
        }
    }
}


pub trait Address {
//    type PubKey: PublicKey;

     fn is_valid(address: &str) -> bool;
//     fn new(address: &str) -> String;
    // Incompatible between the trait `Address:PubKey is not implemented for `&<impl curve::PrivateKey as curve::PrivateKey>::PublicKey`
     fn from_public_key(public_key: &[u8]) -> Result<String>;
    // fn from_data(data: &[u8]) -> Box<dyn Address>;

    fn extended_public_key_version() -> [u8;4] {
        // default use btc mainnet
        [0x04, 0x88, 0xb2, 0x1e]
    }
    fn extended_private_key_version() -> [u8;4] {
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

enum ExtendedPubKeyType {
    XPUB()
}

pub struct CoinInfo {
    pub symbol: String,
    pub derivation_path: String,
    pub curve: CurveType,
    pub pub_key_type: PublicKeyType,
}



// todo: process the extra field
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub derivation_path: String,
    pub extended_public_key: String,
    pub curve: CurveType,
    pub pub_key_type: PublicKeyType,
    pub coin: String,
    #[serde(skip_deserializing)]
    pub extra: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyType {
    PrivateKey,
    Mnemonic
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
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

    pub fn mnemonic_to_account<A: Address>(coin_info: &CoinInfo, mnemonic: &str) -> Result<Account>{
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English).map_err(|_| format_err!("invalid_mnemonic"))?;
        let seed = bip39::Seed::new(&mnemonic, &"");
        Self::derive_account_from_coin::<A>(coin_info, &seed)
    }

    pub fn derive_account_from_coin<A: Address>(coin_info: &CoinInfo, seed: &Seed) -> Result<Account> {

        let paths = vec![coin_info.derivation_path.clone()];
        let keys = Self::key_at_paths_with_seed(coin_info.curve, &paths, &seed)?;
        let key = keys.first().ok_or(format_err!("derivate_failed"))?;
        let pub_key = key.public_key();
        // todo: process the main address path
        let address = A::from_public_key(&pub_key.to_bytes())?;
        let derivation_info = match coin_info.curve {
            CurveType::SECP256k1 => {
                Secp256k1Curve::extended_pub_key(&coin_info.derivation_path, &seed)
            },
            _ => Err(format_err!("{}", "unsupport_chain"))
        }?;
        let xpub = A::extended_public_key(&derivation_info);

        Ok(Account {
            address,
            derivation_path: coin_info.derivation_path.to_string(),
            extended_public_key: xpub,
            curve: coin_info.curve,
            pub_key_type: coin_info.pub_key_type,
            coin: coin_info.symbol.to_string(),
            extra: "".to_string()
        })
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
        let mnemonic = Mnemonic::from_phrase(mnemonic_str, Language::English).map_err(|_| format_err!("invalid_mnemonic"))?;
        Ok(bip39::Seed::new(&mnemonic, &""))
    }

    fn key_at_paths_with_seed(curve: CurveType, paths: &[impl AsRef<str>], seed: &Seed) -> Result<Vec<impl PrivateKey>> {
        match curve {
            CurveType::SECP256k1 => {
                Secp256k1Curve::key_at_paths_with_seed(paths, seed)
            },
            _ => Err(format_err!("{}", "unsupport_curve"))
        }
    }

    pub fn key_at_paths(&self, symbol: &str, paths: &[impl AsRef<str>], password: &str) -> Result<Vec<impl PrivateKey>> {
        let acc = self.account(symbol).ok_or(format_err!("{}", "account_not_found"))?;
        let seed = self.seed(password)?;
        Ok(Self::key_at_paths_with_seed(acc.curve, paths, &seed)?)
    }

    pub fn derive_coin<A: Address>(&mut self, coin_info: &CoinInfo, password: &str) -> Result<&Account>{
        let seed = self.seed(password)?;
        let account = Self::derive_account_from_coin::<A>(coin_info, &seed)?;
        self.active_accounts.push(account);
        Ok(self.active_accounts.last().unwrap())
    }

    pub fn account(&self, symbol: &str) -> Option<&Account> {
        self.active_accounts.iter().find(|acc| acc.coin == symbol)
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn load(json: &str) -> Result<HdKeystore> {
        let ret: HdKeystore = serde_json::from_str(json)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";

    #[test]
    pub fn it_works() {
        assert_eq!(1, 1)
    }

    #[test]
    pub fn restore_keystore() {
        let json = r#"
        {
    "id": "41923f0c-427b-4e5f-a55c-a6a30d2ee0a5",
    "version": 11000,
    "keyType": "mnemonic",
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
            "extendedPublicKey": "zpub6qsMtyUc63xx7hDdL5MnLUT3jNV2opgWiugqiYc2CwFdgJPJeC57kQ6VxYiENXtgdDd5APjNHoTHDqj5iyitUo8i66fSsEguf8gPd6LtHkP",
            "coin": "BTC"
        },
        {
            "address": "tokencorex66",
            "derivationPath": "m/84'/0'/0'/0/0",
            "extendedPublicKey": "zpub6qsMtyUc63xx7hDdL5MnLUT3jNV2opgWiugqiYc2CwFdgJPJeC57kQ6VxYiENXtgdDd5APjNHoTHDqj5iyitUo8i66fSsEguf8gPd6LtHkP",
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
//        let keystore = HdKeystore::load(&json);
        let keystore: HdKeystore = serde_json::from_str(&json).unwrap();

//        assert!(keystore.is_ok());
        assert_eq!(keystore.active_accounts.len(), 2);
    }
}

