use crate::SubstrateAddress;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use tcx_chain::Address;

use std::time::{SystemTime, UNIX_EPOCH};
use tcx_constants::{CoinInfo, Result};
use tcx_primitive::{
    DeterministicPrivateKey, PrivateKey, PublicKey, Sr25519PrivateKey, TypedPublicKey,
};
use xsalsa20poly1305::aead::{generic_array::GenericArray, Aead, NewAead};
use xsalsa20poly1305::XSalsa20Poly1305;

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_keystore# {}", _0)]
    InvalidKeystore(String),
    #[fail(display = "keystore_public_key_unmatch")]
    KeystorePublicKeyUnmatch,
    #[fail(display = "password_incorrect")]
    PasswordIncorrect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubstrateKeystore {
    pub address: String,
    pub encoded: String,
    pub encoding: SubstrateKeystoreEncoding,
    pub meta: SubstrateKeystoreMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubstrateKeystoreEncoding {
    pub content: Vec<String>,
    #[serde(rename = "type")]
    pub encoding_type: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubstrateKeystoreMeta {
    pub name: String,
    pub when_created: i64,
}

fn metadata_default_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("get timestamp");
    since_the_epoch.as_secs() as i64
}

impl Default for SubstrateKeystoreMeta {
    fn default() -> Self {
        SubstrateKeystoreMeta {
            name: "Unknown".to_string(),
            when_created: metadata_default_time(),
        }
    }
}

const NONCE_LENGTH: usize = 24;
const PKCS8_DIVIDER: [u8; 5] = [161, 35, 3, 33, 0];
const PKCS8_HEADER: [u8; 16] = [48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32];
const PUB_LENGTH: usize = 32;
const SEC_LENGTH: usize = 64;
const SEED_OFFSET: usize = PKCS8_HEADER.len();
const SEED_LENGTH: usize = 32;

impl SubstrateKeystore {
    pub fn new(
        password: &str,
        prv_key: &[u8],
        pub_key: &[u8],
        addr: &str,
    ) -> Result<SubstrateKeystore> {
        let encoding = format!(
            "0x{}",
            SubstrateKeystore::encrypt(password, prv_key, pub_key)?
        );

        Ok(SubstrateKeystore {
            address: addr.to_string(),
            encoded: encoding.to_string(),
            encoding: SubstrateKeystoreEncoding {
                content: vec!["pkcs8".to_string(), "sr25519".to_string()],
                encoding_type: "xsalsa20-poly1305".to_string(),
                version: "2".to_string(),
            },
            meta: SubstrateKeystoreMeta::default(),
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.address.is_empty() {
            return Err(Error::InvalidKeystore("address is empty".to_string()).into());
        }
        if self.encoded.is_empty() {
            return Err(Error::InvalidKeystore("encoded is empty".to_string()).into());
        }
        if self.encoding.content[0] != "pkcs8" {
            return Err(Error::InvalidKeystore("need pkcs8 padding".to_string()).into());
        }
        if self.encoding.content[1] != "sr25519" {
            return Err(Error::InvalidKeystore("only support sr25519".to_string()).into());
        }
        if self.encoding.encoding_type != "xsalsa20-poly1305" {
            return Err(
                Error::InvalidKeystore("only support xsalsa20-poly1305".to_string()).into(),
            );
        }
        if self.encoding.version != "2" {
            return Err(Error::InvalidKeystore("only support version 2".to_string()).into());
        }

        Ok(())
    }

    pub fn decrypt(&self, password: &str) -> Result<(Vec<u8>, Vec<u8>)> {
        let _ = self.validate()?;
        let encoded = if self.encoded.starts_with("0x") {
            &self.encoded[2..]
        } else {
            &self.encoded
        };

        let decrypted = decrypt_content(password, encoded)?;
        let header = &decrypted[0..PKCS8_HEADER.len()];

        assert!(header == PKCS8_HEADER, "Invalid Pkcs8 header found in body");

        let mut secret_key =
            decrypted[PKCS8_HEADER.len()..PKCS8_HEADER.len() + SEC_LENGTH].to_vec();

        let mut div_offset: usize = SEED_OFFSET + SEC_LENGTH;
        let mut divider = &decrypted[div_offset..div_offset + PKCS8_DIVIDER.len()];
        if divider != PKCS8_DIVIDER {
            div_offset = SEED_OFFSET + SEED_LENGTH;
            secret_key = decrypted[SEED_OFFSET..div_offset].to_vec();
            divider = &decrypted[div_offset..div_offset + PKCS8_DIVIDER.len()];
        }

        assert!(
            divider == PKCS8_DIVIDER,
            "Invalid Pkcs8 divider found in body"
        );

        let pub_offset = div_offset + PKCS8_DIVIDER.len();
        let pub_key = &decrypted[pub_offset..pub_offset + PUB_LENGTH];
        Ok((secret_key, pub_key.to_vec()))
    }

    pub fn encrypt(password: &str, seed: &[u8], pub_key: &[u8]) -> Result<String> {
        let plaintext = [&PKCS8_HEADER, seed, &PKCS8_DIVIDER, pub_key].concat();
        encrypt_content(password, &plaintext)
    }
}

fn decrypt_content(password: &str, encoded: &str) -> Result<Vec<u8>> {
    let encoded_bytes = hex::decode(encoded)?;
    let nonce: &[u8; 24] = &encoded_bytes[0..NONCE_LENGTH].try_into().unwrap();
    let encrypted = &encoded_bytes[NONCE_LENGTH..];
    let padding_password = password_to_key(password);
    let key = GenericArray::from_slice(&padding_password);
    let cipher = XSalsa20Poly1305::new(key);
    let nonce = GenericArray::from_slice(nonce);
    cipher
        .decrypt(nonce, encrypted.as_ref())
        .map_err(|_e| Error::PasswordIncorrect.into())
}

fn encrypt_content(password: &str, plaintext: &[u8]) -> Result<String> {
    let nonce_bytes = gen_nonce();
    let padding_password = password_to_key(password);
    let key = GenericArray::from_slice(&padding_password);
    let cipher = XSalsa20Poly1305::new(key);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let encoded = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_e| format_err!("{}", "encrypt error"))?;

    Ok(hex::encode([nonce_bytes.to_vec(), encoded].concat()))
}

fn gen_nonce() -> [u8; 24] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 24];

    for idx in 0..24 {
        nonce[idx] = rng.gen::<u8>()
    }
    nonce
}

