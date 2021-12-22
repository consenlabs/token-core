use crate::SubstrateAddress;
use rand::Rng;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::convert::TryInto;
use tcx_chain::Address;

use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use regex::Regex;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, marker::PhantomData};
use tcx_constants::{CoinInfo, Result};
use tcx_crypto::numberic_util::random_iv;
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
    #[serde(rename = "type", deserialize_with = "string_or_seq_string")]
    pub encoding_type: Vec<String>,
    pub version: String,
}

fn string_or_seq_string<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> std::result::Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
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
const SALT_LENGTH: usize = 32;
const SCRYPT_LENGTH: usize = SALT_LENGTH + (3 * 4);
const PJS_SCRYPT_N: u32 = 1 << 15;
const PJS_SCRYPT_P: u32 = 1;
const PJS_SCRYPT_R: u32 = 8;

fn u32_to_bytes(num: u32) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u32::<LittleEndian>(num)
        .expect("u32_to_bytes error");
    wtr
}

fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut rdr = Cursor::new(bytes);
    rdr.read_u32::<LittleEndian>().unwrap()
}

fn scrypt_param_from_encoded(encoded: &[u8]) -> Result<(scrypt::ScryptParams, Vec<u8>)> {
    let salt = &encoded[0..SALT_LENGTH];
    let n = bytes_to_u32(&encoded[SALT_LENGTH..SALT_LENGTH + 4]);
    let p = bytes_to_u32(&encoded[SALT_LENGTH + 4..SALT_LENGTH + 4 * 2]);
    let r = bytes_to_u32(&encoded[SALT_LENGTH + 4 * 2..SALT_LENGTH + 4 * 3]);
    let log_n = (n as f64).log2().round();

    let inner_params = scrypt::ScryptParams::new(log_n as u8, r, p).expect("init scrypt params");
    if n != PJS_SCRYPT_N || p != PJS_SCRYPT_P || r != PJS_SCRYPT_R {
        Err(format_err!("Pjs keystore invalid params"))
    } else {
        Ok((inner_params, salt.to_vec()))
    }
}

fn default_scrypt_param() -> (scrypt::ScryptParams, Vec<u8>) {
    let log_n = ((1 << 15) as f64).log2().round();
    let param = scrypt::ScryptParams::new(log_n as u8, 8, 1).expect("init scrypt params");
    let salt = random_iv(32);
    (param, salt)
}

