use bitcoin_hashes::hex::{ToHex, FromHex};
use serde::{Deserialize, Serialize};

//use
//use super::super::utils::numberic_util;
//use super::encpair::EncPair;
//use crate::foundation::utils::token_error::TokenError;
use crate::aes;
use crate::numberic_util;
use crate::token_error::TokenError;

const CREDENTIAL_LEN: usize = 64usize;

//
pub type Credential = [u8; CREDENTIAL_LEN];


pub struct EncPair {
    pub enc_str: String,
    pub nonce: String
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct CipherParams {
    iv: String
}

pub trait KdfParams {
    fn validate();
    fn generate_derived_key(&self, password: &[u8], out: &mut [u8]);
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Pbkdf2Params {
    c: u32,
    prf: &'static str,
    dklen: u32,
    salt: String,
}

impl Pbkdf2Params {
    pub fn default() -> Pbkdf2Params {
        return Pbkdf2Params {
            c: 10240,
            prf: "hmac-sha256",
            dklen: 32,
            salt: String::from(""),
        };
    }
}


impl KdfParams for Pbkdf2Params {
    fn validate() {
        unimplemented!()
    }

    fn generate_derived_key(&self, password: &[u8], out: &mut [u8]) {
        let salt_bytes: Vec<u8> = FromHex::from_hex(&self.salt).unwrap();
        pbkdf2::pbkdf2::<hmac::Hmac<sha2::Sha256>>(password, &salt_bytes, self.c as usize, out);
    }
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Crypto<T: KdfParams> {
    cipher: &'static str,
    cipherparams: CipherParams,
    ciphertext: String,
    kdf: &'static str,
    kdfparams: T,
    mac: String,
}

impl Crypto<Pbkdf2Params> {
    pub fn new(password: &str, origin: &[u8]) -> Crypto<Pbkdf2Params> {
        let mut param = Pbkdf2Params::default();
        param.salt = numberic_util::random_iv(32).to_hex();
        let iv = numberic_util::random_iv(16);


        let mut crypto = Crypto {
            cipher: "aes-128-ctr",
            cipherparams: CipherParams { iv: iv.to_hex() },
            ciphertext: String::from(""),
            kdf: "pbkdf2",
            kdfparams: param,
            mac: String::from(""),
        };

        let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
        crypto.kdfparams.generate_derived_key(password.as_bytes(), &mut derived_key);

        let ciphertext = crypto.encrypt(password, origin);
        crypto.ciphertext = ciphertext.to_hex();
        let mac = Self::generate_mac(&derived_key, &ciphertext);
        crypto.mac = mac.to_hex();
        return crypto;
    }

    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>, TokenError> {
        let encrypted: Vec<u8> = FromHex::from_hex(&self.ciphertext).unwrap();
        let iv: Vec<u8> = FromHex::from_hex(&self.cipherparams.iv).unwrap();
        return self.decrypt_data(password, &encrypted, &iv);
    }


    fn encrypt(&self, password: &str, origin: &[u8]) -> Vec<u8> {
        let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
        self.kdfparams.generate_derived_key(password.as_bytes(), &mut derived_key);
        let key = &derived_key[0..16];
        let iv: Vec<u8> = FromHex::from_hex(&self.cipherparams.iv).unwrap();
        return super::aes::ctr::encrypt_nopadding(origin, key, &iv);
    }

    pub fn derive_enc_pair(&self, password: &str, origin: &[u8]) -> EncPair {
        let iv = numberic_util::random_iv(16);
        let encrypted_data = self.encrypt_data(password, origin, &iv);
        return EncPair {
            enc_str: encrypted_data.to_hex(),
            nonce: iv.to_hex(),
        };
    }

    pub fn decrypt_enc_pair(&self, password: &str, enc_pair: &EncPair) -> Result<Vec<u8>, TokenError> {
        let encrypted: Vec<u8> = FromHex::from_hex(&enc_pair.enc_str).unwrap();
        let iv: Vec<u8> = FromHex::from_hex(&enc_pair.nonce).unwrap();
        return self.decrypt_data(password, &encrypted, &iv);
    }

    fn encrypt_data(&self, password: &str, origin: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
        self.kdfparams.generate_derived_key(password.as_bytes(), &mut derived_key);
        let key = &derived_key[0..16];
        return super::aes::ctr::encrypt_nopadding(origin, key, &iv);
    }

    fn decrypt_data(&self, password: &str, encrypted: &[u8], iv: &[u8]) -> Result<Vec<u8>, TokenError> {
        let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
        self.kdfparams.generate_derived_key(password.as_bytes(), &mut derived_key);

        let mac = Self::generate_mac(&derived_key, encrypted);
        // todo return error when mac not match
//        if (self.mac != mac.to_hex()) {
//            return Error::fmt("Invalid Password");
//        }
        println!("mac: {:?}", mac.to_hex());

        let key = &derived_key[0..16];
        let ret = super::aes::ctr::decrypt_nopadding(encrypted, key, &iv);
        return Ok(ret);
    }

    fn generate_mac(derived_key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
        let result = [&derived_key[16..32], ciphertext].concat();
        let keccak256 = tiny_keccak::keccak256(&result);
        return keccak256.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

// Parse the string of data into serde_json::Value.
        let crypto: Crypto<Pbkdf2Params> = serde_json::from_str(data).unwrap();
//    crypto.kdfparams.generate_derived_key("Insecure Pa55w0rd".as_bytes());
// Access parts of the data by indexing with square brackets.
        crypto.decrypt("password");
        assert_eq!(crypto.mac, "4906577f075ad714f328e7b33829fdccfa8cd22eab2c0a8bc4f577824188ed16");
        assert_eq!(crypto.ciphertext, "17ff4858e697455f4966c6072473f3501534bc20deb339b58aeb8db0bd9fe91777148d0a909f679fb6e3a7a64609034afeb72a");
//        crypto.decrypt("")
    }
}