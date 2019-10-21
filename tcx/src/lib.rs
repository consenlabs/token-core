use std::ffi::{CStr, CString};

use std::os::raw::c_char;

use crate::error_handle::landingpad;
use crate::error_handle::Result;
use crate::error_handle::LAST_BACKTRACE;
use crate::error_handle::LAST_ERROR;
use std::fs::File;
use std::io::{Read, Write};

use core::borrow::Borrow;
use presenter::Presenter;

use serde_json::{Map, Value};
use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;
use std::str::FromStr;
use std::sync::RwLock;
use tcx_bch::{BchAddress, BchExtra, BchTransaction};
use tcx_btc_fork::address::BtcForkAddress;
use tcx_btc_fork::{
    BtcForkExtra, BtcForkSegWitTransaction, BtcForkTransaction, ExternalAddress, Utxo,
};
use tcx_chain::keystore::EmptyExtra;
use tcx_chain::signer::TransactionSigner;
use tcx_chain::{CoinInfo, HdKeystore, Metadata, TxSignResult};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_primitive::CurveType;
use tcx_tron::{TrxAddress, TrxSignedTransaction, TrxTransaction};

use std::convert::TryFrom;
use std::fs;

// #[link(name = "TrezorCrypto")]
// extern {
//     fn mnemonic_generate(strength: c_int, mnemonic: *mut c_char) -> c_int;
// }

#[macro_use]
extern crate failure;

#[macro_use]
pub mod presenter;
pub mod error_handle;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

lazy_static! {
    static ref KEYSTORE_MAP: RwLock<HashMap<String, HdKeystore>> = RwLock::new(HashMap::new());
    static ref WALLET_FILE_DIR: RwLock<String> = RwLock::new("../test-data".to_string());
}

fn cache_keystore(keystore: HdKeystore) {
    KEYSTORE_MAP
        .write()
        .unwrap()
        .insert(keystore.id.to_owned(), keystore);
}

fn _find_keystore_id_by_address(address: &str) -> Option<String> {
    let map = KEYSTORE_MAP.read().unwrap();
    let mut k_id: Option<String> = None;
    for (id, keystore) in map.borrow().iter() {
        let mut iter = keystore.active_accounts.iter();
        if iter.any(|a| a.address == address) {
            k_id = Some(id.to_string());
            break;
        }
    }
    k_id
}

fn _flush_keystore(ks: &HdKeystore) -> Result<()> {
    let json = ks.json();

    let file_dir = WALLET_FILE_DIR.read().unwrap();
    let ks_path = format!("{}/{}.json", file_dir, ks.id);
    let path = Path::new(&ks_path);
    let mut file = File::create(path)?;
    let _ = file.write_all(&json.as_bytes());
    Ok(())
}

fn _delete_keystore_file(wid: &str) -> Result<()> {
    let file_dir = WALLET_FILE_DIR.read().unwrap();
    let ks_path = format!("{}/{}.json", file_dir, wid);
    let path = Path::new(&ks_path);
    fs::remove_file(path)?;
    Ok(())
}

