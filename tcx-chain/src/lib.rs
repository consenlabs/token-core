//! TokenCore Chain
//! This is an abstract package to define basic chain data structures.

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
pub use signer::{TransactionSigner, Transaction, SignedTransaction, TxSignResult};
use std::str::FromStr;

use tcx_primitive::key::Public;

pub trait Address1: Sized {
    type Error;
    type Public: Public;

    fn from_public(public: &Self::Public) -> core::result::Result<Self, Self::Error>;
}

pub type Result<T> = result::Result<T, failure::Error>;
