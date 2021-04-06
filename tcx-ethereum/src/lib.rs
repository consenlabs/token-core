mod address;
mod signer;
mod transaction;
mod chain_id;

pub use crate::transaction::{EthereumTxOut, EthereumTxIn};
pub use crate::chain_id::{ChainInfo, chain_id_from_network};
pub use crate::address::EthereumAddress;

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
    InvalidData
}