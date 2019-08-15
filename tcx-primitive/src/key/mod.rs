pub mod derive;
pub mod secp256k1;

use crate::Error;

use ::secp256k1::{RecoverableSignature, Signature};
use core::result::Result;
use std::str::FromStr;

use crate::key::derive::Ss58Codec;
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
    InvalidBase58,
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidMessage,
    InvalidRecoveryId,
    InvalidTweak,
    Unknown,
}

pub trait EcdsaSigner {
    type Error;

    fn sign(&self, data: &[u8]) -> Result<Signature, Self::Error>;
}

pub trait EcdsaRecoverableSigner {
    type Error;

    fn sign(&self, data: &[u8]) -> Result<RecoverableSignature, Self::Error>;
}

pub trait TypedKey {
    const KEY_TYPE: KeyTypeId;
}

pub trait Public: AsRef<[u8]> + TypedKey + Sized + FromStr + Derive + Ss58Codec {
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
