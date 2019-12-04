use crate::curve::CurveType;
use crate::Result;
use failure::format_err;
use serde_json::Value;

/// Blockchain basic config
///
/// NOTE: Unique key field is `symbol`
pub struct CoinInfo {
    pub symbol: String,
    pub derivation_path: String,
    pub curve: CurveType,
}

pub fn coin_info_from_symbol(symbol: &str) -> Result<CoinInfo> {
    match symbol.to_uppercase().as_str() {
        "BITCOINCASH" => Ok(CoinInfo {
            symbol: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "BITCOINCASH-TESTNET" => Ok(CoinInfo {
            symbol: "BITCOINCASH-TESTNET".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "LITECOIN" => Ok(CoinInfo {
            symbol: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "LITECOIN-P2WPKH" => Ok(CoinInfo {
            symbol: "LITECOIN-P2WPKH".to_string(),
            derivation_path: "m/49'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "LITECOIN-TESTNET" => Ok(CoinInfo {
            symbol: "LITECOIN-TESTNET".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "LITECOIN-TESTNET-P2WPKH" => Ok(CoinInfo {
            symbol: "LITECOIN-TESTNET-P2WPKH".to_string(),
            derivation_path: "m/49'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "TRON" => Ok(CoinInfo {
            symbol: "TRON".to_string(),
            derivation_path: "m/44'/195'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        _ => Err(format_err!("unsupported_chain")),
    }
}

const NETWORK_COINS: [&str; 3] = ["BITCOINCASH", "LITECOIN", "BITCOIN"];

// todo: remove
pub fn coin_symbol_with_param(
    chain_type: &str,
    network: &str,
    chain_id: &str,
    seg_wit: &str,
) -> String {
    //    let chain_type = v["chainType"].as_str().expect("chainType");
    if !NETWORK_COINS.contains(&chain_type) {
        return chain_type.to_string();
    }
    let mut symbol = chain_type.to_string();

    if !network.is_empty() {
        if &network.to_uppercase() != "MAINNET" {
            symbol = format!("{}-{}", symbol, network.to_uppercase());
        }
    }
    if !chain_id.is_empty() {
        if chain_id == "1" {
            symbol = format!("{}-TESTNET", symbol.to_uppercase());
        }
    }

    if !seg_wit.is_empty() {
        if &seg_wit.to_uppercase() != "NONE" {
            symbol = format!("{}-{}", symbol, seg_wit.to_uppercase());
        }
    }
    symbol
}

pub fn coin_symbol_with_network(v: &Value) -> String {
    let chain_type = v["chainType"].as_str().expect("chainType");
    if !NETWORK_COINS.contains(&chain_type) {
        return chain_type.to_string();
    }
    let mut symbol = chain_type.to_string();

    if let Some(network) = v["network"].as_str() {
        if network.to_uppercase() != "MAINNET" {
            symbol = format!("{}-{}", symbol, network.to_uppercase());
        }
    }
    if let Some(chain_id) = v["chainId"].as_str() {
        if chain_id == "1" {
            symbol = format!("{}-TESTNET", symbol.to_uppercase());
        }
    }

    if let Some(seg_wit) = v["segWit"].as_str() {
        if seg_wit.to_uppercase() != "NONE" {
            symbol = format!("{}-{}", symbol, seg_wit.to_uppercase());
        }
    }
    symbol
}
