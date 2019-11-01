pub mod address;
pub mod transaction;

pub use crate::address::Address as TrxAddress;
pub use crate::transaction::{
    SignedTransaction as TrxSignedTransaction, Transaction as TrxTransaction,
};

#[macro_use]
extern crate tcx_chain;
