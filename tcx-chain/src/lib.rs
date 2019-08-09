pub mod bips;
pub mod coin;
pub mod curve;
pub mod keystore;
pub mod signer;

pub use coin::Coin;
use core::result;
use failure::Error;
pub use keystore::{Account, HdKeystore, Metadata, Source};
pub use signer::{TransactionSinger, TxSignResult};

#[macro_use]
extern crate failure;
extern crate regex;

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
