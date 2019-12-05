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

        RwLock::new(coin_infos)
    };
}

//fn all_coin_infos() -> Vec<CoinInfo> {
//
//    //        "BITCOINCASH" => Ok(CoinInfo {
////            symbol: "BITCOINCASH".to_string(),
////            derivation_path: "m/44'/145'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
////        "BITCOINCASH-TESTNET" => Ok(CoinInfo {
////            symbol: "BITCOINCASH-TESTNET".to_string(),
////            derivation_path: "m/44'/1'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
////        "LITECOIN" => Ok(CoinInfo {
////            symbol: "LITECOIN".to_string(),
////            derivation_path: "m/44'/2'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
////        "LITECOIN-P2WPKH" => Ok(CoinInfo {
////            symbol: "LITECOIN-P2WPKH".to_string(),
////            derivation_path: "m/49'/2'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
////        "LITECOIN-TESTNET" => Ok(CoinInfo {
////            symbol: "LITECOIN-TESTNET".to_string(),
////            derivation_path: "m/44'/1'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
////        "LITECOIN-TESTNET-P2WPKH" => Ok(CoinInfo {
////            symbol: "LITECOIN-TESTNET-P2WPKH".to_string(),
////            derivation_path: "m/49'/1'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
////        "TRON" => Ok(CoinInfo {
////            symbol: "TRON".to_string(),
////            derivation_path: "m/44'/195'/0'/0/0".to_string(),
////            curve: CurveType::SECP256k1,
////        }),
//}

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

//pub fn coin_info_from_symbol(symbol: &str) -> Result<CoinInfo> {
//    match symbol.to_uppercase().as_str() {
//        "BITCOINCASH" => Ok(CoinInfo {
//            symbol: "BITCOINCASH".to_string(),
//            derivation_path: "m/44'/145'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        "BITCOINCASH-TESTNET" => Ok(CoinInfo {
//            symbol: "BITCOINCASH-TESTNET".to_string(),
//            derivation_path: "m/44'/1'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        "LITECOIN" => Ok(CoinInfo {
//            symbol: "LITECOIN".to_string(),
//            derivation_path: "m/44'/2'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        "LITECOIN-P2WPKH" => Ok(CoinInfo {
//            symbol: "LITECOIN-P2WPKH".to_string(),
//            derivation_path: "m/49'/2'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        "LITECOIN-TESTNET" => Ok(CoinInfo {
//            symbol: "LITECOIN-TESTNET".to_string(),
//            derivation_path: "m/44'/1'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        "LITECOIN-TESTNET-P2WPKH" => Ok(CoinInfo {
//            symbol: "LITECOIN-TESTNET-P2WPKH".to_string(),
//            derivation_path: "m/49'/1'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        "TRON" => Ok(CoinInfo {
//            symbol: "TRON".to_string(),
//            derivation_path: "m/44'/195'/0'/0/0".to_string(),
//            curve: CurveType::SECP256k1,
//        }),
//        _ => Err(format_err!("unsupported_chain")),
//    }
//}

const NETWORK_COINS: [&str; 3] = ["BITCOINCASH", "LITECOIN", "BITCOIN"];

//// todo: remove
//pub fn coin_symbol_with_param(
//    chain_type: &str,
//    network: &str,
//    chain_id: &str,
//    seg_wit: &str,
//) -> String {
//    //    let chain_type = v["chainType"].as_str().expect("chainType");
//    if !NETWORK_COINS.contains(&chain_type) {
//        return chain_type.to_string();
//    }
//    let mut symbol = chain_type.to_string();
//
//    if !network.is_empty() {
//        if &network.to_uppercase() != "MAINNET" {
//            symbol = format!("{}-{}", symbol, network.to_uppercase());
//        }
//    }
//    if !chain_id.is_empty() {
//        if chain_id == "1" {
//            symbol = format!("{}-TESTNET", symbol.to_uppercase());
//        }
//    }
//
//    if !seg_wit.is_empty() {
//        if &seg_wit.to_uppercase() != "NONE" {
//            symbol = format!("{}-{}", symbol, seg_wit.to_uppercase());
//        }
//    }
//    symbol
//}
//
//pub fn coin_symbol_with_network(v: &Value) -> String {
//    let chain_type = v["chainType"].as_str().expect("chainType");
//    if !NETWORK_COINS.contains(&chain_type) {
//        return chain_type.to_string();
//    }
//    let mut symbol = chain_type.to_string();
//
//    if let Some(network) = v["network"].as_str() {
//        if network.to_uppercase() != "MAINNET" {
//            symbol = format!("{}-{}", symbol, network.to_uppercase());
//        }
//    }
//    if let Some(chain_id) = v["chainId"].as_str() {
//        if chain_id == "1" {
//            symbol = format!("{}-TESTNET", symbol.to_uppercase());
//        }
//    }
//
//    if let Some(seg_wit) = v["segWit"].as_str() {
//        if seg_wit.to_uppercase() != "NONE" {
//            symbol = format!("{}-{}", symbol, seg_wit.to_uppercase());
//        }
//    }
//    symbol
//}
