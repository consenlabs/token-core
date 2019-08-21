use tcx_chain::curve::{PublicKey, Secp256k1PublicKey};

use crate::Error;
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
                .map_err(|_| Error::ConvertToLegacyAddressFailed(addr.to_string()).into())
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
            .map_err(|_| Error::ConvertToCashAddressFailed(legacy.to_string()).into())
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
            .map_err(|_| Error::ConvertToCashAddressFailed(legacy.to_string()).into())
    }

    fn extended_public_key_version() -> [u8; 4] {
        BchTestNetAddress::XPUB_VERSION
    }
    fn extended_private_key_version() -> [u8; 4] {
        BchTestNetAddress::XPRV_VERSION
    }
}

#[cfg(test)]
mod tests {
    use crate::BchAddress;
    use bitcoin::util::misc::hex_bytes;
    use tcx_chain::keystore::Address;
    use tcx_chain::PublicKey;
    use tcx_chain::Secp256k1PublicKey;

    #[test]
    pub fn test_address_valid() {
        assert!(!BchAddress::is_valid(""));

        assert!(!BchAddress::is_valid("n2ZNV88uQbede7C5M5jzi6SyG4GVuPpng6"));

        assert!(BchAddress::is_valid(
            "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885"
        ));
    }

    #[test]
    pub fn test_is_main_net() {
        assert!(!BchAddress::is_main_net(
            "2N54wJxopnWTvBfqgAPVWqXVEdaqoH7Suvf"
        ));
        assert!(!BchAddress::is_main_net(
            "2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB"
        ));

        assert!(BchAddress::is_main_net(
            "3JmreiUEKn8P3SyLYmZ7C1YCd4r2nFy3Dp"
        ));

        assert!(BchAddress::is_main_net(
            "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885"
        ));
    }

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
        let pub_key =
            hex_bytes("026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868")
                .unwrap();
        let addr = BchAddress::from_public_key(&Secp256k1PublicKey::from_slice(&pub_key).unwrap())
            .unwrap();
        assert_eq!(
            addr,
            "bitcoincash:qq2ug6v04ht22n0daxxzl0rzlvsmzwcdwuymj77ymy"
        );
    }
}
