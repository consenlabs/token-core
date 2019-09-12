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

pub mod bips;
pub mod curve;
pub mod keystore;
pub mod signer;

pub use bips::DerivationInfo;
pub use curve::{
    CurveType, PrivateKey, PublicKey, Secp256k1Curve, Secp256k1PrivateKey, Secp256k1PublicKey,
};
pub use keystore::{Account, CoinInfo, HdKeystore, Metadata, Source};

//cfg_if! {
//    if #[cfg(test)] {
//        pub use crate::keystore::MockHdKeystore as HdKeystore;
//    } else {
//        pub use crate::keystore::HdKeystore;
//    }
//}

pub use signer::{SignedTransaction, Transaction, TransactionSigner, TxSignResult};

use tcx_primitive::key::Public;

pub trait Address1: Sized {
    type Error;
    type Public: Public;

    fn from_public(public: &Self::Public) -> core::result::Result<Self, Self::Error>;
}

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
}
