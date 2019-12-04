pub mod address;
pub mod signer;
pub mod transaction;

pub use crate::address::Address as TrxAddress;
pub use crate::signer::{
    Message as TrxMessage, SignedMessage as TrxSignedMessage,
    SignedTransaction as TrxSignedTransaction, Transaction as TrxTransaction,
};

#[macro_use]
extern crate tcx_chain;

use digest::Digest;

pub fn keccak(bytes: &[u8]) -> Vec<u8> {
    let mut keccak = sha3::Keccak256::new();
    keccak.input(bytes);
    keccak.result().to_vec()
}
