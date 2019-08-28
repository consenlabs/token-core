mod derive;
pub mod secp256k1;

use crate::Error;

use ::secp256k1::{recovery::RecoverableSignature, Signature};
use core::result::Result;
use std::str::FromStr;

pub use derive::{Derive, DeriveJunction, DerivePath};

/// An identifier for a type of cryptographic key.
///
pub type KeyTypeId = u32;

pub mod key_types {
    use super::KeyTypeId;

    pub const SECP256K1: KeyTypeId = 10;
}

pub mod key_errors {}

#[allow(dead_code)]
#[derive(Debug)]
pub enum KeyError {
    InvalidEcdsa,
    InvalidChildNumberFormat,
    OverflowChildNumber,
    InvalidDerivationPathFormat,
    InvalidKeyLength,
    InvalidSignature,
    InvalidSignatureLength,
    InvalidChildNumber,
    CannotDeriveFromHardenedKey,
    CannotDeriveKey,
    InvalidBase58,
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidMessage,
    InvalidRecoveryId,
    InvalidTweak,
    NotEnoughMemory,
    Unknown,
}

pub trait Signer<U> {
    type Error;

    fn sign<T: AsRef<[u8]>>(&self, data: T) -> Result<U, Self::Error>;
}

pub trait TypedKey {
    const KEY_TYPE: KeyTypeId;
}

pub trait Public: AsRef<[u8]> + TypedKey + Sized + FromStr + Derive {
    fn from_slice(data: &[u8]) -> Result<Self, Self::Error>;

    fn as_slice(&self) -> &[u8];
}

pub trait Pair: TypedKey + Sized + FromStr + Derive {
    type Public: Public;

    type Seed: Default + AsRef<[u8]> + AsMut<[u8]> + Clone;

    fn from_slice(data: &[u8]) -> Result<Self, Self::Error>;

    fn from_seed(seed: &Self::Seed) -> Result<Self, Self::Error>;

    fn from_seed_slice(seed: &[u8]) -> Result<Self, Self::Error>;

    fn public(&self) -> Self::Public;
}
