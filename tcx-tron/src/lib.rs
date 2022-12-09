pub mod address;
pub mod signer;
pub mod transaction;

pub use crate::address::Address as TrxAddress;

use sha3::{Digest, Keccak256, Sha3_256};

pub fn keccak(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}
