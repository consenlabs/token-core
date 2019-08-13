use crate::types::Result;

pub mod secp256k1;

pub trait PublicKey: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_compressed(&self) -> Vec<u8>;
    fn to_uncompressed(&self) -> Vec<u8>;

    fn from_slice(data:&[u8]) -> Result<Self>;
}

pub trait PrivateKey {
    type PublicKey: PublicKey;

    fn public_key(&self) -> Self::PublicKey ;
}