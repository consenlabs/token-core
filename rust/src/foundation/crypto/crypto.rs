use bitcoin::util::address::Address;
use bitcoin::PrivateKey;
use bip39::{Mnemonic, Language};
use std::str::FromStr;
use bitcoin_hashes::hex::{ToHex, FromHex};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult, RefReadBuffer, RefWriteBuffer};
use crypto::aes::KeySize;
use crypto::symmetriccipher::Decryptor;

use super::super::utils::numberic_util;
use super::encpair::EncPair;
use crate::foundation::utils::token_error::TokenError;


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


static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN * 2;

pub type Credential = [u8; CREDENTIAL_LEN];

impl KdfParams for Pbkdf2Params {
    fn validate() {
        unimplemented!()
    }

    fn generate_derived_key(&self, password: &[u8], out: &mut [u8]) {
        let c_nzu = NonZeroU32::new(self.c).unwrap();
        let salt_bytes: Vec<u8> = FromHex::from_hex(&self.salt).unwrap();
        pbkdf2::derive(DIGEST_ALG, c_nzu, &salt_bytes, password, out);
        println!("derived key: {}", out.to_hex());
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
        let mut mac = [0u8; 32];
        Self::generate_mac(&derived_key, &ciphertext, &mut mac);
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
            nonce: iv.to_hex()
        }
    }

    pub fn decrypt_enc_pair(&self, password: &str, enc_pair: &EncPair) -> Result<Vec<u8>, TokenError> {
        let encrypted : Vec<u8>= FromHex::from_hex(&enc_pair.enc_str).unwrap();
        let iv: Vec<u8> = FromHex::from_hex(&enc_pair.nonce).unwrap();
        return self.decrypt_data(password, &encrypted, &iv);
    }

    fn encrypt_data(&self, password: &str, origin: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
        self.kdfparams.generate_derived_key(password.as_bytes(), &mut derived_key);
        let key = &derived_key[0..16];
//        let iv: Vec<u8> = FromHex::from_hex(&self.cipherparams.iv).unwrap();
        return super::aes::ctr::encrypt_nopadding(origin, key, &iv);
    }

    fn decrypt_data(&self, password: &str, encrypted: &[u8], iv: &[u8]) -> Result<Vec<u8>, TokenError> {
        let mut derived_key: Credential = [0u8; CREDENTIAL_LEN];
        self.kdfparams.generate_derived_key(password.as_bytes(), &mut derived_key);

        let mut mac = [0u8; 32];
        Self::generate_mac(&derived_key, encrypted, &mut mac);
        println!("mac: {}", mac.to_hex());
        // todo return error when mac not match
//        if (self.mac != mac.to_hex()) {
//            return Error::fmt("Invalid Password");
//        }

        let key = &derived_key[0..16];
        let iv: Vec<u8> = FromHex::from_hex(&self.cipherparams.iv).unwrap();
        let ret = super::aes::ctr::decrypt_nopadding(encrypted, key, &iv);
        println!("decrypted result: {:?}", ret);
        return Ok(ret);
    }

    fn generate_mac(derived_key: &[u8], ciphertext: &[u8], out: &mut [u8; 32]) {
        let result = [&derived_key[16..32], ciphertext].concat();
        let mut keccak256 = Sha3::keccak256();
        keccak256.input(&result);
        keccak256.result(out);
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

//        assert_eq!("", crypto);
    }
}