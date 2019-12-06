use super::Result;
use crate::{
    Bip32DeterministicPrivateKey, Bip32DeterministicPublicKey, Derive, DeriveJunction, FromHex,
    Secp256k1PrivateKey, Secp256k1PublicKey, ToHex,
};
use std::io;

use crate::ecc::TypedDeterministicPrivateKey::Bip32Sepc256k1;
use serde::{Deserialize, Serialize};
use tcx_constants::CurveType;

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

pub trait DeterministicPublicKey: Derive + ToHex + FromHex {
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

    pub fn as_secp256k1(&self) -> Result<&Secp256k1PrivateKey> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => Ok(sk),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => sk.to_bytes(),
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
            _ => Err(KeyError::InvalidCurveType.into()),
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

    pub fn as_secp256k1(&self) -> Result<&Secp256k1PublicKey> {
        match self {
            TypedPublicKey::Secp256k1(pk) => Ok(pk),
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

            _ => Err(KeyError::InvalidCurveType.into()),
        }
    }
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
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => {
                TypedPublicKey::Secp256k1(epk.public_key())
            }
        }
    }
}

impl ToString for TypedDeterministicPublicKey {
    fn to_string(&self) -> String {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => epk.to_string()
        }
    }
}

impl Derive for TypedDeterministicPublicKey {
    fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter) -> Result<Self> {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => Ok(
                TypedDeterministicPublicKey::Bip32Sepc256k1(epk.derive(path)?),
            ),
        }
    }
}

pub enum TypedDeterministicPrivateKey {
    Bip32Sepc256k1(Bip32DeterministicPrivateKey),
}

impl TypedDeterministicPrivateKey {
    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(_) => CurveType::SECP256k1,
        }
    }

    pub fn from_seed(
        _deterministic_type: DeterministicType,
        _curve_type: CurveType,
        seed: &[u8],
    ) -> Result<TypedDeterministicPrivateKey> {
        Ok(Bip32Sepc256k1(Bip32DeterministicPrivateKey::from_seed(
            seed,
        )?))
    }

    pub fn private_key(&self) -> TypedPrivateKey {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(dsk) => {
                TypedPrivateKey::Secp256k1(dsk.private_key())
            }
        }
    }

    pub fn deterministic_public_key(&self) -> TypedDeterministicPublicKey {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(sk) => {
                TypedDeterministicPublicKey::Bip32Sepc256k1(sk.deterministic_public_key())
            }
        }
    }
}

impl ToString for TypedDeterministicPrivateKey {
    fn to_string(&self) -> String {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(sk) => sk.to_string()
        }
    }
}

impl TypedDeterministicPublicKey {
    pub fn from_hex(
        _deterministic_type: DeterministicType,
        curve_type: CurveType,
        hex: &str,
    ) -> Result<TypedDeterministicPublicKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(TypedDeterministicPublicKey::Bip32Sepc256k1(
                Bip32DeterministicPublicKey::from_hex(hex)?,
            )),
            _ => Err(KeyError::InvalidCurveType.into()),
        }
    }
}

impl ToHex for TypedDeterministicPublicKey {
    fn to_hex(&self) -> String {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => epk.to_hex(),
        }
    }
}

impl Derive for TypedDeterministicPrivateKey {
    fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter) -> Result<Self> {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(dsk) => Ok(
                TypedDeterministicPrivateKey::Bip32Sepc256k1(dsk.derive(path)?),
            ),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{TypedPublicKey, TypedPrivateKey, PublicKey, PrivateKey, DeterministicType, TypedDeterministicPrivateKey, TypedDeterministicPublicKey, DeterministicPublicKey, DeterministicPrivateKey};
    use crate::{DerivePath, Derive};
    use std::str::FromStr;
    use tcx_constants::CurveType;
    use bip39::{Language, Mnemonic, Seed};

    fn default_seed() -> Seed {
        let mn = Mnemonic::from_phrase(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            Language::English,
        )
            .unwrap();
        Seed::new(&mn, "")
    }

    fn default_private_key() -> Vec<u8>{
        hex::decode("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc").unwrap()
    }

    #[test]
    fn typed_private_key() {
        let sk = TypedPrivateKey::from_slice(CurveType::SECP256k1,&default_private_key()).unwrap();

        assert_eq!(sk.to_bytes(), default_private_key());
        assert_eq!(sk.as_secp256k1().unwrap().to_bytes(), default_private_key());
        assert_eq!(sk.curve_type(), CurveType::SECP256k1);
    }

    #[test]
    fn typed_deterministic_private_key() {
        let root = TypedDeterministicPrivateKey::from_seed(
            DeterministicType::BIP32, CurveType::SECP256k1, &default_seed().as_bytes()).unwrap();

        let dpk = root
            .derive(DerivePath::from_str("m/44'/0'/0'").unwrap().into_iter())
            .unwrap()
            .deterministic_public_key();

        assert_eq!(dpk.to_string(), "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8");

        let dsk = root
            .derive(DerivePath::from_str("m/44'/0'/0'").unwrap().into_iter())
            .unwrap();

        assert_eq!(dsk.to_string(), "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ");
    }

    #[test]
    fn typed_public_key() {
        let sk = TypedPrivateKey::from_slice(CurveType::SECP256k1, &default_private_key()).unwrap();

        let pk = sk.public_key();

        assert_eq!(hex::encode(pk.to_bytes()), "02b95c249d84f417e3e395a127425428b540671cc15881eb828c17b722a53fc599");
        assert_eq!(hex::encode(pk.as_secp256k1().unwrap().to_bytes()), "02b95c249d84f417e3e395a127425428b540671cc15881eb828c17b722a53fc599");
        assert_eq!(pk.curve_type(), CurveType::SECP256k1);
    }

}
