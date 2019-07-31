use crate::types::{Result, Error};
use super::{PublicKey, PrivateKey};

impl PublicKey for bitcoin::PublicKey {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    fn to_compressed(&self) -> Vec<u8> {
        self.to_compressed()
    }

    fn to_uncompressed(&self) -> Vec<u8> {
        self.to_uncompressed()
    }

    fn from_slice(data: &[u8]) -> Result<bitcoin::PublicKey> {
        if let Ok(key) = bitcoin::PublicKey::from_slice(data) {
            Ok(key)
        } else {
            Err(Error::InvalidPublicKey)
        }
    }
}

impl PrivateKey for bitcoin::PrivateKey {
    type PublicKey = bitcoin::PublicKey;

    fn public_key(&self) -> Self::PublicKey {
        self.public_key(&secp256k1::Secp256k1::new())
    }
}

