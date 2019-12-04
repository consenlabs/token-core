//! TokenCore Chain
//! This is an abstract package to define basic chain data structures.

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! tcx_ensure {
        ($cond:expr, $e:expr) => {
            if !($cond) {
                return Err($e.into());
            }
        };
    }
}

use core::result;

#[macro_use]
extern crate failure;
extern crate regex;

mod keystore;
mod signer;

pub use signer::{
    Message, MessageSigner, SignedMessage, SignedTransaction, Transaction, TransactionSigner,
    TxSignResult,
};

pub type Result<T> = result::Result<T, failure::Error>;
pub use keystore::{
    Account, Address, EmptyExtra, Extra, HdKeystore, KeystoreGuard, Metadata, PrivateKeystore,
    Source,
};
