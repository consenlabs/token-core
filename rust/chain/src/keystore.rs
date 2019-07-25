use bitcoin::network::constants::Network;
use bitcoin::util::address::Address as BtcAddress;
use secp256k1::Secp256k1;
use bitcoin::PrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language};
use std::str::FromStr;
use bitcoin_hashes::hex::{ToHex, FromHex};
use serde::{Deserialize, Serialize};
use tcx_crypto::{Crypto, Pbkdf2Params, EncPair};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use failure::Error;
use crate::Result;
use crate::bips;

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


// Send  used fo lazy_static
pub trait Keystore: Send {
    fn get_metadata(&self) -> Metadata;
    fn get_address(&self) -> String;
    fn decrypt_cipher_text(&self, password: &str) -> Result<Vec<u8>>;
    fn export_json(&self) -> String;
    fn get_id(&self) -> String;
    fn clone_box(&self) -> Box<Keystore>;
    fn set_id(&mut self, id: String);
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V3Keystore {
    pub id: String,
    pub version: i32,
    pub address: String,
    pub crypto: Crypto<Pbkdf2Params>,
    pub metadata: Metadata,
}

impl V3Keystore {
    pub fn new(metadata: Metadata, password: &str, prv_key: &str) -> Result<V3Keystore> {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, prv_key.to_owned().as_bytes());
        let mut metadata = metadata.clone();
        metadata.source = Source::Wif;
        let keystore = V3Keystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 3,
            address: generate_address_from_wif(prv_key)?,
            crypto,
            metadata,
        };
        Ok(keystore)
    }
}

impl Keystore for V3Keystore {
    fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn get_address(&self) -> String {
        self.address.clone()
    }

    fn decrypt_cipher_text(&self, password: &str) -> Result<Vec<u8>> {
        self.crypto.decrypt(password)
    }

    fn export_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn get_id(&self) -> String {
        self.id.to_owned()
    }

    fn clone_box(&self) -> Box<Keystore> {
        Box::new(self.clone()) as Box<Keystore>
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

pub struct V3MnemonicKeystore {
    id: String,
    version: i32,
    address: String,
    crypto: Crypto<Pbkdf2Params>,
    mnemonic_path: String,
    enc_mnemonic: EncPair,
}

impl V3MnemonicKeystore {
    pub fn new(password: &str, mnemonic: &str, path: &str) -> Result<V3MnemonicKeystore> {
        let prv_key = Self::generate_prv_key_from_mnemonic(mnemonic, path)?;
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, &prv_key.to_bytes());
        let enc_mnemonic = crypto.derive_enc_pair(password, mnemonic.as_bytes());

        let keystore = V3MnemonicKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 3,
            address: Self::address_from_private_key(&prv_key),
            crypto,
            mnemonic_path: String::from(path),
            enc_mnemonic,
        };
        Ok(keystore)
    }

    fn generate_prv_key_from_mnemonic(mnemonic_str: &str, path: &str) -> Result<PrivateKey> {
        let mnemonic = Mnemonic::from_phrase(mnemonic_str, Language::English).map_err(|_| format_err!("invalid_mnemonic"))?;
        let seed = bip39::Seed::new(&mnemonic, &"");
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;

        let path = DerivationPath::from_str(path)?;
        let main_address_pk = sk.derive_priv(&s, &path)?;
        Ok(main_address_pk.private_key)
    }

    fn address_from_private_key(pk: &PrivateKey) -> String {
        let s = Secp256k1::new();
        let pub_key = pk.public_key(&s);
        // Generate pay-to-pubkey-hash address
        let address = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
        address.to_string()
    }

    pub fn export_private_key(&self, password: &str) -> Result<String> {
        let pk_bytes = self.crypto.decrypt(password)?;
        let pk = pk_bytes.to_hex();
        Ok(pk)
    }
}

fn generate_address_from_wif(wif: &str) -> Result<String> {
    let s = Secp256k1::new();
    let prv_key = PrivateKey::from_wif(wif).map_err(|_| format_err!("invalid_wif"))?;
    let pub_key = prv_key.public_key(&s);
    // Generate pay-to-pubkey-hash address
    let address = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
    Ok(address.to_string())
}


pub trait Address {
    // fn is_valid(address: &str) -> bool;
    // fn new(address: &str) -> Box<dyn Address>;
    // fn from_public_key(public_key: &str) -> Box<dyn Address>;
    // fn from_data(data: &[u8]) -> Box<dyn Address>;
}

// todo: process the extra field
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub derivation_path: String,
    pub extended_public_key: String,
    pub coin: String,
    #[serde(skip_deserializing)]
    pub extra: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdKeystore {
    pub id: String,
    pub version: i32,
    pub key_type: String,
    pub crypto: Crypto<Pbkdf2Params>,
    pub active_accounts: Vec<Account>,
    #[serde(alias = "imTokenMeta")]
    pub meta: Metadata,
}

impl HdKeystore {
    pub fn new(password: &str) -> HdKeystore {
        let mnemonic = bips::generate_mnemonic();
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        let meta = Metadata::default();
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 11000,
            key_type: "MNEMONIC".to_string(),
            crypto,
            active_accounts: vec![],
            meta,
        }
    }

    pub fn from_mnemonic(mnemonic: &str, password: &str) -> HdKeystore {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        let meta = Metadata::default();
        HdKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 11000,
            key_type: "MNEMONIC".to_string(),
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
            key_type: "PRIVATE_KEY".to_string(),
            crypto,
            active_accounts: vec![account],
            meta,
        }
    }

    pub fn append_account(&mut self, account: Account) {
        self.active_accounts.push(account);
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
    pub fn new_v3_mnemonic_keystore() {
//        let meta = Metadata::default();
        let keystore = V3MnemonicKeystore::new(&PASSWORD, &MNEMONIC, &ETHEREUM_PATH);

        assert!(keystore.is_ok());

        let keystore = keystore.unwrap();
        assert_eq!("16Hp1Ga779iaTe1TxUFDEBqNCGvfh3EHDZ", keystore.address);

//        println!(se)
    }

    #[test]
    pub fn bch_address() {
        match generate_address_from_wif("L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy") {
            Ok(address) => assert_eq!("17XBj6iFEsf8kzDMGQk5ghZipxX49VXuaV", address),
            Err(_) => panic!("could not get address"),
        };
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

