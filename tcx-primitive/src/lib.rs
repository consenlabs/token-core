#[macro_use]
extern crate failure;

mod bip32;
mod constant;
mod derive;
mod ecc;
mod rand;
mod secp256k1;

use core::result;

pub type Result<T> = result::Result<T, failure::Error>;

pub use crate::bip32::{Bip32DeterministicPrivateKey, Bip32DeterministicPublicKey};
pub use crate::derive::{get_account_path, Derive, DeriveJunction, DerivePath};
pub use crate::ecc::{
    DeterministicPrivateKey, DeterministicPublicKey, Ecdsa, EcdsaSignature, PrivateKey, PublicKey,
};
pub use crate::rand::generate_mnemonic;
pub use crate::secp256k1::{verify_wif, Secp256k1PrivateKey, Secp256k1PublicKey};

/// Key that can be encoded to/from SS58.
pub trait Ss58Codec: Sized {
    /// Some if the string is a properly encoded SS58Check address.
    fn from_ss58check(s: &str) -> Result<Self> {
        let (parsed, _) = Self::from_ss58check_with_version(s)?;
        Ok(parsed)
    }
    /// Some if the string is a properly encoded SS58Check address.
    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)>;

    /// Return the ss58-check string for this key.
    fn to_ss58check_with_version(&self, version: &[u8]) -> String;
}

pub trait ToHex: Sized {
    fn to_hex(&self) -> String;
}

pub trait FromHex: Sized {
    type Error;
    fn from_hex(hex: &str) -> result::Result<Self, Self::Error>;
}
