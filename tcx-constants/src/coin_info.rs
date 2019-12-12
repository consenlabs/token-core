use crate::curve::CurveType;
use crate::Result;
use failure::format_err;
use serde_json::Value;
use std::sync::RwLock;

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
            coin: "NERVOS_CKB".to_string(),
            derivation_path: "m/44'/309'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "NERVOS_CKB".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });

        RwLock::new(coin_infos)
    };
}

pub fn coin_info_from_param(chain_type: &str, network: &str, seg_wit: &str) -> Result<CoinInfo> {
    let coin_infos = COIN_INFOS.read().unwrap();
    let mut coins = coin_infos
        .iter()
        .filter(|x| x.coin.as_str() == chain_type)
        .filter(|x| x.network.as_str() == network)
        .filter(|x| x.seg_wit.as_str() == seg_wit)
        .map(|x| x.clone())
        .collect::<Vec<CoinInfo>>();
    if coins.is_empty() {
        Err(format_err!("unsupported_chain"))
    } else {
        Ok(coins.pop().expect("coin_info_from_param"))
    }
}

const NETWORK_COINS: [&str; 3] = ["BITCOINCASH", "LITECOIN", "BITCOIN"];
