use crate::ecc::{KeyError, PrivateKey as TraitPrivateKey, PublicKey as TraitPublicKey};
use crate::Result;
use bls_signatures::{PrivateKey, PublicKey, Serialize};

#[derive(Clone)]
pub struct BLSPublicKey(PublicKey);

#[derive(Clone)]
pub struct BLSPrivateKey(PrivateKey);

impl From<PublicKey> for BLSPublicKey {
    fn from(pk: PublicKey) -> Self {
        BLSPublicKey(pk)
    }
}

impl From<PrivateKey> for BLSPrivateKey {
    fn from(sk: PrivateKey) -> Self {
        BLSPrivateKey(sk)
    }
}

impl TraitPrivateKey for BLSPrivateKey {
    type PublicKey = BLSPublicKey;

    fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(BLSPrivateKey(PrivateKey::from_bytes(data)?))
    }

    fn public_key(&self) -> Self::PublicKey {
        BLSPublicKey(self.0.public_key())
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(message).as_bytes())
    }

    fn sign_recoverable(&self, _: &[u8]) -> Result<Vec<u8>> {
        Err(KeyError::NotImplement.into())
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }
}

impl TraitPublicKey for BLSPublicKey {
    fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(BLSPublicKey(PublicKey::from_bytes(data)?))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_private_key() {}

    #[test]
    fn test_public_key() {}
}
