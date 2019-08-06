
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use secp256k1::{Secp256k1, Message};

use bitcoin::{PrivateKey, TxIn, OutPoint, Script, PublicKey, TxOut, Transaction};
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::hex::FromHex;
use std::str::FromStr;
use bitcoin_hashes::Hash;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language};

pub mod bip143_with_forkid;
pub mod hard_wallet_keystore;
pub mod bch_coin;
pub mod bch_transaction;

use bip143_with_forkid::SighashComponentsWithForkId;
use core::result;

#[macro_use] extern crate failure;
#[macro_use] extern crate hex_literal;
extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;

pub type Result<T> = result::Result<T, failure::Error>;


#[cfg(test)]
mod tests {


}
