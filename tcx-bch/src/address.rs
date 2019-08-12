use tcx_chain::curve::{PublicKey, Secp256k1PublicKey};

use crate::Result;
use bch_addr::Converter;
use bitcoin::network::constants::Network;
use bitcoin::Address as BtcAddress;
use tcx_chain::keystore::Address;

pub struct BchAddress {}

impl BchAddress {
    const XPUB_VERSION: [u8; 4] = [0x04, 0x88, 0xb2, 0x1e];
    const XPRV_VERSION: [u8; 4] = [0x04, 0x88, 0xad, 0xe4];

    pub fn is_main_net(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_mainnet_addr(addr)
    }

    pub fn convert_to_legacy_if_need(addr: &str) -> Result<String> {
        let convert = Converter::new();
        if !convert.is_legacy_addr(&addr) {
            convert
                .to_legacy_addr(&addr)
                .map_err(|_| format_err!("convert failed"))
        } else {
            Ok(addr.to_string())
        }
    }
}

impl Address for BchAddress {
    fn is_valid(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(addr)
    }

    fn from_public_key(pub_key: &impl PublicKey) -> Result<String> {
        let pub_key: Secp256k1PublicKey = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let legacy = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
        let convert = Converter::new();
        convert
            .to_cash_addr(&legacy.to_string())
            .map_err(|_err| format_err!("{}", "generate_address_failed"))
    }
}

pub struct BchTestNetAddress {}

impl BchTestNetAddress {
    const XPUB_VERSION: [u8; 4] = [0x04, 0x35, 0x87, 0xCF];
    const XPRV_VERSION: [u8; 4] = [0x04, 0x35, 0x83, 0x94];
}

impl Address for BchTestNetAddress {
    fn is_valid(address: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(address)
    }

    fn from_public_key(pub_key: &impl PublicKey) -> Result<String> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let legacy = BtcAddress::p2pkh(&pub_key, Network::Testnet);
        let convert = Converter::new();
        convert
            .to_cash_addr(&legacy.to_string())
            .map_err(|_err| format_err!("{}", "generate_address_failed"))
    }

    fn extended_public_key_version() -> [u8; 4] {
        BchTestNetAddress::XPUB_VERSION
    }
    fn extended_private_key_version() -> [u8; 4] {
        BchTestNetAddress::XPRV_VERSION
    }
}