fn _coin_info_from_symbol(symbol: &str) -> Result<CoinInfo> {
    match symbol.to_uppercase().as_str() {
        "BITCOINCASH" => Ok(CoinInfo {
            symbol: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "BITCOINCASH-TESTNET" => Ok(CoinInfo {
            symbol: "BITCOINCASH".to_string(),
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
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "LITECOIN-TESTNET" => Ok(CoinInfo {
            symbol: "LITECOIN-TESTNET".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        }),
        "LITECOIN-TESTNET-P2WPKH" => Ok(CoinInfo {
            symbol: "LITECOIN-TESTNET-P2WPKH".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
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

fn _coin_symbol_with_network(v: &Value) -> String {
    let chain_type = v["chainType"].as_str().expect("chainType");
    if !NETWORK_COINS.contains(&chain_type) {
        return chain_type.to_string();
    }
    let mut symbol = chain_type.to_string();

    if let Some(network) = v["network"].as_str() {
        if network.to_uppercase() != "MAINNET" {
            symbol = format!("{}-{}", symbol, network);
        }
    }
    if let Some(chain_id) = v["chainId"].as_str() {
        if chain_id == "1" {
            symbol = format!("{}-TESTNET", symbol);
        }
    }

    if let Some(seg_wit) = v["segWit"].as_str() {
        if seg_wit.to_uppercase() != "NONE" {
            symbol = format!("{}-{}", symbol, seg_wit);
        }
    }
    symbol
}

#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}

#[no_mangle]
pub unsafe extern "C" fn free_const_string(s: *const c_char) {
    if s.is_null() {
        return;
    }
    CStr::from_ptr(s);
}

fn parse_arguments(json_str: *const c_char) -> Value {
    let json_c_str = unsafe { CStr::from_ptr(json_str) };
    let json_str = json_c_str.to_str().expect("parse_arguments to_str");
    serde_json::from_str(json_str).expect("parse_arguments serde_json")
}

pub extern "C" fn create_wallet(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = unsafe { landingpad(|| _create_wallet(&v)) };
    CString::new(json).expect("ret json").into_raw()
}

fn _create_wallet(v: &Value) -> Result<String> {
    let meta: Metadata = serde_json::from_value(v.clone())?;
    let password = v["password"].as_str().unwrap();
    let keystore = HdKeystore::new(password, meta);
    let _json = keystore.json();
    let _ = _flush_keystore(&keystore);
    let ret = keystore.present();
    cache_keystore(keystore);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn init_token_core_x(json_str: *const c_char) {
    let v = parse_arguments(json_str);
    // !!! warning !!! just set_panic_hook when debug
    // set_panic_hook();
    landingpad(|| _init_token_core_x(&v));
}

fn _init_token_core_x(v: &Value) -> Result<()> {
    let file_dir = v["fileDir"].as_str().expect("fileDir");
    let xpub_common_key = v["xpubCommonKey128"].as_str().expect("XPubCommonKey128");
    let xpub_common_iv = v["xpubCommonIv"].as_str().expect("xpubCommonIv");
    *WALLET_FILE_DIR.write().unwrap() = file_dir.to_string();
    *XPUB_COMMON_KEY_128.write().unwrap() = xpub_common_key.to_string();
    *XPUB_COMMON_IV.write().unwrap() = xpub_common_iv.to_string();

    let p = Path::new(file_dir);
    let walk_dir = std::fs::read_dir(p).expect("read dir");
    for entry in walk_dir {
        let entry = entry.expect("DirEntry");
        let fp = entry.path();
        if !fp
            .file_name()
            .expect("file_name")
            .to_str()
            .expect("file_name str")
            .ends_with(".json")
        {
            continue;
        }

        let mut f = File::open(fp).expect("open file");
        let mut contents = String::new();

        let _ = f.read_to_string(&mut contents);
        let v: Value = serde_json::from_str(&contents).expect("read json from content");

        let version = v["version"].as_i64().expect("version");
        if version != i64::from(HdKeystore::VERSION) {
            continue;
        }
        let keystore: HdKeystore = serde_json::from_str(&contents)?;
        cache_keystore(keystore);
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn find_wallet_by_mnemonic(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _find_wallet_by_mnemonic(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _find_wallet_by_mnemonic(v: &Value) -> Result<String> {
    let mnemonic = v["mnemonic"].as_str().unwrap();
    let path = v["path"].as_str().unwrap();
    let symbol = _coin_symbol_with_network(v);

    let mut coin_info = _coin_info_from_symbol(&symbol)?;
    coin_info.derivation_path = path.to_string();
    let acc = match symbol.as_str() {
        "BITCOINCASH" => {
            HdKeystore::mnemonic_to_account::<BchAddress, BchExtra>(&coin_info, mnemonic)
        }
        "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
            HdKeystore::mnemonic_to_account::<BtcForkAddress, BtcForkExtra>(&coin_info, mnemonic)
        }
        "TRON" => HdKeystore::mnemonic_to_account::<TrxAddress, EmptyExtra>(&coin_info, mnemonic),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;
    let address = acc.address;
    let kid = _find_keystore_id_by_address(&address);
    if let Some(id) = kid {
        let map = KEYSTORE_MAP.read().unwrap();
        let ks: &HdKeystore = map.get(&id).unwrap();
        ks.present()
    } else {
        Ok("{}".to_owned())
    }
}

#[no_mangle]
pub unsafe extern "C" fn import_wallet_from_mnemonic(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _import_wallet_from_mnemonic(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _import_wallet_from_mnemonic(v: &Value) -> Result<String> {
    let password = v["password"].as_str().unwrap();
    let mnemonic = v["mnemonic"].as_str().unwrap();
    let path = v["path"].as_str().unwrap();
    let overwrite = v["overwrite"].as_bool().unwrap();
    let symbol = _coin_symbol_with_network(v);

    let meta: Metadata = serde_json::from_value(v.clone())?;
    let mut ks = HdKeystore::from_mnemonic(mnemonic, password, meta);
    let _pw = Map::new();

    let mut coin_info = _coin_info_from_symbol(&symbol)?;
    coin_info.derivation_path = path.to_string();
    let account = match symbol.as_str() {
        "BITCOINCASH" => ks.derive_coin::<BchAddress, BchExtra>(&coin_info, password),
        "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
            ks.derive_coin::<BtcForkAddress, BtcForkExtra>(&coin_info, password)
        }
        "TRON" => ks.derive_coin::<TrxAddress, EmptyExtra>(&coin_info, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;

    let exist_kid_opt = _find_keystore_id_by_address(&account.address);
    if let Some(exist_kid) = exist_kid_opt {
        if !overwrite {
            return Err(format_err!("{}", "wallet_exists"));
        } else {
            ks.id = exist_kid;
        }
    }

    _flush_keystore(&ks)?;
    let json = ks.present();
    cache_keystore(ks);

    json
}

#[no_mangle]
pub unsafe extern "C" fn export_mnemonic(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = landingpad(|| _export_mnemonic(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _export_mnemonic(v: &Value) -> Result<String> {
    let wid = v["id"].as_str().expect("id");
    let password = v["password"].as_str().expect("password");

    let map = KEYSTORE_MAP.read().unwrap();
    let keystore = match map.get(wid) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;
    let mnemonic = keystore.mnemonic(password)?;
    Ok(serde_json::to_string(
        &json!({"ok": true, "mnemonic": mnemonic}),
    )?)
}

#[no_mangle]
pub unsafe extern "C" fn verify_password(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = landingpad(|| _verify_password(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _verify_password(v: &Value) -> Result<String> {
    let wid = v["id"].as_str().expect("id");
    let password = v["password"].as_str().expect("password");

    let map = KEYSTORE_MAP.read().unwrap();
    let keystore: &HdKeystore = match map.get(wid) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;
    if !keystore.verify_password(password) {
        Err(format_err!("{}", "password_incorrect"))
    } else {
        Ok(serde_json::to_string(
            &json!({"ok": true, "id": wid.to_string()}),
        )?)
    }
}

#[no_mangle]
pub unsafe extern "C" fn sign_transaction(json_str: *const c_char) -> *const c_char {
    let json_c_str = CStr::from_ptr(json_str);
    let json_str = json_c_str.to_str().unwrap();

    let json = landingpad(|| _sign_transaction(json_str));
    CString::new(json).expect("ret json").into_raw()
}

fn _sign_transaction(json_str: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json_str).unwrap();
    let w_id = v["id"].as_str().expect("wid");
    let password = v["password"].as_str().expect("password");
    let symbol = _coin_symbol_with_network(&v);

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    match symbol.as_str() {
        "BITCOINCASH"
        | "LITECOIN"
        | "LITECOIN-P2WPKH"
        | "LITECOIN-TESTNET"
        | "LITECOIN-TESTNET-P2WPKH" => {
            _sign_btc_fork_transaction(json_str, &symbol, keystore, password)
        }
        "TRON" => _sign_trx_transaction(json_str, keystore, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }
}

fn _sign_btc_fork_transaction(
    json: &str,
    coin: &str,
    keystore: &HdKeystore,
    password: &str,
) -> Result<String> {
    let v: Value = serde_json::from_str(json).expect("sign_transaction_json");
    let utxos = v["outputs"].as_array().expect("expect outputs");

    let unspents: Vec<Utxo> = utxos
        .iter()
        .map(|v| Utxo {
            tx_hash: v["txHash"].as_str().expect("utxo txHash").to_string(),
            vout: v["vout"].as_f64().expect("utxo vout") as i32,
            amount: v["amount"]
                .as_str()
                .expect("utxo amount")
                .to_string()
                .parse::<i64>()
                .expect("utxo converter amount to i64"),
            address: v["address"].as_str().expect("utxo address").to_string(),
            script_pub_key: v["scriptPubKey"]
                .as_str()
                .expect("utxo scriptPubKey")
                .to_string(),
            derived_path: v["derivedPath"]
                .as_str()
                .expect("utxo derivedPath")
                .to_string(),
            sequence: 0,
        })
        .collect();
    let internal_used = v["internalUsed"].as_i64();
    let change_address = v["changeAddress"].as_str();
    let to = v["to"].as_str().expect("to");
    let seg_wit = v["segWit"].as_str().expect("segWit");
    let is_seg_wit = seg_wit == "P2WPKH";
    let amount = v["amount"]
        .as_str()
        .expect("amount")
        .parse::<i64>()
        .unwrap();
    let fee = v["fee"].as_str().expect("fee").parse::<i64>().unwrap();
    let ret: TxSignResult;
    if coin.starts_with("BITCOINCASH") {
        let tran = BchTransaction::new(
            to.to_owned(),
            amount,
            unspents,
            fee,
            internal_used.map(|x| (x + 1) as u32),
            change_address.map(str::to_string),
            coin.to_string(),
        );
        ret = keystore.sign_transaction(&tran, Some(&password))?;
    } else if is_seg_wit {
        let tran = BtcForkSegWitTransaction::new(
            to.to_owned(),
            amount,
            unspents,
            fee,
            internal_used.map(|x| (x + 1) as u32),
            change_address.map(str::to_string),
            coin.to_string(),
        );
        ret = keystore.sign_transaction(&tran, Some(&password))?;
    } else {
        let tran = BtcForkTransaction::new(
            to.to_owned(),
            amount,
            unspents,
            fee,
            internal_used.map(|x| (x + 1) as u32),
            change_address.map(str::to_string),
            coin.to_string(),
        );

        ret = keystore.sign_transaction(&tran, Some(&password))?;
    }

    Ok(serde_json::to_string(&ret)?)
}

fn _sign_trx_transaction(json: &str, keystore: &HdKeystore, password: &str) -> Result<String> {
    let v = Value::from_str(json)?;
    let tx = TrxTransaction::try_from(v)?;
    let signed: TrxSignedTransaction = keystore.sign_transaction(&tx, Some(password))?;
    let signed_v: Value = signed.try_into()?;
    Ok(signed_v.to_string())
}

#[no_mangle]
pub unsafe extern "C" fn calc_external_address(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _calc_external_address(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _calc_external_address(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let external_id = v["externalIdx"].as_i64().expect("external_id");
    let _network = v["network"].as_str().unwrap();
    let chain_type = v["chainType"].as_str().unwrap();

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let account = keystore
        .account(&chain_type)
        .ok_or_else(|| format_err!("account_not_found, chainType: {}", &chain_type))?;
    let external_addr: ExternalAddress;
    if chain_type.starts_with("BITCOINCASH") {
        let extra = BchExtra::from(account.extra.clone());
        external_addr = extra.calc_external_address(external_id, &chain_type)?;
    } else {
        let extra = BtcForkExtra::from(account.extra.clone());
        external_addr = extra.calc_external_address(external_id, &chain_type)?;
    }

    Ok(serde_json::to_string(&external_addr)?)
}

#[no_mangle]
pub unsafe extern "C" fn remove_wallet(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _remove_wallet(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _remove_wallet(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let password = v["password"].as_str().expect("password");

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore: &HdKeystore = match map.get(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    if keystore.verify_password(password) {
        _delete_keystore_file(w_id)?;
        map.remove(w_id);
        Ok(serde_json::to_string(&json!({ "id": w_id }))?)
    } else {
        Err(format_err!("{}", "password_incorrect"))
    }
}

// get_derived_key and cache_derived_key functions are one way to speed decrypt data,
// you should cache the derived_key in some secure place like keystore in iOS, and protect it by biometric.
#[no_mangle]
pub unsafe extern "C" fn get_derived_key(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _get_derived_key(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _get_derived_key(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let password = v["password"].as_str().expect("password");

    let map = KEYSTORE_MAP.read().unwrap();
    let keystore: &HdKeystore = match map.get(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let derived_key = keystore.crypto.generate_derived_key(password)?;

    Ok(hex::encode(derived_key))
}

#[no_mangle]
pub unsafe extern "C" fn verify_derived_key(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _verify_derived_key(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _verify_derived_key(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let derived_key = v["derivedKey"].as_str().expect("derivedKey");

    let map = KEYSTORE_MAP.read().unwrap();
    let keystore: &HdKeystore = match map.get(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;
    let derived_key_bytes: Vec<u8> = hex::decode(derived_key)?;
    if !keystore.crypto.verify_derived_key(&derived_key_bytes) {
        Err(format_err!("{}", "invalid_cached_derived_key"))
    } else {
        Ok(serde_json::to_string(
            &json!({ "id": w_id, "derivedKey": derived_key }),
        )?)
    }
}

#[no_mangle]
pub unsafe extern "C" fn cache_derived_key(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| _cache_derived_key(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn _cache_derived_key(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let derived_key = v["derivedKey"].as_str().expect("derivedKey");
    let tmp_password = v["tempPassword"].as_str().expect("tempPassword");

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore: &mut HdKeystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;
    let derived_key_bytes: Vec<u8> = hex::decode(derived_key)?;
    if !keystore.crypto.verify_derived_key(&derived_key_bytes) {
        Err(format_err!("{}", "invalid_cached_derived_key"))
    } else {
        keystore
            .crypto
            .cache_derived_key(tmp_password, &derived_key_bytes);
        Ok(serde_json::to_string(
            &json!({ "id": w_id, "derivedKey": derived_key }),
        )?)
    }
}

#[no_mangle]
pub unsafe extern "C" fn clear_derived_key() -> *const c_char {
    //    let v = parse_arguments(json_str);
    let json = landingpad(_clear_derived_key);
    CString::new(json).expect("ret json").into_raw()
}

fn _clear_derived_key() -> Result<String> {
    let map: &mut HashMap<String, HdKeystore> = &mut KEYSTORE_MAP.write().unwrap();
    map.values_mut()
        .map(|keystore| {
            keystore.crypto.clear_cache_derived_key();
        })
        .collect::<()>();
    Ok(serde_json::to_string(&json!({ "ok": true }))?)
}

#[no_mangle]
pub unsafe extern "C" fn clear_err() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
    LAST_BACKTRACE.with(|e| {
        *e.borrow_mut() = None;
    });
}

#[no_mangle]
pub unsafe extern "C" fn get_last_err_message() -> *const c_char {
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            let msg = err.to_string();
            // todo: follow cause
            //            let mut cause = err.cause();
            //            while let Some(the_cause) = cause {
            //                write!(&mut msg, "\n  caused by: {}", the_cause).ok();
            //                cause = &the_cause.cause();
            //            }
            CString::new(msg).unwrap().into_raw()
        } else {
            CString::new("").unwrap().into_raw()
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        cache_derived_key, clear_derived_key, clear_err, export_mnemonic, get_derived_key,
        get_last_err_message, remove_wallet, sign_transaction,
    };
    use crate::{
        create_wallet, find_wallet_by_mnemonic, import_wallet_from_mnemonic, init_token_core_x,
    };
    use crate::{KEYSTORE_MAP, WALLET_FILE_DIR};
    use serde_json::Value;
    use std::ffi::{CStr, CString};
    use std::fs::remove_file;
    use std::os::raw::c_char;
    use std::panic;
    use std::path::Path;
    use std::str::FromStr;

    use tcx_chain::HdKeystore;

    static WALLET_ID: &'static str = "9c6cbc21-1c43-4c8b-bb7a-5e538f908819";

    fn _to_c_char(str: &str) -> *const c_char {
        CString::new(str).unwrap().into_raw()
    }

    fn _to_str(json_str: *const c_char) -> &'static str {
        let json_c_str = unsafe { CStr::from_ptr(json_str) };
        json_c_str.to_str().unwrap()
    }

    fn setup() {
        let init_params = r#"
        {
            "fileDir": "../test-data",
            "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
            "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
        }
        "#;
        unsafe {
            init_token_core_x(_to_c_char(init_params));
        }
    }

    #[allow(dead_code)]
    fn teardown() {
        let file_dir = WALLET_FILE_DIR.read().unwrap();
        let file_dir_str = file_dir.to_string();
        let p = Path::new(&file_dir_str);
        let walk_dir = std::fs::read_dir(p).unwrap();
        for entry in walk_dir {
            let entry = entry.unwrap();
            let fp = entry.path();
            let file_name = fp.file_name().unwrap();
            if file_name != ".gitignore" && file_name != "default_keystore.json" {
                let _ = remove_file(fp);
            }
        }
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();
        let result = panic::catch_unwind(|| test());
        //        teardown();
        assert!(result.is_ok())
    }

    fn remove_created_wallet(wid: &str) {
        let file_dir = WALLET_FILE_DIR.read().unwrap();
        let _file_dir_str = file_dir.to_string();
        let full_file_path = format!("{}/{}.json", file_dir, wid);
        let p = Path::new(&full_file_path);
        remove_file(p);
    }

    #[test]
    fn init_token_core_x_test() {
        run_test(|| {
            let init_params = r#"
        {
            "fileDir": "../test-data",
            "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
            "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
        }
        "#;
            unsafe {
                init_token_core_x(_to_c_char(init_params));
            }

            let map = KEYSTORE_MAP.read().unwrap();
            let ks: &HdKeystore = map.get(WALLET_ID).unwrap();
            assert_eq!(ks.id, WALLET_ID);
        });
    }

    #[test]
    fn create_wallet_test() {
        run_test(|| {
            let params = r#"
        {
            "name": "createWalletTest",
            "password": "Insecure Password",
            "passwordHint": "Insecure Password",
            "source": "MNEMONIC"
        }
        "#;
            let json = _to_str(create_wallet(_to_c_char(params)));
            let v = Value::from_str(json).unwrap();
            let _expected = Value::from_str(params).unwrap();
            let id = v["id"].as_str().expect("wallet_id");
            assert_eq!(v["source"].as_str().unwrap(), "MNEMONIC");
            let map = KEYSTORE_MAP.read().unwrap();
            assert!(map.get(id).is_some());
            remove_created_wallet(id);
        })
    }

    #[test]
    fn find_wallet_by_mnemonic_test() {
        run_test(|| {
            let param = r#"{"chainType":"BITCOINCASH","mnemonic":"blind gravity card grunt basket expect garment tilt organ concert great critic","network":"MAINNET","path":"m/44'/145'/0'/0/0","segWit":"NONE"}"#;
            let ret = unsafe { _to_str(find_wallet_by_mnemonic(_to_c_char(param))) };
            assert_eq!("{}", ret);

            let param = r#"{"chainType":"BITCOINCASH","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","network":"MAINNET","path":"m/44'/145'/0'/0/0","segWit":"NONE"}"#;
            let ret = unsafe { _to_str(find_wallet_by_mnemonic(_to_c_char(param))) };
            let v = Value::from_str(ret).expect("find wallet");
            assert_eq!(
                v["address"],
                "bitcoincash:qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r"
            );
        })
    }

    #[test]
    fn import_wallet_from_mnemonic_test() {
        run_test(|| {
            let param = r#"{"chainType":"BITCOINCASH","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"BCH-Wallet-1","network":"MAINNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/145'/0'/0/0","segWit":"NONE","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let expected = r#"
            {
                "address": "bitcoincash:qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
                "chainType": "BITCOINCASH",
                "createdAt": 1566455834,
                "encXPub": "wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==",
                "externalAddress": {
                    "address": "bitcoincash:qzyrtfn4a7cdkn7sp60tw7hl8zndt0tk0sst3p6qr5",
                    "derivedPath": "0/1",
                    "type": "EXTERNAL"
                },
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "BCH-Wallet-1",
                "passwordHint": "",
                "source": "MNEMONIC"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            let ret_v = Value::from_str(ret).unwrap();
            assert_eq!(expected_v["address"], ret_v["address"]);
            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);

            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
        });
    }

    #[test]
    fn import_ltc_wallet_from_mnemonic_test() {
        run_test(|| {
            let param = r#"{"chainType":"LITECOIN","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"LTC-Wallet-1","network":"TESTNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/2'/0'/0/0","segWit":"P2WPKH","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let expected = r#"
            {
                "address": "QLfctE6KMv3ZzQod6UA37w3EPTuLS4tg1T",
                "chainType": "LITECOIN",
                "createdAt": 1566455834,
                "encXPub": "k4GbrxWCcsrGokCos50O69Wg9reixsDqPHkciU4xeUi9dpICotcOMQSgTgRd7XtGXXjdV/SUuTBkPXNQikqORvvW2CnHNe7+iJsTdHebynq2Y3ZXMFUWt8WJkgB5NotqkjOik89LvJBKYKvnon2B0g==",
                "externalAddress": {
                    "address": "QPvKbnvZxAF1KVk5LfXbqtfnkwTymMf2Xu",
                    "derivedPath": "0/1",
                    "type": "EXTERNAL"
                },
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "LTC-Wallet-1",
                "passwordHint": "",
                "source": "MNEMONIC"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            let ret_v = Value::from_str(ret).unwrap();

            assert_eq!(expected_v["address"], ret_v["address"]);
            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);

            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
        });
    }

    #[test]
    fn import_legacy_ltc_wallet_from_mnemonic_mainnet() {
        run_test(|| {
            let param = r#"{"chainType":"LITECOIN","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"LTC-Wallet-1","network":"MAINNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/2'/0'/0/0","segWit":"NONE","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let expected = r#"
            {
                "address": "Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP",
                "chainType": "LITECOIN",
                "createdAt": 1566455834,
                "encXPub": "MwDMFXVWDEuWvBogeW1v/MOMFDnGnnflm2JAPvJaJZO4HXp8fCsWETA7u8MzOW3KaPksglpUHLN3xkDr2QWMEQq0TewFZoZ3KsjmLW0KGMRN7XQKqo/omkSEsPfalVnp9Zxm2lpxVmIacqvlernVSg==",
                "externalAddress": {
                    "address": "LavE5eHDvw9VDiNifbraR7GyY8MRvcQSLQ",
                    "derivedPath": "0/1",
                    "type": "EXTERNAL"
                },
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "LTC-Wallet-1",
                "passwordHint": "",
                "source": "MNEMONIC"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            let ret_v = Value::from_str(ret).unwrap();

            assert_eq!(expected_v["address"], ret_v["address"]);
            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);

            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
        });
    }

    #[test]
    fn import_legacy_ltc_wallet_from_mnemonic_test() {
        run_test(|| {
            let param = r#"{"chainType":"LITECOIN","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"LTC-Wallet-1","network":"TESTNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/1'/0'/0/0","segWit":"NONE","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let expected = r#"
            {
                "address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
                "chainType": "LITECOIN",
                "createdAt": 1566455834,
                "encXPub": "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==",
                "externalAddress": {
                    "address": "mj78AbVtQ9SWnvbU7pcrueyE1krMmZtoUU",
                    "derivedPath": "0/1",
                    "type": "EXTERNAL"
                },
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "LTC-Wallet-1",
                "passwordHint": "",
                "source": "MNEMONIC"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            let ret_v = Value::from_str(ret).unwrap();

            assert_eq!(expected_v["address"], ret_v["address"]);
            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);

            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
        });
    }

    #[test]
    fn remove_wallet_test() {
        run_test(|| {
            let param = r#"{"chainType":"LITECOIN","mnemonic":"calm release clay imitate top extend close draw quiz refuse shuffle injury","name":"LTC-Wallet-1","network":"MAINNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/1'/0'/0/0","segWit":"NONE","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let ret_v = Value::from_str(ret).unwrap();
            let imported_id = ret_v["id"].as_str().expect("wallet_id");
            let param = json!({
                "id": imported_id,
                "password": "Insecure Password"
            });
            let param = serde_json::to_string(&param).unwrap();
            let ret = unsafe { _to_str(remove_wallet(_to_c_char(&param))) };
            let ret_v = Value::from_str(ret).unwrap();
            assert_eq!(ret_v["id"], imported_id);

            //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
        });
    }

    #[test]
    fn import_trx_wallet_from_mnemonic_test() {
        run_test(|| {
            let param = r#"{"chainType":"TRON","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"TRX-Wallet-1","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/195'/0'/0/0","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let expected = r#"
            {
                "address": "TY2uroBeZ5trA9QT96aEWj32XLkAAhQ9R2",
                "chainType": "TRON",
                "createdAt": 1566455834,
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "LTC-Wallet-1",
                "passwordHint": "",
                "source": "MNEMONIC"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            let ret_v = Value::from_str(ret).unwrap();

            assert_eq!(expected_v["address"], ret_v["address"]);
            assert_eq!(expected_v["chainType"], ret_v["chainType"]);

            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
        });
    }

    #[test]
    fn export_mnemonic_test() {
        run_test(|| {
            let param = r#"
        {
            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
            "password": "Insecure Password"
        }
        "#;
            unsafe { clear_err() }
            let exported_mnemonic = unsafe { _to_str(export_mnemonic(_to_c_char(param))) };
            let _err = unsafe { _to_str(get_last_err_message()) };
            let expected_v = Value::from_str(r#"{"mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","ok":true}"#).unwrap();
            let actual_v = Value::from_str(exported_mnemonic).unwrap();
            assert_eq!(actual_v, expected_v);

            let param = r#"
        {
            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
            "password": "Wrong Password"
        }
        "#;
            unsafe { clear_err() }
            let _exported_mnemonic = unsafe { _to_str(export_mnemonic(_to_c_char(param))) };
            let err = unsafe { _to_str(get_last_err_message()) };
            assert_eq!(err, "invalid_password");
        })
    }

    #[test]
    fn sign_transaction_test() {
        run_test(|| {
            let param = r#"
            {
                "id":"9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
                "password": "Insecure Password",
                "to": "bitcoincash:qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s",
                "amount": "93454",
                "fee": "6000",
                "internalUsed": 0,
                "chainType": "BITCOINCASH",
                "chainId": "145",
                "segWit":"NONE",
                "outputs": [
                    {
                        "txHash": "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2",
                        "vout": 0,
                        "amount": "100000",
                        "address": "bitcoincash:qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
                        "scriptPubKey": "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac",
                        "derivedPath": "0/0"
                    }
                ]
            }
            "#;

            unsafe { clear_err() }
            let ret = unsafe { _to_str(sign_transaction(_to_c_char(param))) };
            let ret_v = Value::from_str(ret).unwrap();
            let expected = r#"{"sign":"0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006b483045022100b3d91f406cdc33eb4d8f2b56491e6c87da2372eb83f1f384fc3f02f81a5b21b50220324dd7ecdc214721c542db252078473f9e7172bf592fa55332621c3e348be45041210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a9148835a675efb0db4fd00e9eb77aff38a6d5bd767c88ac00000000","hash":"4d43cc66e9763a4e263fdb592591b9f19a6915ac821c92896d13f95beaca3b28","wtxId":""}"#;
            let expected_v = Value::from_str(expected).unwrap();
            assert_eq!(ret_v, expected_v);
        })
    }

    #[test]
    fn cache_derived_key_test() {
        run_test(|| {
            let param = r#"{"chainType":"LITECOIN","mnemonic":"salute slush now script nest law admit achieve voice soda fruit field","name":"LTC-Wallet-1","network":"MAINNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/1'/0'/0/0","segWit":"NONE","source":"MNEMONIC"}"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let ret_v = Value::from_str(ret).unwrap();
            let imported_id = ret_v["id"].as_str().expect("wallet_id");
            let param = json!({
                "id": imported_id,
                "password": "Insecure Password"
            });

            let derived_key =
                unsafe { _to_str(get_derived_key(_to_c_char(param.to_string().as_str()))) };
            let param: Value =
                json!({"id": imported_id, "tempPassword": "88888888", "derivedKey": derived_key});
            unsafe { _to_str(cache_derived_key(_to_c_char(param.to_string().as_str()))) };

            let param = json!({
                "id": imported_id,
                "password": "888888"
            });

            unsafe {
                clear_err();
            }
            let _ = unsafe { export_mnemonic(_to_c_char(param.to_string().as_str())) };
            let err = unsafe { _to_str(get_last_err_message()) };
            assert_eq!("invalid_password", err);

            let param = json!({
                "id": imported_id,
                "password": "88888888"
            });

            unsafe {
                clear_err();
            }
            let exported_mnemonic =
                unsafe { _to_str(export_mnemonic(_to_c_char(param.to_string().as_str()))) };
            assert_eq!(
                r#"{"mnemonic":"salute slush now script nest law admit achieve voice soda fruit field","ok":true}"#,
                exported_mnemonic
            );
            unsafe { clear_derived_key() };

            remove_created_wallet(imported_id);
        })
    }

    #[test]
    fn sign_transaction_ltc_legacy_change_address() {
        run_test(|| {
            //            let param = r#"{"chainType":"LITECOIN","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"LTC-Wallet-1","network":"MAINNET","overwrite":true,"password":"Insecure Password","passwordHint":"","path":"m/44'/1'/0'/0/0","segWit":"NONE","source":"MNEMONIC"}"#;
            //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let param = r#"
            {
                "id":"9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
                "password": "Insecure Password",
                "to": "mrU9pEmAx26HcbKVrABvgL7AwA5fjNFoDc",
                "amount": "500000",
                "fee": "100000",
                "chainType": "LITECOIN",
                "chainId": "1",
                "segWit":"NONE",
                "changeAddress": "mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1",
                "outputs": [
                    {
                        "txHash": "a477af6b2667c29670467e4e0728b685ee07b240235771862318e29ddbe58458",
                        "vout": 0,
                        "amount": "1000000",
                        "address": "mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1",
                        "scriptPubKey": "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac",
                        "derivedPath": "0/0"
                    }
                ]
            }
            "#;

            unsafe { clear_err() }
            let ret = unsafe { _to_str(sign_transaction(_to_c_char(param))) };
            //            unsafe { assert_eq!("", _to_str(get_last_err_message()));}
            assert_eq!("", ret);
            //            let ret_v = Value::from_str(ret).unwrap();
            //            let expected = r#"{"sign":"01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006b483045022100eefdd6cace70ee64d6a29bca5f52c338b2b3ecf6e6c7b222818c9bba60f094fb022053535e23a77afc7255c18ae8c6e6bf0f8b6e3f552d08519455714cbe59e489cf01210223078d2942df62c45621d209fab84ea9a7a23346201b7727b9b45a29c4e76f5effffffff0220a10700000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac801a0600000000001976a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac00000000","hash":"4d43cc66e9763a4e263fdb592591b9f19a6915ac821c92896d13f95beaca3b28","wtxId":""}"#;
            //            let expected_v = Value::from_str(expected).unwrap();
            //            assert_eq!(ret_v, expected_v);
        })
    }
}
