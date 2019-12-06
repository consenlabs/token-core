pub mod address;
pub mod bip143_with_forkid;
pub mod signer;
pub mod transaction;

use core::result;
use serde::{Deserialize, Serialize};
use std::iter::IntoIterator;
use std::str::FromStr;
use tcx_chain::Address;
use tcx_chain::Extra;

#[macro_use]
extern crate failure;

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

#[macro_use]
extern crate tcx_chain;

pub type Result<T> = result::Result<T, failure::Error>;

pub use signer::{BitcoinForkSinger, BtcForkSegWitTransaction, BtcForkTransaction};
pub use transaction::{BtcForkSignedTxOutput, BtcForkTxInput, Utxo};

pub use address::{BtcForkAddress, PubKeyScript};
pub use signer::ScriptPubKeyComponent;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "decrypt_xpub_error")]
    DecryptXPubError,
    #[fail(display = "unsupported_chain")]
    UnsupportedChain,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAddress {
    pub address: String,
    #[serde(rename = "type")]
    pub addr_type: String,
    pub derived_path: String,
}
