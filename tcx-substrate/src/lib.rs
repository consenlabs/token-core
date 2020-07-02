mod address;
mod keystore;
mod signer;
mod transaction;

pub use address::SubstrateAddress;
pub use keystore::{decode_substrate_keystore, encode_substrate_keystore};
pub use transaction::{SubstrateRawTxIn, SubstrateTxOut};

pub(crate) const SIGNATURE_TYPE_SR25519: u8 = 0x01;
pub(crate) const PAYLOAD_HASH_THRESHOLD: usize = 256;

#[macro_use]
extern crate failure;
extern crate serde_json;
