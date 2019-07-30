pub mod signer;
pub mod keystore;
pub mod bips;
pub mod coin;
pub mod curve;


pub use keystore::V3MnemonicKeystore;
pub use signer::{TxSignResult, TransactionSinger};
pub use keystore::{Metadata, Keystore, Source, V3Keystore, HdKeystore, Account, Address};
pub use coin::Coin;
use failure::Error;
use core::result;

#[macro_use] extern crate failure;

pub type Result<T> = result::Result<T, Error>;