impl SubstrateKeystore {
    pub fn new(
        password: &str,
        prv_key: &[u8],
        pub_key: &[u8],
        addr: &str,
    ) -> Result<SubstrateKeystore> {
        let encoding = SubstrateKeystore::encrypt(password, prv_key, pub_key)?;

        Ok(SubstrateKeystore {
            address: addr.to_string(),
            encoded: encoding.to_string(),
            encoding: SubstrateKeystoreEncoding {
                content: vec!["pkcs8".to_string(), "sr25519".to_string()],
                encoding_type: vec!["scrypt".to_string(), "xsalsa20-poly1305".to_string()],
                version: "3".to_string(),
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
        if self
            .encoding
            .encoding_type
            .iter()
            .all(|x| x != &"xsalsa20-poly1305".to_owned())
        {
            return Err(
                Error::InvalidKeystore("only support xsalsa20-poly1305".to_string()).into(),
            );
        }
        if self.encoding.version != "2" && self.encoding.version != "3" {
            return Err(Error::InvalidKeystore("only support version 2 or 3".to_string()).into());
        }

        Ok(())
    }

    fn decode_cipher_text(&self) -> Result<Vec<u8>> {
        let hex_re = Regex::new(r"^(?:0[xX])?[0-9a-fA-F]+$").unwrap();
        if self.encoding.version == "3" {
            if !hex_re.is_match(&self.encoded) {
                return base64::decode(&self.encoded)
                    .map_err(|_| format_err!("decode_cipher_text"));
            }
        }

        if self.encoded.starts_with("0x") {
            return hex::decode(&self.encoded[2..])
                .map_err(|_| format_err!("decode_cipher_text decode hex"));
        } else {
            return hex::decode(&self.encoded)
                .map_err(|_| format_err!("decode_cipher_text decode hex"));
        }
    }

    pub fn decrypt(&self, password: &str) -> Result<(Vec<u8>, Vec<u8>)> {
        let _ = self.validate()?;
        let mut encoded = self.decode_cipher_text()?;

        let password_bytes = if self.encoding.version == "3"
            && self.encoding.encoding_type.iter().any(|x| x == "scrypt")
        {
            let (params, salt) = scrypt_param_from_encoded(&encoded)?;

            let mut out = [0u8; 64];
            scrypt::scrypt(password.as_bytes(), &salt, &params, &mut out)
                .expect("can not execute scrypt");
            encoded = encoded[SCRYPT_LENGTH..].to_vec();
            out.to_vec()
        } else {
            password.as_bytes().to_vec()
        };
        let decrypted = decrypt_content(&password_bytes, &encoded)?;
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

fn decrypt_content(password: &[u8], encoded_bytes: &[u8]) -> Result<Vec<u8>> {
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
    let mut out = [0u8; 64];
    let (param, salt) = default_scrypt_param();
    scrypt::scrypt(password.as_bytes(), &salt, &param, &mut out).expect("can not execute scrypt");
    let padding_password = password_to_key(&out);
    let key = GenericArray::from_slice(&padding_password);
    let cipher = XSalsa20Poly1305::new(key);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let encoded = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_e| format_err!("{}", "encrypt error"))?;

    let scrypt_params_encoded = [
        salt,
        u32_to_bytes(1 << 15),
        u32_to_bytes(1),
        u32_to_bytes(8),
    ]
    .concat();
    let complete_encoded: Vec<u8> = [scrypt_params_encoded, nonce_bytes.to_vec(), encoded].concat();
    Ok(base64::encode(&complete_encoded))
}

fn gen_nonce() -> [u8; 24] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 24];

    for idx in 0..24 {
        nonce[idx] = rng.gen::<u8>()
    }
    nonce
}

fn password_to_key(password_bytes: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let pwd_len = password_bytes.len();
    let iter_len = if pwd_len > 32 { 32 } else { pwd_len };
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
        let encoded_bytes = hex::decode(encoded).expect("encoded_bytes");
        let decrypted = decrypt_content("testing".as_bytes(), &encoded_bytes).unwrap();
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
    fn test_decrypt_from_keystore_v2() {
        let ks: SubstrateKeystore = serde_json::from_str(KEYSTORE_STR).unwrap();
        let decrypted = decode_substrate_keystore(&ks, TEST_PASSWORD).unwrap();
        assert_eq!(hex::encode(decrypted), "00ea01b0116da6ca425c477521fd49cc763988ac403ab560f4022936a18a4341016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd");
        let decrypted = decode_substrate_keystore(&ks, "wrong_password");
        assert_eq!(
            format!("{}", decrypted.err().unwrap()),
            "password_incorrect"
        );
    }

    const KEYSTORE_STR_V3: &str = r#"{
                                      "address": "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS",
                                      "encoded": "DoCvvaXYkyi8zrm/ZXQoT4HQD+ScaP8fY2GeDSsXfZUAgAAAAQAAAAgAAACYwzCG5dX8jyj7/gAHI9ZK2NoZsEp6wHGtVTrhZErbmVTlajvbggvtI1YkgTFEUGrr4nbIAbOheGtPPvu009YPDrv7akowGP+nAqHuQV36ZQFdf5/2ns4SbIulemweJN3L2caMrQI2iGtx7y+vFD63tYVaYuMY1vcOANTYRZsXOTUuIpRlrWhUZTTOKQcewYkOaYXbC953O3y3oZA5",
                                      "encoding": {
                                        "content": [
                                          "pkcs8",
                                          "sr25519"
                                        ],
                                        "type": [
                                          "scrypt",
                                          "xsalsa20-poly1305"
                                        ],
                                        "version": "3"
                                      },
                                      "meta": {
                                        "genesisHash": "0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                                        "name": "V3Test",
                                        "tags": [],
                                        "whenCreated": 1595895332579
                                      }
                                    }"#;

    #[test]
    fn test_decrypt_from_keystore_v3() {
        let ks: SubstrateKeystore = serde_json::from_str(KEYSTORE_STR_V3).unwrap();
        assert_eq!(ks.encoding.encoding_type.len(), 2);
        assert_eq!(ks.encoding.encoding_type[0], "scrypt");
        let decrypted = decode_substrate_keystore(&ks, &TEST_PASSWORD).unwrap();
        assert_eq!(hex::encode(decrypted), "00ea01b0116da6ca425c477521fd49cc763988ac403ab560f4022936a18a4341016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd");
        let decrypted = decode_substrate_keystore(&ks, "wrong_password");
        assert_eq!(
            format!("{}", decrypted.err().unwrap()),
            "password_incorrect"
        );
    }

    #[test]
    fn test_decrypt_from_keystore_v3_hex() {
        let keystore_str_v3_hex = r#"{
                                          "address": "FLiSDPCcJ6auZUGXALLj6jpahcP6adVFDBUQznPXUQ7yoqH",
                                          "encoded": "0xcd238963070cc4d6806053ee1ac500c7add9c28732bb5d434a332f84a91d9be0008000000100000008000000cf630a1113941b350ddd06697e50399183162e5e9a0e893eafc7f5f4893a223dca5055706b9925b56fdb4304192143843da718e11717daf89cf4f4781f94fb443f61432f782d54280af9eec90bd3069c3cc2d957a42b7c18dc2e9497f623735518e0e49b58f8e4db2c09da3a45dbb935659d015fc94b946cba75b606a6ff7f4e823f6b049e2e6892026b49de02d6dbbd64646fe0933f537d9ea53a70be",
                                          "encoding": {
                                            "content": [
                                              "pkcs8",
                                              "sr25519"
                                            ],
                                            "type": [
                                              "scrypt",
                                              "xsalsa20-poly1305"
                                            ],
                                            "version": "3"
                                          },
                                          "meta": {
                                            "genesisHash": "0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                                            "name": "version3",
                                            "tags": [],
                                            "whenCreated": 1595277797639
                                          }
                                        }"#;
        let ks: SubstrateKeystore = serde_json::from_str(keystore_str_v3_hex).unwrap();
        let decrypted = decode_substrate_keystore(&ks, "version3").unwrap();
        assert_eq!(hex::encode(decrypted), "50f027d401a8572ff06381cf9dc633d08bd46af0d8bd6ded4ffdb4c706afdd669c80475085d55140552e00ff1fabb70be0d67c0db540c23549922b4edb4e4add");
        let decrypted = decode_substrate_keystore(&ks, "wrong_password");
        assert_eq!(
            format!("{}", decrypted.err().unwrap()),
            "password_incorrect"
        );
    }

    #[test]
    fn test_export_from_secret_key() {
        let prv_key = hex::decode("00ea01b0116da6ca425c477521fd49cc763988ac403ab560f4022936a18a4341016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd").unwrap();
        let coin_info = coin_info_from_param("KUSAMA", "", "", "").unwrap();
        let keystore = encode_substrate_keystore(&TEST_PASSWORD, &prv_key, &coin_info).unwrap();
        assert_eq!(
            keystore.address,
            "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS"
        );
        assert_eq!(keystore.encoding.version, "3");
        assert!(keystore
            .encoding
            .encoding_type
            .iter()
            .any(|x| x == "scrypt"));
        assert!(base64::decode(&keystore.encoded).is_ok());
    }

    #[test]
    fn is_valid_keystore() {
        let keystore: SubstrateKeystore = serde_json::from_str(KEYSTORE_STR).unwrap();
        assert!(keystore.validate().is_ok())
    }
}
