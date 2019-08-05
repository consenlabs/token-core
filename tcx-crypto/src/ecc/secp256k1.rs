use crate::types::{Result, Error};
use super::{PublicKey, PrivateKey};

impl PublicKey for bitcoin::PublicKey {
    fn to_bytes(&self) -> Vec<u8> {
        bitcoin::PublicKey::to_bytes(self)
    }

    fn to_compressed(&self) -> Vec<u8> {
        bitcoin::PublicKey::to_compressed(self)
    }

    fn to_uncompressed(&self) -> Vec<u8> {
        bitcoin::PublicKey::to_uncompressed(self)
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

