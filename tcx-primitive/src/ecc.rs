use super::Result;
use crate::{Bip32DeterministicPrivateKey, Bip32DeterministicPublicKey, Derive, Secp256k1PrivateKey, Secp256k1PublicKey, DeriveJunction};
use std::io;

use serde::{Deserialize, Serialize};
use tcx_constants::CurveType;
use crate::ecc::TypedDeterministicPrivateKey::Bip32Sepc256k1;

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
    #[fail(display = "invalid_curve_type")]
    InvalidCurveType,
}

/// An identifier for a type of cryptographic key.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeterministicType {
    BIP32,
}

pub trait PublicKey: Sized {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn write_into<W: io::Write>(&self, mut writer: W);

    fn to_bytes(&self) -> Vec<u8>;
}

pub trait PrivateKey: Sized {
    type PublicKey: PublicKey;

    fn from_slice(data: &[u8]) -> Result<Self>;

    fn public_key(&self) -> Self::PublicKey;

    fn sign(&self, _: &[u8]) -> Result<Vec<u8>>;

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>>;

    fn to_bytes(&self) -> Vec<u8>;
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

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey;
}

pub struct KeyManage();

pub enum TypedPrivateKey {
    Secp256k1(Secp256k1PrivateKey),
}

impl TypedPrivateKey {
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => sk.sign(data),
        }
    }

    pub fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => sk.sign_recoverable(data),
        }
    }

    pub fn public_key(&self) -> TypedPublicKey {
        match self {
            TypedPrivateKey::Secp256k1(sk) => TypedPublicKey::Secp256k1(sk.public_key()),
        }
    }

    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedPrivateKey::Secp256k1(_) => CurveType::SECP256k1,
        }
    }

    pub fn from_slice(curve_type: CurveType, data: &[u8]) -> Result<TypedPrivateKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(TypedPrivateKey::Secp256k1(
                Secp256k1PrivateKey::from_slice(data)?,
            )),
            _ => panic!("invalid curve type"),
        }
    }
}

pub enum TypedPublicKey {
    Secp256k1(Secp256k1PublicKey),
}

impl TypedPublicKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TypedPublicKey::Secp256k1(pk) => pk.to_bytes(),
        }
    }

    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedPublicKey::Secp256k1(_) => CurveType::SECP256k1,
        }
    }

    pub fn from_slice(curve_type: CurveType, data: &[u8]) -> Result<TypedPublicKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(TypedPublicKey::Secp256k1(Secp256k1PublicKey::from_slice(
                data,
            )?)),
            _ => panic!("invalid curve type"),
        }
    }
}

pub enum TypedDeterministicPrivateKey {
    Bip32Sepc256k1(Bip32DeterministicPrivateKey),
}

pub enum TypedDeterministicPublicKey {
    Bip32Sepc256k1(Bip32DeterministicPublicKey),
}

impl TypedDeterministicPublicKey {
    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(_) => CurveType::SECP256k1,
        }
    }

    pub fn public_key(&self) -> TypedPublicKey {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(esk) => TypedPublicKey::Secp256k1(esk.public_key())
        }
    }
}

impl TypedDeterministicPrivateKey {
    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(_) => CurveType::SECP256k1,
        }
    }

    pub fn from_seed(deterministic_type: DeterministicType,  curve_type: CurveType, seed: &[u8]) -> Result<TypedDeterministicPrivateKey> {
        Ok(Bip32Sepc256k1(Bip32DeterministicPrivateKey::from_seed(seed)?))
    }

    pub fn private_key(&self) -> TypedPrivateKey {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(esk) => TypedPrivateKey::Secp256k1(esk.private_key())
        }
    }

    pub fn deterministic_public_key(&self) -> TypedDeterministicPublicKey {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(sk) => TypedDeterministicPublicKey::Bip32Sepc256k1(sk.deterministic_public_key()),
        }
    }
}

impl Derive for TypedDeterministicPrivateKey {
    fn derive<Iter: Iterator<Item=DeriveJunction>>(&self, path: Iter) -> Result<Self> {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(esk) => Ok(TypedDeterministicPrivateKey::Bip32Sepc256k1(esk.derive(path)?))
        }
    }
}

impl Derive for TypedDeterministicPublicKey {
    fn derive<Iter: Iterator<Item=DeriveJunction>>(&self, path: Iter) -> Result<Self> {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => Ok(TypedDeterministicPublicKey::Bip32Sepc256k1(epk.derive(path)?))
        }
    }
}

