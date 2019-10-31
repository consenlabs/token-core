use crate::numberic_util;
use crate::Error;
use crate::Result;
use bitcoin_hashes::hex::{FromHex, ToHex};
use digest::Digest;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const CREDENTIAL_LEN: usize = 64usize;

pub type Credential = [u8; CREDENTIAL_LEN];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncPair {
    pub enc_str: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CipherParams {
    iv: String,
}

pub trait KdfParams {
    fn validate(&self) -> Result<()>;
    fn generate_derived_key(&self, password: &[u8], out: &mut [u8]);
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pbkdf2Params {
    c: u32,
    prf: String,
    dklen: u32,
    salt: String,
}

impl Pbkdf2Params {
    pub fn default() -> Pbkdf2Params {
        Pbkdf2Params {
            c: 10240,
            prf: "hmac-sha256".to_owned(),
            dklen: 32,
            salt: "".to_owned(),
        }
    }
}

impl KdfParams for Pbkdf2Params {
    fn validate(&self) -> Result<()> {
        if self.dklen == 0 || self.c == 0 || self.salt.is_empty() || self.prf.is_empty() {
            Err(Error::KdfParamsInvalid.into())
        } else {
            Ok(())
        }
    }

    fn generate_derived_key(&self, password: &[u8], out: &mut [u8]) {
        let salt_bytes: Vec<u8> = FromHex::from_hex(&self.salt).unwrap();
        pbkdf2::pbkdf2::<hmac::Hmac<sha2::Sha256>>(password, &salt_bytes, self.c as usize, out);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CacheDerivedKey {
    hashed_key: String,
    derived_key: Vec<u8>,
}

impl CacheDerivedKey {
    pub fn new(key: &str, derived_key: &[u8]) -> CacheDerivedKey {
        CacheDerivedKey {
            hashed_key: Self::hash(key),
            derived_key: derived_key.to_vec(),
        }
    }

    fn hash(key: &str) -> String {
        let key_data: Vec<u8> = FromHex::from_hex(key).expect("cache derived key from key");
        Sha256::digest(&Sha256::digest(&key_data)).to_hex()
    }

    pub fn get_derived_key(&self, key: &str) -> Result<Vec<u8>> {
        if self.hashed_key == Self::hash(key) {
            Ok(self.derived_key.clone())
        } else {
            Err(Error::PasswordIncorrect.into())
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crypto<T: KdfParams> {
    cipher: String,
    cipherparams: CipherParams,
    ciphertext: String,
    kdf: String,
    kdfparams: T,
    mac: String,
    #[serde(skip)]
    cached_derived_key: Option<CacheDerivedKey>,
}

impl Crypto<Pbkdf2Params> {
    pub fn new(password: &str, origin: &[u8]) -> Crypto<Pbkdf2Params> {
        let mut param = Pbkdf2Params::default();
        param.salt = numberic_util::random_iv(32).to_hex();
        let iv = numberic_util::random_iv(16);

        let mut crypto = Crypto {
            cipher: "aes-128-ctr".to_owned(),
            cipherparams: CipherParams { iv: iv.to_hex() },
            ciphertext: String::from(""),
            kdf: "pbkdf2".to_owned(),
            kdfparams: param,
            mac: String::from(""),
            cached_derived_key: None,
        };

        let derived_key = crypto
            .generate_derived_key(password)
            .expect("new crypto generate_derived_key");

        let ciphertext = crypto.encrypt(password, origin);
        crypto.ciphertext = ciphertext.to_hex();
        let mac = Self::generate_mac(&derived_key, &ciphertext);
        crypto.mac = mac.to_hex();
        crypto
    }

    pub fn generate_derived_key(&self, key: &str) -> Result<Vec<u8>> {
        if let Some(ckd) = &self.cached_derived_key {
            ckd.get_derived_key(key)
        } else {
            let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
            self.kdfparams
                .generate_derived_key(key.as_bytes(), &mut derived_key);
            if self.mac != "" && !self.verify_derived_key(&derived_key) {
                return Err(Error::PasswordIncorrect.into());
            }
            Ok(derived_key.to_vec())
        }
    }

    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>> {
        let encrypted: Vec<u8> = FromHex::from_hex(&self.ciphertext).expect("ciphertext");
        let iv: Vec<u8> = FromHex::from_hex(&self.cipherparams.iv).expect("iv");
        self.decrypt_data(password, &encrypted, &iv)
    }

    fn encrypt(&self, password: &str, origin: &[u8]) -> Vec<u8> {
        let derived_key = self
            .generate_derived_key(password)
            .expect("crypto::encrypt must no error");
        let key = &derived_key[0..16];
        let iv: Vec<u8> = FromHex::from_hex(&self.cipherparams.iv).unwrap();
        super::aes::ctr::encrypt_nopadding(origin, key, &iv)
            .expect("encrypt_nopadding key or iv's length must be 16")
    }

    pub fn derive_enc_pair(&self, password: &str, origin: &[u8]) -> Result<EncPair> {
        let iv = numberic_util::random_iv(16);
        let encrypted_data = self.encrypt_data(password, origin, &iv)?;
        Ok(EncPair {
            enc_str: encrypted_data.to_hex(),
            nonce: iv.to_hex(),
        })
    }

    pub fn decrypt_enc_pair(&self, password: &str, enc_pair: &EncPair) -> Result<Vec<u8>> {
        let encrypted: Vec<u8> = FromHex::from_hex(&enc_pair.enc_str).unwrap();
        let iv: Vec<u8> = FromHex::from_hex(&enc_pair.nonce).unwrap();
        self.decrypt_data(password, &encrypted, &iv)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let derived_key_ret = self.generate_derived_key(password);

        derived_key_ret.is_ok() && self.verify_derived_key(&derived_key_ret.expect(""))
    }

    fn encrypt_data(&self, password: &str, origin: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let derived_key = self.generate_derived_key(password)?;

        if !self.verify_derived_key(&derived_key) {
            return Err(Error::PasswordIncorrect.into());
        }

        let key = &derived_key[0..16];
        super::aes::ctr::encrypt_nopadding(origin, key, &iv)
    }

    fn decrypt_data(&self, password: &str, encrypted: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let derived_key = self.generate_derived_key(password)?;

        if !self.verify_derived_key(&derived_key) {
            return Err(Error::PasswordIncorrect.into());
        }

        let key = &derived_key[0..16];
        super::aes::ctr::decrypt_nopadding(encrypted, key, &iv)
    }

    pub fn verify_derived_key(&self, dk: &[u8]) -> bool {
        let cipher_bytes = Vec::from_hex(&self.ciphertext).expect("vec::from_hex");
        let mac = Self::generate_mac(&dk, &cipher_bytes);
        self.mac == mac.to_hex()
    }

    fn generate_mac(derived_key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
        let result = [&derived_key[16..32], ciphertext].concat();
        let keccak256 = tiny_keccak::keccak256(&result);
        keccak256.to_vec()
    }

    pub fn cache_derived_key(&mut self, key: &str, derived_key: &[u8]) {
        let cdk = CacheDerivedKey::new(key, derived_key);
        self.cached_derived_key = Some(cdk);
    }

    pub fn clear_cache_derived_key(&mut self) {
        self.cached_derived_key = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSWORD: &str = "Insecure Password";
    #[test]
    pub fn pbkdf2_params_default_test() {
        let param = Pbkdf2Params::default();
        let default = Pbkdf2Params {
            c: 10240,
            prf: "hmac-sha256".to_owned(),
            dklen: 32,
            salt: "".to_owned(),
        };
        assert_eq!(default, param);
    }

    #[test]
    pub fn new_crypto() {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(PASSWORD, "TokenCoreX".as_bytes());
        assert_ne!("", crypto.ciphertext);
        assert_ne!("", crypto.cipher);
        assert_ne!("", crypto.mac);
        assert_ne!("", crypto.cipherparams.iv);
        assert_ne!("", crypto.kdfparams.salt);
        assert_eq!("pbkdf2", crypto.kdf)
    }

    #[test]
    pub fn decrypt_crypto() {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(PASSWORD, "TokenCoreX".as_bytes());
        let cipher_bytes = crypto.decrypt(PASSWORD).expect("cipher bytes");
        assert_eq!("TokenCoreX", String::from_utf8(cipher_bytes).unwrap());

        let ret = crypto.decrypt("WrongPassword");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(
            Error::PasswordIncorrect,
            err.downcast::<crate::Error>().unwrap()
        );
    }

    #[test]
    pub fn enc_pair_test() {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(PASSWORD, "TokenCoreX".as_bytes());
        let enc_pair = crypto
            .derive_enc_pair(PASSWORD, "TokenCoreX".as_bytes())
            .unwrap();

        assert_ne!("", enc_pair.nonce);
        assert_ne!("", enc_pair.enc_str);

        let decrypted_bytes = crypto.decrypt_enc_pair(PASSWORD, &enc_pair).unwrap();
        let decrypted = String::from_utf8(decrypted_bytes).unwrap();

        assert_eq!("TokenCoreX", decrypted);

        let ret = crypto.decrypt_enc_pair("WrongPassword", &enc_pair);
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(
            Error::PasswordIncorrect,
            err.downcast::<crate::Error>().unwrap()
        );

        let ret = crypto.derive_enc_pair("WrongPassword", &hex!("01020304"));
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(
            Error::PasswordIncorrect,
            err.downcast::<crate::Error>().unwrap()
        );
    }

    #[test]
    pub fn kdfparams_trait_validate_test() {
        let err = Pbkdf2Params::default().validate().err().unwrap();
        assert_eq!(
            Error::KdfParamsInvalid,
            err.downcast::<crate::Error>().unwrap()
        )
    }

    #[test]
    pub fn generate_derived_key_pbkdf2_test() {
        let mut pbkdf2_param = Pbkdf2Params::default();
        pbkdf2_param.salt = "01020304010203040102030401020304".to_string();
        let mut derived_key = [0; CREDENTIAL_LEN];
        pbkdf2_param.generate_derived_key(PASSWORD.as_bytes(), &mut derived_key);
        let dk_hex = derived_key.to_hex();
        assert_eq!("5c8764983679d5b1362ef992f764e772b84901060dcf2077c41d336feb29c8afcaf05e9e8be6f8e420b9b662411e5b7ba78541bcdd898683ccf686b424aa7951", dk_hex);
    }

    #[test]
    pub fn json_serial() {
        let data = r#"
    {
    "cipher": "aes-128-ctr",
    "cipherparams": {
      "iv": "2cb9d4457b284e47877d08a5c9493b46"
    },
    "ciphertext": "17ff4858e697455f4966c6072473f3501534bc20deb339b58aeb8db0bd9fe91777148d0a909f679fb6e3a7a64609034afeb72a",
    "kdf": "pbkdf2",
    "kdfparams": {
      "c": 10240,
      "dklen": 32,
      "prf": "hmac-sha256",
      "salt": "37890eb305866aa07853d14e7666c2ed31e18efc1129f1c5a66b9cc93d03fd73"
    },
    "mac": "4906577f075ad714f328e7b33829fdccfa8cd22eab2c0a8bc4f577824188ed16"
  }"#;

        let crypto: Crypto<Pbkdf2Params> = serde_json::from_str(data).unwrap();
        let result = crypto.decrypt("aaa");
        assert!(result.is_err());
        assert_eq!(
            crypto.mac,
            "4906577f075ad714f328e7b33829fdccfa8cd22eab2c0a8bc4f577824188ed16"
        );
        assert_eq!(crypto.ciphertext, "17ff4858e697455f4966c6072473f3501534bc20deb339b58aeb8db0bd9fe91777148d0a909f679fb6e3a7a64609034afeb72a");
    }
}
