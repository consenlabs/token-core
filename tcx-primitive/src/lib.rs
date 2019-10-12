#[macro_use]
extern crate failure;

mod error;
pub mod key;
mod types;

use core::result;

pub use error::Error;
pub use types::{U128, U160, U256};

pub type Result<T> = result::Result<T, failure::Error>;

pub use key::{
    secp256k1::ArbitraryNetworkExtendedPrivKey, secp256k1::ArbitraryNetworkExtendedPubKey,
    secp256k1::Pair as Secp256k1Pair, secp256k1::Public as Secp256k1Public, Derive, DeriveJunction,
    DerivePath, Pair, Public,
};
