use super::Result;
use crate::{
    Bip32DeterministicPrivateKey, Bip32DeterministicPublicKey, Derive, DeriveJunction, FromHex,
    Secp256k1PrivateKey, Secp256k1PublicKey, ToHex,
};
use std::io;

use crate::ecc::TypedDeterministicPrivateKey::{Bip32Sepc256k1, SubSr25519};
use crate::sr25519::{Sr25519PrivateKey, Sr25519PublicKey};
use serde::{Deserialize, Serialize};
use tcx_constants::CurveType;
//use parity_scale_codec::Encode;

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
    #[fail(display = "invalid_signature")]
    InvalidSignature,
    #[fail(display = "invalid_child_number")]
    InvalidChildNumber,
    #[fail(display = "cannot_derive_from_hardened_key")]
    CannotDeriveFromHardenedKey,
    #[fail(display = "cannot_derive_key")]
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
    #[fail(display = "invalid_curve_type")]
    InvalidCurveType,
}

///// An identifier for a type of cryptographic key.
//#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
//#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
//pub enum DeterministicType {
//    BIP32,
//    SUBSTRATE,
//}

pub trait PublicKey: Sized {
    fn from_slice(data: &[u8]) -> Result<Self>;

    fn write_into<W: io::Write>(&self, writer: W);

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

    fn from_mnemonic(mnemonic: &str) -> Result<Self>;

    fn private_key(&self) -> Self::PrivateKey;

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey;
}

pub trait TypedPrivateKeyDisplay {
    fn fmt(data: &[u8], network: &str) -> Result<String>;
}

pub enum TypedPrivateKey {
    Secp256k1(Secp256k1PrivateKey),
    Sr25519(Sr25519PrivateKey),
}

impl TypedPrivateKey {
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => sk.sign(data),
            TypedPrivateKey::Sr25519(sk) => sk.sign(data),
        }
    }

    pub fn as_secp256k1(&self) -> Result<&Secp256k1PrivateKey> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => Ok(sk),
            _ => Err(format_err!("not_support")),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => sk.to_bytes(),
            TypedPrivateKey::Sr25519(sk) => sk.to_bytes(),
        }
    }

    pub fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            TypedPrivateKey::Secp256k1(sk) => sk.sign_recoverable(data),
            TypedPrivateKey::Sr25519(sk) => sk.sign_recoverable(data),
        }
    }

    pub fn public_key(&self) -> TypedPublicKey {
        match self {
            TypedPrivateKey::Secp256k1(sk) => TypedPublicKey::Secp256k1(sk.public_key()),
            TypedPrivateKey::Sr25519(sk) => TypedPublicKey::Sr25519(sk.public_key()),
        }
    }

    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedPrivateKey::Secp256k1(_) => CurveType::SECP256k1,
            TypedPrivateKey::Sr25519(_) => CurveType::SubSr25519,
        }
    }

    pub fn from_slice(curve_type: CurveType, data: &[u8]) -> Result<TypedPrivateKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(TypedPrivateKey::Secp256k1(
                Secp256k1PrivateKey::from_slice(data)?,
            )),
            CurveType::SubSr25519 => Ok(TypedPrivateKey::Sr25519(Sr25519PrivateKey::from_slice(
                data,
            )?)),
            _ => Err(KeyError::InvalidCurveType.into()),
        }
    }
}

pub enum TypedPublicKey {
    Secp256k1(Secp256k1PublicKey),
    Sr25519(Sr25519PublicKey),
}

impl TypedPublicKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TypedPublicKey::Secp256k1(pk) => pk.to_bytes(),
            TypedPublicKey::Sr25519(pk) => pk.to_bytes(),
        }
    }

    pub fn as_secp256k1(&self) -> Result<&Secp256k1PublicKey> {
        match self {
            TypedPublicKey::Secp256k1(pk) => Ok(pk),
            _ => Err(format_err!("not support")),
        }
    }

    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedPublicKey::Secp256k1(_) => CurveType::SECP256k1,
            TypedPublicKey::Sr25519(_) => CurveType::SubSr25519,
        }
    }

    pub fn from_slice(curve_type: CurveType, data: &[u8]) -> Result<TypedPublicKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(TypedPublicKey::Secp256k1(Secp256k1PublicKey::from_slice(
                data,
            )?)),
            CurveType::SubSr25519 => {
                Ok(TypedPublicKey::Sr25519(Sr25519PublicKey::from_slice(data)?))
            }

            _ => Err(KeyError::InvalidCurveType.into()),
        }
    }
}

