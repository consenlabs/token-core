use secp256k1::{Secp256k1, Message, SecretKey};
use bitcoin::{PrivateKey};
use crate::error::Result;

pub trait Curve {
    fn sign(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn sign_canonical(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn sign_der(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn sign_schnorr(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn verify(signature: &[u8], msg: &[u8]) -> bool {
        unimplemented!();
    }
    fn verify_schnorr(signature: &[u8], msg: &[u8]) ->bool {
        unimplemented!();
    }

    fn public_key(prv_key: &[u8]) -> String {
        unimplemented!();
    }
    fn compressed_public_key(prv_key: &[u8]) -> String {
        unimplemented!();
    }
}

pub struct Secp256k1Curve {

}

impl Curve for Secp256k1Curve {
    fn sign(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let msg = Message::from_slice(bytes)?;
        let key = SecretKey::from_slice(pk)?;
        let signature = s.sign(&msg, &key);
        Ok(signature.serialize_compact().to_vec())
    }

    
}

