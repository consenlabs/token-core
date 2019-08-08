pub mod signer;
pub mod keystore;
pub mod bips;
pub mod coin;
pub mod curve;



pub use signer::{TxSignResult, TransactionSinger};
pub use keystore::{Metadata, Source,  HdKeystore, Account};
pub use coin::Coin;
use failure::Error;
use core::result;

#[macro_use] extern crate failure;
extern crate regex;

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
