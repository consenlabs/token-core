use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
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
    pub chain_type: String,
    #[serde(default = "metadata_default_time")]
    pub timestamp: i64,
    #[serde(default = "metadata_empty_str")]
    pub network: String,
    #[serde(default = "metadata_default_source")]
    pub source: Source,
    #[serde(default = "metadata_empty_str")]
    pub mode: String,
    #[serde(default = "metadata_empty_str")]
    pub wallet_type: String,
    #[serde(default = "metadata_empty_str")]
    pub seg_wit: String,
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
            chain_type: String::from("BCH"),
            timestamp: metadata_default_time(),
            network: String::from("MAINNET"),
            source: Source::Mnemonic,
            mode: String::from("NORMAL"),
            wallet_type: String::from("HD"),
            seg_wit: String::from("NONE"),
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
        let mnemonic = Mnemonic::from_phrase(mnemonic_str, Language::English).map_err(| _ | format_err!("invalid_mnemonic"))?;
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
        let address = Address::p2pkh(&pub_key, Network::Bitcoin);
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
    let prv_key = PrivateKey::from_wif(wif).map_err(| _ | format_err!("invalid_wif"))?;
    let pub_key = prv_key.public_key(&s);
    // Generate pay-to-pubkey-hash address
    let address = Address::p2pkh(&pub_key, Network::Bitcoin);
    Ok(address.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";


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
        let address = generate_address_from_wif("L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy");
        assert_eq!("17XBj6iFEsf8kzDMGQk5ghZipxX49VXuaV", address);
    }
}

