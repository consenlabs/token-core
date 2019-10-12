#[macro_use]
extern crate failure;

mod error;
pub mod key;
mod types;

use core::result;
use serde::{Deserialize, Serialize};

pub use error::Error;
pub use types::{U128, U160, U256};

pub type Result<T> = result::Result<T, failure::Error>;

pub use key::{
    secp256k1::ArbitraryNetworkExtendedPrivKey, secp256k1::ArbitraryNetworkExtendedPubKey,
    secp256k1::Pair as Secp256k1Pair, secp256k1::Public as Secp256k1PublicKey, Derive,
    DeriveJunction, DerivePath, Pair, Public,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CurveType {
    SECP256k1,          /* "secp256k1" */
    ED25519,            /* "ed25519" */
    ED25519Blake2bNano, /* "ed25519-blake2b-nano" */
    Curve25519,         /* "curve25519" */
    NIST256p1,
}
