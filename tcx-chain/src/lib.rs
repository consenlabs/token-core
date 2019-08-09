pub mod bips;
pub mod curve;
pub mod keystore;
pub mod signer;

pub use bips::DerivationInfo;
use core::result;
pub use curve::{
    CurveType, PrivateKey, PublicKey, Secp256k1Curve, Secp256k1PrivateKey, Secp256k1PublicKey,
};
use failure::Error;
pub use keystore::{Account, CoinInfo, HdKeystore, Metadata, Source};
pub use signer::{TransactionSinger, TxSignResult};

#[macro_use]
extern crate failure;
extern crate regex;

pub type Result<T> = result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