fn password_to_key(password: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let password_bytes = password.as_bytes();
    let pwd_len = password_bytes.len();
    let iter_len = if pwd_len > 32 { 0 } else { pwd_len };
    for idx in 0..iter_len {
        key[idx] = password_bytes[idx]
    }
    key
}

pub fn decode_substrate_keystore(keystore: &SubstrateKeystore, password: &str) -> Result<Vec<u8>> {
    let (secret_key, pub_key) = keystore.decrypt(password)?;
    let priv_key = if secret_key.len() == 32 {
        Sr25519PrivateKey::from_seed(&secret_key)
    } else {
        Sr25519PrivateKey::from_slice(&secret_key)
    }?;
    if priv_key.public_key().to_bytes() != pub_key {
        return Err(Error::KeystorePublicKeyUnmatch.into());
    }
    Ok(secret_key)
}

pub fn encode_substrate_keystore(
    password: &str,
    prv_key: &[u8],
    coin: &CoinInfo,
) -> Result<SubstrateKeystore> {
    let pk = Sr25519PrivateKey::from_slice(prv_key)?;
    let pub_key = pk.public_key();
    let addr = SubstrateAddress::from_public_key(&TypedPublicKey::Sr25519(pub_key.clone()), &coin)?;
    SubstrateKeystore::new(password, prv_key, &pub_key.to_bytes(), &addr)
}

#[cfg(test)]
mod test_super {
    use super::*;
    use tcx_constants::{coin_info_from_param, TEST_PASSWORD};

    #[test]
    fn test_decrypt_encoded() {
        let encoded = "d80bcaf72c744d5a9a6c4229280e360d98d408afbe67232c3418a2a591b3f2bf468a319b7e5c1717bb8285619a76584a7961eac2183f94cfa56ad975cb78ae87b4dc18e7c20036bd448aa52c5ee7a45c4cdf41923c8133d6bfc29c737b65dcfb357884b55fb36d4762446fb26bfd8fce49142cf0e7d3642e2095ea6e425a8e923629306875c36b72a82d517478a19c8786b1be611e77286ba6448bf93c";
        let decrypted = decrypt_content("testing", encoded).unwrap();
        assert_eq!(hex::encode(decrypted), "3053020101300506032b657004220420416c696365202020202020202020202020202020202020202020202020202020d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4fa123032100d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4f");
    }

    const KEYSTORE_STR: &str = r#"{
                                  "address": "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS",
                                  "encoded": "0x3f6fb3c7a49b647d31999394a54f76ee451a23f46978bbca9e7d8e6a4d5d5c7457edaecf4108e2b77b157353f446d7c27bfc6e75032b60f28012f54495b9148ebe06ea5522f4d95e2a87d46eaae156372ae1111627f6d17a0b02830f7eb0f207061df299c730ea6e50bef02e3a218dbe29a0649769a7ad0a3ee1e10bc001e7b19a0d44b9a73e4889933635e4d1faa4da203955f6e29ec7b2df4fcf3f42",
                                  "encoding": {
                                    "content": [
                                      "pkcs8",
                                      "sr25519"
                                    ],
                                    "type": "xsalsa20-poly1305",
                                    "version": "2"
                                  },
                                  "meta": {
                                    "genesisHash": "0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                                    "name": "keystore_import_test",
                                    "tags": [],
                                    "whenCreated": 1593567896695
                                  }
                                }"#;

    #[test]
    fn test_decrypt_from_keystore() {
        let ks: SubstrateKeystore = serde_json::from_str(KEYSTORE_STR).unwrap();
        let decrypted = decode_substrate_keystore(&ks, TEST_PASSWORD).unwrap();
        assert_eq!(hex::encode(decrypted), "00ea01b0116da6ca425c477521fd49cc763988ac403ab560f4022936a18a4341016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd");
        let decrypted = decode_substrate_keystore(&ks, "wrong_password");
        assert_eq!(
            format!("{}", decrypted.err().unwrap()),
            "password_incorrect"
        );
    }

    #[test]
    fn test_export_from_sertcet_key() {
        let prv_key = hex::decode("00ea01b0116da6ca425c477521fd49cc763988ac403ab560f4022936a18a4341016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd").unwrap();
        let coin_info = coin_info_from_param("KUSAMA", "", "", "").unwrap();
        let keystore = encode_substrate_keystore(&TEST_PASSWORD, &prv_key, &coin_info).unwrap();
        assert_eq!(
            keystore.address,
            "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS"
        );
    }

    #[test]
    fn is_valid_keystore() {
        let keystore: SubstrateKeystore = serde_json::from_str(KEYSTORE_STR).unwrap();
        assert!(keystore.validate().is_ok())
    }
}
