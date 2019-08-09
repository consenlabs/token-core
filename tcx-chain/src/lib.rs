pub mod bips;
pub mod curve;
pub mod keystore;
pub mod signer;


use core::result;
use failure::Error;
pub use keystore::{Account, HdKeystore, Metadata, Source, CoinInfo};
pub use curve::{PublicKey, PrivateKey, Secp256k1PublicKey, Secp256k1PrivateKey, Secp256k1Curve, CurveType};
pub use bips::DerivationInfo;
pub use signer::{TransactionSinger, TxSignResult};

#[macro_use]
extern crate failure;
extern crate regex;

pub type Result<T> = result::Result<T,failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