pub enum TypedDeterministicPublicKey {
    Bip32Sepc256k1(Bip32DeterministicPublicKey),
    SubSr25519(Sr25519PublicKey), //    SubstrateSr25519()
}

impl TypedDeterministicPublicKey {
    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(_) => CurveType::SECP256k1,
            TypedDeterministicPublicKey::SubSr25519(_) => CurveType::SubSr25519,
        }
    }

    pub fn public_key(&self) -> TypedPublicKey {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => {
                TypedPublicKey::Secp256k1(epk.public_key())
            }
            TypedDeterministicPublicKey::SubSr25519(epk) => {
                TypedPublicKey::Sr25519(epk.public_key())
            }
        }
    }
}

impl ToString for TypedDeterministicPublicKey {
    fn to_string(&self) -> String {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => epk.to_string(),
            TypedDeterministicPublicKey::SubSr25519(epk) => epk.to_string(),
        }
    }
}

impl Derive for TypedDeterministicPublicKey {
    fn derive(&self, path: &str) -> Result<Self> {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => Ok(
                TypedDeterministicPublicKey::Bip32Sepc256k1(epk.derive(path)?),
            ),
            TypedDeterministicPublicKey::SubSr25519(epk) => {
                Ok(TypedDeterministicPublicKey::SubSr25519(epk.derive(path)?))
            }
        }
    }
}

pub enum TypedDeterministicPrivateKey {
    Bip32Sepc256k1(Bip32DeterministicPrivateKey),
    SubSr25519(Sr25519PrivateKey),
}

impl TypedDeterministicPrivateKey {
    pub fn curve_type(&self) -> CurveType {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(_) => CurveType::SECP256k1,
            TypedDeterministicPrivateKey::SubSr25519(_) => CurveType::SubSr25519,
        }
    }

    //    pub fn from_seed(
    //        curve_type: CurveType,
    //        seed: &[u8],
    //    ) -> Result<TypedDeterministicPrivateKey> {
    //        match curve_type {
    //            CurveType::SECP256k1 => Ok(Bip32Sepc256k1(
    //                Bip32DeterministicPrivateKey::from_seed(seed)?,
    //            )),
    //            CurveType::SubSr25519 => Ok(SubSr25519(Sr25519PrivateKey::from_seed(seed)?)),
    //            _ => Err(format_err!("unsupported_curve_type"))
    //        }
    //    }

    pub fn from_mnemonic(
        curve_type: CurveType,
        mnemonic: &str,
    ) -> Result<TypedDeterministicPrivateKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(Bip32Sepc256k1(
                Bip32DeterministicPrivateKey::from_mnemonic(mnemonic)?,
            )),
            CurveType::SubSr25519 => Ok(SubSr25519(Sr25519PrivateKey::from_mnemonic(mnemonic)?)),
            _ => Err(format_err!("unsupported_curve_type")),
        }
    }

    pub fn private_key(&self) -> TypedPrivateKey {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(dsk) => {
                TypedPrivateKey::Secp256k1(dsk.private_key())
            }
            TypedDeterministicPrivateKey::SubSr25519(dsk) => {
                TypedPrivateKey::Sr25519(dsk.private_key())
            }
        }
    }

    pub fn deterministic_public_key(&self) -> TypedDeterministicPublicKey {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(sk) => {
                TypedDeterministicPublicKey::Bip32Sepc256k1(sk.deterministic_public_key())
            }
            TypedDeterministicPrivateKey::SubSr25519(sk) => {
                TypedDeterministicPublicKey::SubSr25519(sk.deterministic_public_key())
            }
        }
    }
}

impl ToString for TypedDeterministicPrivateKey {
    fn to_string(&self) -> String {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(sk) => sk.to_string(),
            // todo: does sr need export?
            TypedDeterministicPrivateKey::SubSr25519(sk) => "".to_owned(),
        }
    }
}

impl TypedDeterministicPublicKey {
    pub fn from_hex(curve_type: CurveType, hex: &str) -> Result<TypedDeterministicPublicKey> {
        match curve_type {
            CurveType::SECP256k1 => Ok(TypedDeterministicPublicKey::Bip32Sepc256k1(
                Bip32DeterministicPublicKey::from_hex(hex)?,
            )),
            CurveType::SubSr25519 => Ok(TypedDeterministicPublicKey::SubSr25519(
                Sr25519PublicKey::from_hex(hex)?,
            )),
            _ => Err(format_err!("unsupported_curve_type")),
        }
    }
}

