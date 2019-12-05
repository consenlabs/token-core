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

#[cfg(test)]
mod tests {
    use crate::BtcForkExtra;
    use bip39::{Language, Mnemonic, Seed};
    use tcx_chain::Extra;
    use tcx_constants::{CoinInfo, CurveType};

    #[test]
    pub fn extra_test() {
        let coin_info = CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        };
        let extra = BtcForkExtra::from_xpub("tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R", "LITECOIN").unwrap();
        assert_eq!("GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==", extra.enc_xpub);
        assert_eq!(
            "LNp88kijfnFKGcp1aPdnMkpfMycw1v7KdQ",
            extra.external_address.address
        );
        let xpub = extra.xpub().unwrap();
        assert_eq!("tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R", xpub);

        let mnemonic = Mnemonic::from_phrase(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            Language::English,
        )
        .unwrap();
        let seed = Seed::new(&mnemonic, "");
        let extra = BtcForkExtra::new(&coin_info, seed.as_bytes()).unwrap();
        assert_eq!(extra.enc_xpub, "MwDMFXVWDEuWvBogeW1v/MOMFDnGnnflm2JAPvJaJZO4HXp8fCsWETA7u8MzOW3KaPksglpUHLN3xkDr2QWMEQq0TewFZoZ3KsjmLW0KGMRN7XQKqo/omkSEsPfalVnp9Zxm2lpxVmIacqvlernVSg==");
        assert_eq!(extra.xpub().unwrap(), "xpub6D3MqTwuLWB5veAfhDjPu1oHfS6L1imVbf22zQFWJW9EtnSmYYqiGMGkW1MCsT2HmkW872tefMY9deewW6DGd8zE7RcXVv8wKhZnbJeidjT");

        let next_receive_address = extra.calc_external_address(2, "LITECOIN").unwrap();
        assert_eq!(
            "LYdJgidYoP6kvDkyyczPcLn78vghCFfKpe",
            next_receive_address.address
        );
    }
}
