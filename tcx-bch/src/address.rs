use crate::{Error, Result};
use bch_addr::Converter;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Error as BtcAddressError;
use bitcoin::{Address as BtcAddress, Script};
use core::result;

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tcx_btc_fork::{PubKeyScript, ScriptPubKeyComponent};
use tcx_chain::keystore::Address;
use tcx_primitive::Secp256k1PublicKey;
use tcx_primitive::{Pair, Public, Secp256k1Pair};

fn _legacy_to_bch(addr: &str) -> Result<String> {
    let convert = Converter::new();
    if convert.is_legacy_addr(&addr) {
        convert
            .to_cash_addr(&addr)
            .map_err(|_| Error::ConvertToCashAddressFailed(addr.to_string()).into())
    } else {
        Ok(addr.to_string())
    }
}

fn _bch_to_legacy(addr: &str) -> Result<String> {
    let convert = Converter::new();
    if !convert.is_legacy_addr(&addr) {
        convert
            .to_legacy_addr(&addr)
            .map_err(|_| Error::ConvertToLegacyAddressFailed(addr.to_string()).into())
    } else {
        Ok(addr.to_string())
    }
}

impl FromStr for BchAddress {
    type Err = BtcAddressError;

    fn from_str(s: &str) -> result::Result<BchAddress, BtcAddressError> {
        let legacy = _bch_to_legacy(s).expect("_bch_to_legacy");
        let btc_addr = BtcAddress::from_str(&legacy)?;
        Ok(BchAddress(btc_addr))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BchAddress(pub BtcAddress);

impl BchAddress {
    pub fn convert_to_legacy_if_need(addr: &str) -> Result<String> {
        if None == addr.rfind("bitcoincash:") {
            return Ok(addr.to_string());
        }
        _bch_to_legacy(addr)
    }
}

impl Address for BchAddress {
    fn from_public_key(public_key: &[u8], _coin: Option<&str>) -> Result<String> {
        let pubkey = Secp256k1PublicKey::from_slice(&public_key)?;
        let btc_addr = BtcAddress::p2pkh(&pubkey.public_key(), Network::Bitcoin);
        let btc_addr_str = btc_addr.to_string();
        _legacy_to_bch(&btc_addr_str)
    }

    fn from_private_key(wif: &str, coin: Option<&str>) -> Result<String> {
        let pair = Secp256k1Pair::from_wif(wif)?;
        Self::from_public_key(&pair.public_key().to_compressed(), coin)
    }
}

impl Display for BchAddress {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let legacy = self.0.to_string();
        let baddr = _legacy_to_bch(&legacy).expect("legacy_to_bch");
        std::fmt::Display::fmt(&baddr, f)
    }
}

impl PubKeyScript for BchAddress {
    fn script_pub_key(&self) -> Script {
        self.0.script_pubkey()
    }
}

impl ScriptPubKeyComponent for BchAddress {
    fn address_script_like(_target_addr: &str, pub_key: &bitcoin::PublicKey) -> Result<Script> {
        Ok(BtcAddress::p2pkh(&pub_key, Network::Bitcoin).script_pubkey())
    }

    fn address_script_pub_key(target_addr: &str) -> Result<Script> {
        let target_addr = BchAddress::convert_to_legacy_if_need(target_addr)?;
        let addr = BtcAddress::from_str(&target_addr)?;
        Ok(addr.script_pubkey())
    }
}

#[cfg(test)]
mod tests {
    use crate::address::BchAddress;
    use bitcoin::util::misc::hex_bytes;

    use tcx_chain::keystore::Address;

    #[test]
    pub fn test_convert() {
        assert_eq!(
            BchAddress::convert_to_legacy_if_need("2N54wJxopnWTvBfqgAPVWqXVEdaqoH7Suvf").unwrap(),
            "2N54wJxopnWTvBfqgAPVWqXVEdaqoH7Suvf"
        );
        assert_eq!(
            BchAddress::convert_to_legacy_if_need(
                "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885"
            )
            .unwrap(),
            "1oEx5Ztg2DUDYJDxb1AeaiG5TYesikMVU"
        );

        assert_eq!(
            format!(
                "{}",
                BchAddress::convert_to_legacy_if_need("bitcoincash:")
                    .err()
                    .unwrap()
            ),
            "bch_convert_to_legacy_address_failed# address: bitcoincash:"
        );
    }

    #[test]
    pub fn test_from_pub_key() {
        let addr = BchAddress::from_public_key(
            &hex_bytes("026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868")
                .unwrap(),
            Some("bch"),
        )
        .unwrap();
        assert_eq!(
            format!("{}", addr),
            "bitcoincash:qq2ug6v04ht22n0daxxzl0rzlvsmzwcdwuymj77ymy"
        );
    }
}
