mod derive;
pub mod secp256k1;

//use core::result::Result;
use crate::Result;
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
//
//#[derive(Fail, Debug, PartialEq)]
//pub enum Error {
//    #[fail(display = "invalid_mnemonic")]
//    InvalidMnemonic,
//    #[fail(display = "invalid_key_type")]
//    InvalidKeyType,
//    #[fail(display = "invalid_secp256k1_public_key")]
//    InvalidSecp256k1PublicKey,
//    #[fail(display = "unsupported_curve")]
//    UnsupportedCurve,
//    #[fail(display = "account_not_found")]
//    AccountNotFound,
//    #[fail(display = "can_not_derive_pair_from_seed")]
//    CanNotDerivePairFromSeed,
//    #[fail(display = "can_not_derive_key")]
//    CannotDeriveKey,
//}
//
//#[allow(dead_code)]
//#[derive(Debug)]
//pub enum KeyError {
//    InvalidEcdsa,
//    InvalidChildNumberFormat,
//    OverflowChildNumber,
//    InvalidDerivationPathFormat,
//    InvalidKeyLength,
//    InvalidSignature,
//    InvalidSignatureLength,
//    InvalidChildNumber,
//    CannotDeriveFromHardenedKey,
//    CannotDeriveKey,
//    InvalidBase58,
//    InvalidPrivateKey,
//    InvalidPublicKey,
//    InvalidMessage,
//    InvalidRecoveryId,
//    InvalidTweak,
//    NotEnoughMemory,
//    Unknown,
//}

#[allow(dead_code)]
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

pub trait Signer<U> {
    type Error;

    fn sign<T: AsRef<[u8]>>(&self, data: T) -> Result<U>;
}

pub trait TypedKey {
    const KEY_TYPE: KeyTypeId;
}

pub trait Public: AsRef<[u8]> + TypedKey + Sized + FromStr + Derive {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn as_slice(&self) -> &[u8];
}

pub trait Pair: TypedKey + Sized + FromStr + Derive {
    type Public: Public;

    type Seed: Default + AsRef<[u8]> + AsMut<[u8]> + Clone;

    fn from_slice(data: &[u8]) -> Result<Self>;

    fn from_seed(seed: &Self::Seed) -> Result<Self>;

    fn from_seed_slice(seed: &[u8]) -> Result<Self>;

    fn public(&self) -> Self::Public;

    fn sign(&self, _: &[u8]) -> Result<Vec<u8>>;

    fn to_normal_pair(&self) -> Self;

    fn public_key(&self) -> Self::Public;

    fn is_extendable(&self) -> bool;
}
