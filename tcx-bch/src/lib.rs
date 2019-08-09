
use bitcoin::network::constants::Network;
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
use bip39::{Mnemonic, Language, Seed};

pub mod bip143_with_forkid;
pub mod hard_wallet_keystore;
pub mod address;
pub mod transaction;

use bip143_with_forkid::SighashComponentsWithForkId;
use core::result;
use tcx_chain::keystore::{Extra, CoinInfo};
use tcx_chain::curve::{CurveType, Secp256k1Curve};
use serde::{Deserialize, Serialize};
use tcx_chain::keystore::Address;

#[macro_use] extern crate failure;
#[macro_use] extern crate hex_literal;
extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;

pub type Result<T> = result::Result<T, failure::Error>;


const SYMBOL: &'static str = "BCH";
const PATH: &'static str = "m/44'/145'/0'/0/0";

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedPubKeyExtra {
    xpub: String
}


impl Extra for ExtendedPubKeyExtra {
    fn from(coin_info: &CoinInfo, seed: &Seed) -> Result<Self> {
        let derivation_info = match coin_info.curve {
            CurveType::SECP256k1 => {
                Secp256k1Curve::extended_pub_key(&coin_info.derivation_path, &seed)
            },
            _ => Err(format_err!("{}", "unsupport_chain"))
        }?;
        let xpub = address::BchAddress::extended_public_key(&derivation_info);
        Ok(ExtendedPubKeyExtra {
            xpub
        })
    }

}



#[cfg(test)]
mod tests {


}
