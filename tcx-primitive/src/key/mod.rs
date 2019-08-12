mod secp256k1;
mod derive;

use core::result::Result;
use std::str::FromStr;

/// An identifier for a type of cryptographic key.
///
/// 0..=2048 is
pub type KeyTypeId = u32;

pub mod key_types {
    use super::KeyTypeId;

    pub const SECP256K1: KeyTypeId = 10;
}

#[allow(dead_code)]
pub enum KeyError {
    InvalidEcdsa,
    OverflowChildNumber,
    InvalidDerivationPathFormat,
    InvalidKeyLength,
    InvalidSignature,
    InvalidSignatureLength,
    InvalidChildNumber,
    CannotDeriveFromHardenedKey,
    InvalidBase58,
    InvalidPrivateKey,
    InvalidPublicKey,
    Unknown,
}

pub trait TypedKey {
    const KEY_TYPE: KeyTypeId;
}

pub trait Public: AsRef<[u8]> + TypedKey + Sized + FromStr {
    type Error;

    fn from_slice(data: &[u8]) -> Result<Self, Self::Error>;

    fn as_slice(&self) -> &[u8];
}

pub trait Pair: TypedKey + Sized + FromStr {
    type Public: Public;

    type Seed: Default + AsRef<[u8]> + AsMut<[u8]> + Clone;

    type Error;

    fn from_slice(data: &[u8]) -> Result<Self, Self::Error>;

    fn from_seed(seed: &Self::Seed) -> Result<Self, Self::Error>;

    fn from_seed_slice(seed: &[u8]) -> Result<Self, Self::Error>;

    fn public(&self) -> Self::Public;
}



