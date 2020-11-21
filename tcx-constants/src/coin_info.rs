use crate::curve::CurveType;
use crate::Result;
use failure::format_err;

use parking_lot::RwLock;

/// Blockchain basic config
///
/// NOTE: Unique key field is `symbol`
#[derive(Clone)]
pub struct CoinInfo {
    pub coin: String,
    pub derivation_path: String,
    pub curve: CurveType,
    pub network: String,
    pub seg_wit: String,
}

lazy_static! {
    static ref COIN_INFOS: RwLock<Vec<CoinInfo>> = {
        let mut coin_infos = Vec::new();
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/49'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/49'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/49'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/49'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "TRON".to_string(),
            derivation_path: "m/44'/195'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "m/44'/309'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "m/44'/309'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "POLKADOT".to_string(),
            derivation_path: "//polkadot//imToken/0".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "KUSAMA".to_string(),
            derivation_path: "//kusama//imToken/0".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/44'/461'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/44'/461'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/2334/461/0/0".to_string(),
            curve: CurveType::BLS,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/2334/461/0/0".to_string(),
            curve: CurveType::BLS,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });

        RwLock::new(coin_infos)
    };
}

pub fn coin_info_from_param(
    chain_type: &str,
    network: &str,
    seg_wit: &str,
    curve: &str,
) -> Result<CoinInfo> {
    let coin_infos = COIN_INFOS.read();
    let mut coins = coin_infos
        .iter()
        .filter(|x| {
            x.coin.as_str() == chain_type
                && (x.network.as_str() == network || network.is_empty())
                && (x.seg_wit.as_str() == seg_wit || seg_wit.is_empty())
                && (x.curve.as_str() == curve || curve.is_empty())
        })
        .map(|x| x.clone())
        .collect::<Vec<CoinInfo>>();

    if coins.is_empty() {
        Err(format_err!("unsupported_chain"))
    } else {
        Ok(coins.pop().expect("coin_info_from_param"))
    }
}
