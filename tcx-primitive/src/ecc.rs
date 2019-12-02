use super::Result;
use crate::{Derive, Secp256k1PrivateKey, Secp256k1PublicKey};
use std::io;

use serde::{Deserialize, Serialize};

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
    #[fail(display = "invalid_key_type")]
    InvalidKeyType,
}

/// An identifier for a type of cryptographic key.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurveType {
    Secp256k1,
    Secp256r1,
}

pub trait TypedCurve {
    const CURVE_TYPE: CurveType;
}

pub trait PublicKey: TypedCurve + Sized {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn write_into<W: io::Write>(&self, mut writer: W);

    fn to_bytes(&self) -> Vec<u8>;
}

pub trait PrivateKey: TypedCurve + Sized {
    type PublicKey: PublicKey;

    fn from_slice(data: &[u8]) -> Result<Self>;

    fn public_key(&self) -> Self::PublicKey;

    fn sign(&self, _: &[u8]) -> Result<Vec<u8>>;

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait DeterministicPublicKey: Derive {
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

pub struct KeyManage();

impl KeyManage {
    pub fn private_key_from_slice(curve_type: CurveType, data: &[u8]) -> Result<impl PrivateKey> {
        match curve_type {
            CurveType::Secp256k1 => Secp256k1PrivateKey::from_slice(data),
            _ => Err(KeyError::InvalidKeyType.into()),
        }
    }

    pub fn public_key_from_slice(curve_type: CurveType, data: &[u8]) -> Result<impl PublicKey> {
        match curve_type {
            CurveType::Secp256k1 => Secp256k1PublicKey::from_slice(data),
            _ => Err(KeyError::InvalidKeyType.into()),
        }
    }
}
