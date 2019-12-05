use super::Result;
use crate::{Derive, FromHex, ToHex};
use std::io;

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
pub type KeyTypeId = u32;

pub mod key_types {
    use super::KeyTypeId;

    pub const SECP256K1: KeyTypeId = 10;
    pub const SECP256R1: KeyTypeId = 20;
}

pub trait TypedKey {
    const KEY_TYPE: KeyTypeId;
}

pub struct EcdsaSignature {
    pub r: Vec<u8>,
    pub s: Vec<u8>,
    pub v: i8,
}

pub trait Ecdsa {
    fn sign<T: AsRef<[u8]>>(&self, data: T) -> Result<EcdsaSignature>;
}

pub trait PublicKey: TypedKey + Sized {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn write_into<W: io::Write>(&self, mut writer: W);

    fn to_bytes(&self) -> Vec<u8>;
}

pub trait PrivateKey: TypedKey + Sized {
    type PublicKey: PublicKey;

    fn from_slice(data: &[u8]) -> Result<Self>;

    fn public_key(&self) -> Self::PublicKey;

    fn sign(&self, _: &[u8]) -> Result<Vec<u8>>;

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait DeterministicPublicKey: Derive + ToHex + FromHex {
    type PublicKey: PublicKey;

    fn public_key(&self) -> Self::PublicKey;
}

pub trait DeterministicPrivateKey: Derive {
    type DeterministicPublicKey: DeterministicPublicKey;
    type PrivateKey: PrivateKey;

    fn from_seed(seed: &[u8]) -> Result<Self>;

    fn private_key(&self) -> Self::PrivateKey;

    fn deterministic_public_key(&self) -> Result<Self::DeterministicPublicKey>;
}
