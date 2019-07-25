pub mod signer;
pub mod keystore;
pub mod bips;
pub mod coin;

pub use keystore::V3MnemonicKeystore;
pub use signer::{TxSignResult, TransactionSinger};
pub use keystore::{Metadata, Keystore, Source, V3Keystore};
use failure::Error;
use core::result;

#[macro_use] extern crate failure;

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