impl ToHex for TypedDeterministicPublicKey {
    fn to_hex(&self) -> String {
        match self {
            TypedDeterministicPublicKey::Bip32Sepc256k1(epk) => epk.to_hex(),
            TypedDeterministicPublicKey::SubSr25519(epk) => epk.to_hex(),
        }
    }
}

impl Derive for TypedDeterministicPrivateKey {
    fn derive(&self, path: &str) -> Result<Self> {
        match self {
            TypedDeterministicPrivateKey::Bip32Sepc256k1(dsk) => Ok(
                TypedDeterministicPrivateKey::Bip32Sepc256k1(dsk.derive(path)?),
            ),
            TypedDeterministicPrivateKey::SubSr25519(dsk) => {
                Ok(TypedDeterministicPrivateKey::SubSr25519(dsk.derive(path)?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PrivateKey, PublicKey, TypedDeterministicPrivateKey, TypedPrivateKey};
    use crate::{Derive, DerivePath, TypedPublicKey};
    use bip39::{Language, Mnemonic, Seed};
    use std::str::FromStr;
    use tcx_constants::{CurveType, TEST_MNEMONIC};

    fn default_seed() -> Seed {
        let mn = Mnemonic::from_phrase(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            Language::English,
        )
        .unwrap();
        Seed::new(&mn, "")
    }

    fn default_private_key() -> Vec<u8> {
        hex::decode("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc").unwrap()
    }

    const PUB_KEY_HEX: &'static str =
        "02b95c249d84f417e3e395a127425428b540671cc15881eb828c17b722a53fc599";

    #[test]
    fn typed_private_key() {
        let ret = TypedPrivateKey::from_slice(CurveType::ED25519, &default_private_key());
        assert!(ret.is_err());

        let sk = TypedPrivateKey::from_slice(CurveType::SECP256k1, &default_private_key()).unwrap();

        assert_eq!(sk.to_bytes(), default_private_key());
        assert_eq!(sk.as_secp256k1().unwrap().to_bytes(), default_private_key());
        assert_eq!(sk.curve_type(), CurveType::SECP256k1);
        assert_eq!(hex::encode(sk.public_key().to_bytes()), PUB_KEY_HEX);

        let sign_ret = sk.sign(&default_private_key()).unwrap();
        assert_eq!(hex::encode(sign_ret), "304402206614e4bfa3ba1f6c975286a0a683871d6f0525a0860631afa5bea4da78ca012a02207a663d4980abed218683f66a63bbb766975fd525b8442a0424f6347c3d4f9261");
    }

    #[test]
    fn typed_deterministic_private_key() {
        let root =
            TypedDeterministicPrivateKey::from_mnemonic(CurveType::SECP256k1, &TEST_MNEMONIC)
                .unwrap();

        let dpk = root
            .derive("m/44'/0'/0'")
            .unwrap()
            .deterministic_public_key();

        assert_eq!(dpk.to_string(), "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8");

        assert_eq!(dpk.curve_type(), CurveType::SECP256k1);
        assert_eq!(
            hex::encode(dpk.public_key().to_bytes()),
            "029d23439ecb195eb06a0d44a608960d18702fd97e19c53451f0548f568207af77"
        );
        let child_dpk = dpk.derive("0/0").unwrap();
        assert_eq!(child_dpk.to_string(), "xpub6FuzpGNBc46EfvmcvECyqXjrzGcKErQgpQcpvhw1tiC5yXvi1jUkzudMpdg5AaguiFstdVR5ASDbSceBswKRy6cAhpTgozmgxMUayPDrLLX");

        let dsk = root.derive("m/44'/0'/0'").unwrap();

        assert_eq!(dsk.to_string(), "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ");
    }

    #[test]
    fn test_typed_public_key() {
        let pub_key = hex::decode(PUB_KEY_HEX).unwrap();
        let ret = TypedPublicKey::from_slice(CurveType::ED25519, &pub_key);
        assert!(ret.is_err());

        let pk = TypedPublicKey::from_slice(CurveType::SECP256k1, &pub_key).unwrap();

        assert_eq!(pk.curve_type(), CurveType::SECP256k1);

        assert_eq!(hex::encode(pk.to_bytes()), PUB_KEY_HEX);
        assert_eq!(
            hex::encode(pk.as_secp256k1().unwrap().to_bytes()),
            PUB_KEY_HEX
        );
        assert_eq!(pk.curve_type(), CurveType::SECP256k1);
    }
}
