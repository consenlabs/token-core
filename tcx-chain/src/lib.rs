//! TokenCore Chain
//! This is an abstract package to define basic chain data structures.

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! tcx_ensure {
        ($cond:expr, $e:expr) => {
            if !($cond) {
                return Err($e.into());
            }
        };
    }
}

use core::result;

#[macro_use]
extern crate failure;
extern crate regex;

pub mod keystore;
pub mod keystore_guard;
pub mod signer;

pub use keystore::{Account, HdKeystore, Metadata, Source};
pub use signer::{Message, MessageSigner, SignedMessage, TransactionSigner};

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_mnemonic")]
    InvalidMnemonic,
    #[fail(display = "invalid_key_type")]
    InvalidKeyType,
    #[fail(display = "invalid_secp256k1_public_key")]
    InvalidSecp256k1PublicKey,
    #[fail(display = "unsupported_curve")]
    UnsupportedCurve,
    #[fail(display = "account_not_found")]
    AccountNotFound,
    #[fail(display = "can_not_derive_pair_from_seed")]
    CanNotDerivePairFromSeed,
    #[fail(display = "can_not_derive_key")]
    CannotDeriveKey,
    #[fail(display = "keystore_locked")]
    KeystoreLocked,
}
