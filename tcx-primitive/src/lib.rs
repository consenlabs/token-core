#[macro_use]
extern crate failure;

pub mod derive;
mod secp256k1;

use core::result;

pub type Result<T> = result::Result<T, failure::Error>;

pub use crate::secp256k1::{Pair as Secp256k1Pair, Public as Secp256k1PublicKey};

pub use derive::{Derive, DeriveJunction, DerivePath};
use std::str::FromStr;

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
    #[fail(display = "invalid_xpub")]
    InvalidXpub,
    #[fail(display = "invalid_xprv")]
    InvalidXprv,
    #[fail(display = "unsupported_chain")]
    UnsupportedChain,
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

pub trait Public: TypedKey + Sized + Derive {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait Pair: TypedKey + Sized + Derive {
    type Public: Public;

    fn from_slice(data: &[u8]) -> Result<Self>;

    fn from_seed(seed: &bip39::Seed) -> Result<Self>;

    fn from_seed_slice(seed: &[u8]) -> Result<Self>;

    fn extended_public_key(&self) -> Result<Self::Public>;

    fn public_key(&self) -> Self::Public;

    fn sign(&self, _: &[u8]) -> Result<Vec<u8>>;

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>>;

    fn is_extendable(&self) -> bool;
}

/// Key that can be encoded to/from SS58.
//#[cfg(feature = "std")]
pub trait Ss58Codec: Sized {
    /// Some if the string is a properly encoded SS58Check address.
    fn from_ss58check(s: &str) -> Result<Self> {
        let (parsed, _) = Self::from_ss58check_with_version(s)?;
        Ok(parsed)
    }
    /// Some if the string is a properly encoded SS58Check address.
    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)>;
    /// Some if the string is a properly encoded SS58Check address, optionally with
    /// a derivation path following.
    //    fn from_string(s: &str) -> Result<Self> {
    //        Self::from_string_with_version(s)
    //            .and_then(|(r, v)| match v {
    //                Ss58AddressFormat::SubstrateAccountDirect => Ok(r),
    //                v if v == *DEFAULT_VERSION.lock() => Ok(r),
    //                _ => Err(PublicError::UnknownVersion),
    //            })
    //    }

    /// Return the ss58-check string for this key.
    fn to_ss58check_with_version(&self, version: &[u8]) -> String;
}
