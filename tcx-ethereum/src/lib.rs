mod address;
mod chain_id;
mod signer;
mod transaction;

pub use crate::address::EthereumAddress;
pub use crate::chain_id::{chain_id_from_network, ChainInfo};
pub use crate::transaction::{EthereumMsgIn, EthereumMsgOut, EthereumTxIn, EthereumTxOut};
use digest::Digest;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "cannot_found_account")]
    CannotFoundAccount,

    #[fail(display = "cannot_get_private_key")]
    CannotGetPrivateKey,

    #[fail(display = "invalid_nonce")]
    InvalidNonce,

    #[fail(display = "invalid_to")]
    InvalidTo,

    #[fail(display = "invalid_value")]
    InvalidValue,

    #[fail(display = "invalid_gas_price")]
    InvalidGasPrice,

    #[fail(display = "invalid_gas")]
    InvalidGas,

    #[fail(display = "invalid_data")]
    InvalidData,
}

pub fn keccak(bytes: &[u8]) -> Vec<u8> {
    let mut keccak = sha3::Keccak256::new();
    keccak.input(bytes);
    keccak.result().to_vec()
}
