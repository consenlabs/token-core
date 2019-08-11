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
pub use signer::{TransactionSinger, TxSignResult};

pub type Result<T> = result::Result<T, failure::Error>;
