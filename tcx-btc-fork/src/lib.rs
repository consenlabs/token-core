pub mod address;
pub mod bip143_with_forkid;
pub mod signer;
pub mod transaction;

use core::result;
use serde::{Deserialize, Serialize};

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

pub use address::{BtcForkAddress, PubKeyScript, WifDisplay};
pub use signer::ScriptPubKeyComponent;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "decrypt_xpub_error")]
    DecryptXPubError,
    #[fail(display = "unsupported_chain")]
    UnsupportedChain,
    #[fail(display = "missing_network")]
    MissingNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAddress {
    pub address: String,
    #[serde(rename = "type")]
    pub addr_type: String,
    pub derived_path: String,
}
