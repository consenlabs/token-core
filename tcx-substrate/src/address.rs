use sp_core::crypto::Ss58AddressFormat;
use sp_core::crypto::Ss58Codec;
use sp_core::sr25519::Public;
use tcx_chain::Address;
use tcx_constants::{CoinInfo, Result};
use tcx_primitive::{PublicKey, Sr25519PublicKey, TypedPublicKey};

pub struct SubstrateAddress();

impl Address for SubstrateAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        // todo: TypedPublicKey to public key
        let sr_pk = Sr25519PublicKey::from_slice(&public_key.to_bytes())?;
        let address = match coin.coin.as_str() {
            "KUSAMA" => sr_pk
                .0
                .to_ss58check_with_version(Ss58AddressFormat::custom(2)),
            "POLKADOT" => sr_pk
                .0
                .to_ss58check_with_version(Ss58AddressFormat::custom(0)),
            _ => "".to_owned(),
        };

        Ok(address)
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        match Public::from_ss58check_with_version(address) {
            Ok((_addr, version)) => match coin.coin.as_str() {
                "KUSAMA" => version == Ss58AddressFormat::custom(2),
                "POLKADOT" => version == Ss58AddressFormat::custom(0),
                _ => false,
            },
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::FromHex;

    #[test]
    fn test_address_from_public() {
        let pub_key: Sr25519PublicKey = Sr25519PublicKey::from_hex(
            "50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859",
        )
        .unwrap();
        let typed_key: TypedPublicKey = TypedPublicKey::Sr25519(pub_key);

        let coin_infos = vec![
            (
                "12pWV6LvG4iAfNpFNTvvkWy3H9H8wtCkjiXupAzo2BCmPViM",
                CoinInfo {
                    coin: "POLKADOT".to_string(),
                    derivation_path: "//imToken//polakdot/0".to_string(),
                    curve: CurveType::SubSr25519,
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                },
            ),
            (
                "EPq15Rj2eTcyVdBBXgyWKVta7Zj4FTo7beB3YHPwtPjxEkr",
                CoinInfo {
                    coin: "KUSAMA".to_string(),
                    derivation_path: "//imToken//kusama/0".to_string(),
                    curve: CurveType::SubSr25519,
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                },
            ),
        ];
        for addr_and_coin in coin_infos {
            let addr = SubstrateAddress::from_public_key(&typed_key, &addr_and_coin.1).unwrap();
            assert_eq!(addr_and_coin.0, addr);
        }
    }

    #[test]
    fn test_address_is_valid() {
        let coin_info = CoinInfo {
            coin: "KUSAMA".to_string(),
            derivation_path: "//imToken//kusama/0".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        };
        let addresses = vec![
            "FwMF8FdFKxPtt9enzZ2Zf7dJCxiu4HqK6GhRAsKCvbNkSqx",
            "DksmaiRqSAXNqsWvGXDMdr1VqixoYUAALAgCEJ5cPYiwZeE",
            "GksmaDbSL6XX2z8VZzsiiGdEp6qZY4jKQRtTvyqu3T16cW1",
        ];
        for addr in addresses {
            assert!(SubstrateAddress::is_valid(addr, &coin_info));
        }
    }

    #[test]
    fn test_address_is_invalid() {
        let coin_info = CoinInfo {
            coin: "KUSAMA".to_string(),
            derivation_path: "//imToken//kusama/0".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        };
        let addresses = vec![
            "3BMEXohjFLZJGBLkCbF9zreee1eJjoM3ZB",
            "17A16QmavnUfCW11DAApiJxp7ARnxN5pGX",
            "0x891D85380A227e5a8443bd0f39bDedBB6DA79883",
        ];
        for addr in addresses {
            assert!(!SubstrateAddress::is_valid(addr, &coin_info));
        }
    }
}
