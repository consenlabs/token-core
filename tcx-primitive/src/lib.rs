#[macro_use]
extern crate failure;

pub mod derive;
mod error;
mod secp256k1;

use core::result;
use serde::{Deserialize, Serialize};

pub use error::Error;

pub type Result<T> = result::Result<T, failure::Error>;

pub use crate::secp256k1::{
    ArbitraryNetworkExtendedPrivKey, ArbitraryNetworkExtendedPubKey, Pair as Secp256k1Pair,
    Public as Secp256k1PublicKey,
};

pub use derive::{Derive, DeriveJunction, DerivePath};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CurveType {
    SECP256k1,          /* "secp256k1" */
    ED25519,            /* "ed25519" */
    ED25519Blake2bNano, /* "ed25519-blake2b-nano" */
    Curve25519,         /* "curve25519" */
    NIST256p1,
}

#[derive(Fail, Debug, PartialEq)]
pub enum KeyError {
    #[fail(display = "invalid_ecdsa")]
    InvalidEcdsa,
    #[fail(display = "invalid_child_number_format")]
    InvalidChildNumberFormat,
    #[fail(display = "overflow_child_number")]
    OverflowChildNumber,
    #[fail(display = "invalid_derivation_path_format")]
    InvalidDerivationPathFormat,
    #[fail(display = "invalid_key_length")]
    InvalidKeyLength,
    #[fail(display = "invalid_signature")]
    InvalidSignature,
    #[fail(display = "invalid_signature_length")]
    InvalidSignatureLength,
    #[fail(display = "invalid_child_number")]
    InvalidChildNumber,
    #[fail(display = "cannot_derive_from_hardened_key")]
    CannotDeriveFromHardenedKey,
    #[fail(display = "cannot_derive_key")]
    CannotDeriveKey,
    #[fail(display = "invalid_base58")]
    InvalidBase58,
    #[fail(display = "invalid_private_key")]
    InvalidPrivateKey,
    #[fail(display = "invalid_public_key")]
    InvalidPublicKey,
    #[fail(display = "invalid_message")]
    InvalidMessage,
    #[fail(display = "invalid_recovery_id")]
    InvalidRecoveryId,
    #[fail(display = "invalid_tweak")]
    InvalidTweak,
    #[fail(display = "not_enough_memory")]
    NotEnoughMemory,
    #[fail(display = "unknown")]
    Unknown,
}

/// An identifier for a type of cryptographic key.
///
pub type KeyTypeId = u32;

pub mod key_types {
    use super::KeyTypeId;

    pub const SECP256K1: KeyTypeId = 10;
}

pub trait Signer<U> {
    type Error;

    fn sign<T: AsRef<[u8]>>(&self, data: T) -> Result<U>;
}

pub trait TypedKey {
    const KEY_TYPE: KeyTypeId;
}

pub trait Public: TypedKey + Sized + FromStr + Derive {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait Pair: TypedKey + Sized + FromStr + Derive {
    type Public: Public;

    fn from_slice(data: &[u8]) -> Result<Self>;

    fn from_seed(seed: &bip39::Seed) -> Result<Self>;

    fn from_seed_slice(seed: &[u8]) -> Result<Self>;

    fn public(&self) -> Self::Public;

    fn sign(&self, _: &[u8]) -> Result<Vec<u8>>;

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>>;

    fn public_key(&self) -> Self::Public;

    fn is_extendable(&self) -> bool;
}
