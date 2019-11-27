use std::ffi::{CStr, CString};

use std::os::raw::c_char;

use crate::error_handle::landingpad;
use crate::error_handle::Result;
use crate::error_handle::LAST_BACKTRACE;
use crate::error_handle::LAST_ERROR;
use std::fs::File;
use std::io::{Read, Write};

use core::borrow::Borrow;

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
use tcx_chain::{HdKeystore, MessageSigner, Metadata, Source, TxSignResult};
use tcx_constants::coin_info::{coin_info_from_symbol, coin_symbol_with_network};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_tron::{TrxAddress, TrxMessage, TrxSignedTransaction, TrxTransaction};

use std::convert::TryFrom;
use std::fs;
use tcx_constants::network_from_coin;
use tcx_primitive::{verify_wif, Secp256k1PrivateKey};

#[macro_use]
extern crate failure;

#[macro_use]
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

fn find_keystore_id_by_address(address: &str) -> Option<String> {
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

fn flush_keystore(ks: &HdKeystore) -> Result<()> {
    let json = ks.json();

    let file_dir = WALLET_FILE_DIR.read().unwrap();
    let ks_path = format!("{}/{}.json", file_dir, ks.id);
    let path = Path::new(&ks_path);
    let mut file = File::create(path)?;
    let _ = file.write_all(&json.as_bytes());
    Ok(())
}

fn delete_keystore_file(wid: &str) -> Result<()> {
    let file_dir = WALLET_FILE_DIR.read().unwrap();
    let ks_path = format!("{}/{}.json", file_dir, wid);
    let path = Path::new(&ks_path);
    fs::remove_file(path)?;
    Ok(())
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

#[no_mangle]
pub extern "C" fn create_wallet(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = unsafe { landingpad(|| create_wallet_internal(&v)) };
    CString::new(json).expect("ret json").into_raw()
}

fn create_wallet_internal(v: &Value) -> Result<String> {
    let meta: Metadata = serde_json::from_value(v.clone())?;
    let password = v["password"].as_str().unwrap();
    let keystore = HdKeystore::new(password, meta);
    let _json = keystore.json();
    let _ = flush_keystore(&keystore);
    let ret = format!("{}", &keystore);
    cache_keystore(keystore);
    Ok(ret)
}

#[no_mangle]
pub unsafe extern "C" fn init_token_core_x(json_str: *const c_char) {
    let v = parse_arguments(json_str);
    // !!! warning !!! just set_panic_hook when debug
    // set_panic_hook();
    landingpad(|| init_token_core_x_internal(&v));
}

fn init_token_core_x_internal(v: &Value) -> Result<()> {
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
    let json = landingpad(|| find_wallet_by_mnemonic_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn find_wallet_by_mnemonic_internal(v: &Value) -> Result<String> {
    let mnemonic = v["mnemonic"].as_str().unwrap();
    let path = v["path"].as_str().unwrap();
    let symbol = coin_symbol_with_network(v);

    let mut coin_info = coin_info_from_symbol(&symbol)?;
    coin_info.derivation_path = path.to_string();
    let acc = match symbol.as_str() {
        "BITCOINCASH" | "BITCOINCASH-TESTNET" => {
            HdKeystore::mnemonic_to_account::<BchAddress, BchExtra>(&coin_info, mnemonic)
        }
        "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
            HdKeystore::mnemonic_to_account::<BtcForkAddress, BtcForkExtra>(&coin_info, mnemonic)
        }
        "TRON" => HdKeystore::mnemonic_to_account::<TrxAddress, EmptyExtra>(&coin_info, mnemonic),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;
    let address = acc.address;
    let kid = find_keystore_id_by_address(&address);
    if let Some(id) = kid {
        let map = KEYSTORE_MAP.read().unwrap();
        let ks: &HdKeystore = map.get(&id).unwrap();
        Ok(format!("{}", &ks))
    } else {
        Ok("{}".to_owned())
    }
}

#[no_mangle]
pub unsafe extern "C" fn find_wallet_by_private_key(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| find_wallet_by_private_key_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn find_wallet_by_private_key_internal(v: &Value) -> Result<String> {
    let priv_key = v["privateKey"].as_str().unwrap();
    let symbol = coin_symbol_with_network(v);

    verify_wif(priv_key, &symbol)?;

    let coin_info = coin_info_from_symbol(&symbol)?;
    let acc = match symbol.as_str() {
        "BITCOINCASH" | "BITCOINCASH-TESTNET" => {
            HdKeystore::private_key_to_account::<BchAddress, EmptyExtra>(&coin_info, priv_key)
        }
        "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
            HdKeystore::private_key_to_account::<BtcForkAddress, EmptyExtra>(&coin_info, priv_key)
        }
        "TRON" => {
            HdKeystore::private_key_to_account::<TrxAddress, EmptyExtra>(&coin_info, priv_key)
        }
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;
    let address = acc.address;
    let kid = find_keystore_id_by_address(&address);
    if let Some(id) = kid {
        let map = KEYSTORE_MAP.read().unwrap();
        let ks: &HdKeystore = map.get(&id).unwrap();
        Ok(format!("{}", &ks))
    } else {
        Ok("{}".to_owned())
    }
}

#[no_mangle]
pub unsafe extern "C" fn import_wallet_from_mnemonic(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| import_wallet_from_mnemonic_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn import_wallet_from_mnemonic_internal(v: &Value) -> Result<String> {
    let password = v["password"].as_str().unwrap();
    let mnemonic = v["mnemonic"].as_str().unwrap();
    let path = v["path"].as_str().unwrap();
    let overwrite = v["overwrite"].as_bool().unwrap();
    let symbol = coin_symbol_with_network(v);

    let meta: Metadata = serde_json::from_value(v.clone())?;
    // todo: mnemonic not valid
    let mut ks = HdKeystore::from_mnemonic(mnemonic, password, meta);

    let mut coin_info = coin_info_from_symbol(&symbol)?;
    coin_info.derivation_path = path.to_string();
    let account = match symbol.as_str() {
        "BITCOINCASH" | "BITCOINCASH-TESTNET" => {
            ks.derive_coin::<BchAddress, BchExtra>(&coin_info, password)
        }
        "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
            ks.derive_coin::<BtcForkAddress, BtcForkExtra>(&coin_info, password)
        }
        "TRON" => ks.derive_coin::<TrxAddress, EmptyExtra>(&coin_info, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;

    let exist_kid_opt = find_keystore_id_by_address(&account.address);
    if let Some(exist_kid) = exist_kid_opt {
        if !overwrite {
            return Err(format_err!("{}", "wallet_exists"));
        } else {
            ks.id = exist_kid;
        }
    }

    flush_keystore(&ks)?;
    let json = format!("{}", ks);
    cache_keystore(ks);

    Ok(json)
}

#[no_mangle]
pub unsafe extern "C" fn import_wallet_from_private_key(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| import_wallet_from_private_key_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn import_wallet_from_private_key_internal(v: &Value) -> Result<String> {
    let password = v["password"].as_str().unwrap();
    let priv_key = v["privateKey"].as_str().unwrap();

    let overwrite = v["overwrite"].as_bool().unwrap();
    let symbol = coin_symbol_with_network(v);

    verify_wif(priv_key, &symbol)?;
    let mut meta: Metadata = serde_json::from_value(v.clone())?;
    let chain_type = v["chainType"].as_str().unwrap();
    if chain_type.to_uppercase() == "ETHEREUM" {
        meta.source = Source::Private;
    } else {
        meta.source = Source::Wif;
    }
    let mut ks = HdKeystore::from_private_key(priv_key, password, meta.source);

    let coin_info = coin_info_from_symbol(&symbol)?;
    let account = match symbol.as_str() {
        "BITCOINCASH" | "BITCOINCASH-TESTNET" => {
            ks.derive_coin::<BchAddress, EmptyExtra>(&coin_info, password)
        }
        "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
            ks.derive_coin::<BtcForkAddress, EmptyExtra>(&coin_info, password)
        }
        "TRON" => ks.derive_coin::<TrxAddress, EmptyExtra>(&coin_info, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;

    let exist_kid_opt = find_keystore_id_by_address(&account.address);
    if let Some(exist_kid) = exist_kid_opt {
        if !overwrite {
            return Err(format_err!("{}", "wallet_exists"));
        } else {
            ks.id = exist_kid;
        }
    }

    flush_keystore(&ks)?;
    let json = format!("{}", ks);
    cache_keystore(ks);

    Ok(json)
}

#[no_mangle]
pub unsafe extern "C" fn export_mnemonic(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = landingpad(|| export_mnemonic_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn export_mnemonic_internal(v: &Value) -> Result<String> {
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
pub unsafe extern "C" fn export_private_key(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = landingpad(|| export_private_key_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn export_private_key_internal(v: &Value) -> Result<String> {
    let wid = v["id"].as_str().expect("id");
    let password = v["password"].as_str().expect("password");

    let map = KEYSTORE_MAP.read().unwrap();
    let keystore = match map.get(wid) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;
    let pk = keystore.private_key(password)?;
    Ok(serde_json::to_string(
        &json!({"ok": true, "privateKey": pk}),
    )?)
}

#[no_mangle]
pub unsafe extern "C" fn verify_password(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = landingpad(|| verify_password_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn verify_password_internal(v: &Value) -> Result<String> {
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

    let json = landingpad(|| sign_transaction_internal(json_str));
    CString::new(json).expect("ret json").into_raw()
}

fn sign_transaction_internal(json_str: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json_str).unwrap();
    let w_id = v["id"].as_str().expect("wid");
    let password = v["password"].as_str().expect("password");
    let symbol = coin_symbol_with_network(&v);

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    match symbol.as_str() {
        "BITCOINCASH"
        | "BITCOINCASH-TESTNET"
        | "LITECOIN"
        | "LITECOIN-P2WPKH"
        | "LITECOIN-TESTNET"
        | "LITECOIN-TESTNET-P2WPKH" => {
            sign_btc_fork_transaction(json_str, &symbol, keystore, password)
        }
        "TRON" => sign_trx_transaction(json_str, keystore, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }
}

fn sign_btc_fork_transaction(
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

fn sign_trx_transaction(json: &str, keystore: &HdKeystore, password: &str) -> Result<String> {
    let v = Value::from_str(json)?;
    let tx = TrxTransaction::try_from(v)?;
    let signed: TrxSignedTransaction = keystore.sign_transaction(&tx, Some(password))?;
    let signed_v: Value = signed.try_into()?;
    Ok(signed_v.to_string())
}

#[no_mangle]
pub unsafe extern "C" fn sign_message(json_str: *const c_char) -> *const c_char {
    let json_c_str = CStr::from_ptr(json_str);
    let json_str = json_c_str.to_str().unwrap();

    let json = landingpad(|| sign_message_internal(json_str));
    CString::new(json).expect("ret json").into_raw()
}

fn sign_message_internal(json_str: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json_str).unwrap();
    let w_id = v["id"].as_str().expect("wid");
    let password = v["password"].as_str().expect("password");
    let symbol = coin_symbol_with_network(&v);

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    match symbol.as_str() {
        "TRON" => sign_trx_message(json_str, keystore, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }
}

fn sign_trx_message(json: &str, keystore: &HdKeystore, password: &str) -> Result<String> {
    let message = serde_json::from_str::<TrxMessage>(json)?;
    let signed = keystore.sign_message(&message, Some(password))?;
    Ok(signed.signature)
}

#[no_mangle]
pub unsafe extern "C" fn calc_external_address(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| calc_external_address_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn calc_external_address_internal(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let external_id = v["externalIdx"].as_i64().expect("external_id");
    let symbol = coin_symbol_with_network(v);

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let account = keystore
        .account(&symbol)
        .ok_or_else(|| format_err!("account_not_found, chainType: {}", &symbol))?;
    let external_addr: ExternalAddress;
    if symbol.starts_with("BITCOINCASH") {
        let extra = BchExtra::from(account.extra.clone());
        external_addr = extra.calc_external_address(external_id, &symbol)?;
    } else {
        let extra = BtcForkExtra::from(account.extra.clone());
        external_addr = extra.calc_external_address(external_id, &symbol)?;
    }

    Ok(serde_json::to_string(&external_addr)?)
}

#[no_mangle]
pub unsafe extern "C" fn remove_wallet(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = landingpad(|| remove_wallet_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn remove_wallet_internal(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().expect("wallet_id");
    let password = v["password"].as_str().expect("password");

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore: &HdKeystore = match map.get(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    if keystore.verify_password(password) {
        delete_keystore_file(w_id)?;
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
    let json = landingpad(|| get_derived_key_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn get_derived_key_internal(v: &Value) -> Result<String> {
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
    let json = landingpad(|| verify_derived_key_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn verify_derived_key_internal(v: &Value) -> Result<String> {
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
    let json = landingpad(|| cache_derived_key_internal(&v));
    CString::new(json).expect("ret json").into_raw()
}

fn cache_derived_key_internal(v: &Value) -> Result<String> {
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
    let json = landingpad(clear_derived_key_internal);
    CString::new(json).expect("ret json").into_raw()
}

fn clear_derived_key_internal() -> Result<String> {
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
        cache_derived_key, calc_external_address, clear_derived_key, clear_err, export_mnemonic,
        export_private_key, find_wallet_by_private_key, get_derived_key, get_last_err_message,
        import_wallet_from_private_key, remove_wallet, sign_message, sign_transaction,
        verify_derived_key, verify_password,
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
            let param = r#"{
            "chainType":"BITCOINCASH",
            "mnemonic":"blind gravity card grunt basket expect garment tilt organ concert great critic",
            "network":"MAINNET",
            "path":"m/44'/145'/0'/0/0",
            "segWit":"NONE"
            }"#;
            let ret = unsafe { _to_str(find_wallet_by_mnemonic(_to_c_char(param))) };
            assert_eq!("{}", ret);

            let param = r#"{
            "chainType":"BITCOINCASH",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "network":"MAINNET",
            "path":"m/44'/145'/0'/0/0",
            "segWit":"NONE"
            }"#;
            let ret = unsafe { _to_str(find_wallet_by_mnemonic(_to_c_char(param))) };
            let v = Value::from_str(ret).expect("find wallet");
            assert_eq!(v["address"], "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r");
        })
    }

    #[test]
    fn import_wallet_from_mnemonic_test() {
        run_test(|| {
            let param = r#"{
            "chainType":"BITCOINCASH",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"BCH-Wallet-1",
            "network":"MAINNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/145'/0'/0/0",
            "segWit":"NONE",
            "source":"MNEMONIC"
            }"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let expected = r#"
            {
                "address": "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
                "chainType": "BITCOINCASH",
                "createdAt": 1566455834,
                "encXPub": "wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==",
                "externalAddress": {
                    "address": "qzyrtfn4a7cdkn7sp60tw7hl8zndt0tk0sst3p6qr5",
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

            let imported_id = ret_v["id"].as_str().unwrap();
            let param = json!({
                "id": imported_id,
                "chainType": "BITCOINCASH",
                "externalIdx": 2
            });

            let ret = unsafe {
                _to_str(calc_external_address(_to_c_char(
                    param.to_string().as_str(),
                )))
            };
            let ret_v: Value = Value::from_str(ret).unwrap();
            let expected = r#"
            {
                "address": "qzhsz3s4hr0f3x0v00zdn6w50tdpa9zgryp4kxgx49",
                "derivedPath": "0/2",
                "type": "EXTERNAL"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            assert_eq!(expected_v["derivedPath"], ret_v["derivedPath"]);
            assert_eq!(expected_v["address"], ret_v["address"]);
            remove_created_wallet(imported_id);
        });
    }

    #[test]
    fn import_wallet_from_mnemonic_testnet() {
        run_test(|| {
            let param = r#"{
            "chainType":"BITCOINCASH",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"BCH-Wallet-1",
            "network":"TESTNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/1'/0'/0/0",
            "segWit":"NONE",
            "source":"MNEMONIC"
            }"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
            let expected = r#"
            {
                "address": "qqurlwqukz3lcujttcyvlzaagppnd4c37chrtrylmc",
                "chainType": "BITCOINCASH",
                "createdAt": 1566455834,
                "encXPub": "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==",
                "externalAddress": {
                    "address": "qqn4as4zx0jmy02rlgv700umavxt8xtpzus6u7flzk",
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

            let imported_id = ret_v["id"].as_str().unwrap();
            let param = json!({
                "id": imported_id,
                "chainType": "BITCOINCASH",
                "network": "TESTNET",
                "externalIdx": 2
            });

            let ret = unsafe {
                _to_str(calc_external_address(_to_c_char(
                    param.to_string().as_str(),
                )))
            };
            let ret_v: Value = Value::from_str(ret).unwrap();
            let expected = r#"
            {
                "address": "qqrhpq50f5n5sdgj0ehwz8qtrc3m6dnazghh3aj0ag",
                "derivedPath": "0/2",
                "type": "EXTERNAL"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");
            assert_eq!(expected_v["derivedPath"], ret_v["derivedPath"]);
            assert_eq!(expected_v["address"], ret_v["address"]);
            remove_created_wallet(imported_id);
        });
    }

    #[test]
    fn import_bch_wallet_from_private_key_test() {
        run_test(|| {
            let param = r#"{
            "chainType":"BITCOINCASH",
            "privateKey":"L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
            "name":"BCH-Wallet-1",
            "network":"MAINNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "segWit":"NONE",
            "source":"WIF"
            }"#;
            let imported_ret =
                unsafe { _to_str(import_wallet_from_private_key(_to_c_char(param))) };
            let param = r#"{
            "chainType":"BITCOINCASH",
            "privateKey":"L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
            "network":"MAINNET",
            "segWit":"NONE"
            }"#;
            let founded_ret = unsafe { _to_str(find_wallet_by_private_key(_to_c_char(param))) };
            let expected = r#"
            {
                "address": "qrnvl24e5kd6rpls53wmpvtfcgdmfrcfkv8fhnq9kr",
                "chainType": "BITCOINCASH",
                "createdAt": 1566455834,
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "BCH-Wallet-1",
                "passwordHint": "",
                "source": "WIF"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");

            [imported_ret, founded_ret].iter().for_each(|ret| {
                let ret_v = Value::from_str(ret).unwrap();
                assert_eq!(expected_v["address"], ret_v["address"]);
                assert_eq!(expected_v["chainType"], ret_v["chainType"]);
                assert_eq!(expected_v["source"], ret_v["source"]);
            });
            let imported_v = Value::from_str(imported_ret).unwrap();

            let imported_id = imported_v["id"].as_str().unwrap();
            let param = json!({
            "id": imported_id,
            "password": "Insecure Password"
            });
            let ret = unsafe { _to_str(export_private_key(_to_c_char(&param.to_string()))) };
            let ret_v = Value::from_str(ret).expect("from expected");
            let export_expected = r#"
            {
                "privateKey": "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
                "ok": true
            }
            "#;
            let export_expected_v = Value::from_str(export_expected).expect("from expected");
            assert_eq!(export_expected_v["privateKey"], ret_v["privateKey"]);

            remove_created_wallet(imported_id);
        });
    }

    #[test]
    fn import_bch_wallet_from_private_key_testnet() {
        run_test(|| {
            let param = r#"{
            "chainType":"BITCOINCASH",
            "privateKey":"cT4fTJyLd5RmSZFHnkGmVCzXDKuJLbyTt7cy77ghTTCagzNdPH1j",
            "name":"BCH-Wallet-1",
            "network":"TESTNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "segWit":"NONE",
            "source":"WIF"
            }"#;
            let imported_ret =
                unsafe { _to_str(import_wallet_from_private_key(_to_c_char(param))) };
            let founded_ret = unsafe { _to_str(find_wallet_by_private_key(_to_c_char(param))) };
            let expected = r#"
            {
                "address": "qrnvl24e5kd6rpls53wmpvtfcgdmfrcfkvrmn5zj3l",
                "chainType": "BITCOINCASH",
                "createdAt": 1566455834,
                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
                "name": "BCH-Wallet-1",
                "passwordHint": "",
                "source": "WIF"
            }
            "#;
            let expected_v = Value::from_str(expected).expect("from expected");

            [imported_ret, founded_ret].iter().for_each(|ret| {
                let ret_v = Value::from_str(ret).unwrap();
                assert_eq!(expected_v["address"], ret_v["address"]);
                assert_eq!(expected_v["chainType"], ret_v["chainType"]);
                assert_eq!(expected_v["source"], ret_v["source"]);
            });
            let imported_v = Value::from_str(imported_ret).unwrap();
            let imported_id = imported_v["id"].as_str().unwrap();
            remove_created_wallet(imported_id);
        });
    }

    #[test]
    fn import_ltc_wallet_from_mnemonic_test() {
        run_test(|| {
            let param = r#"{
            "chainType":"LITECOIN",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"LTC-Wallet-1",
            "network":"TESTNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/2'/0'/0/0",
            "segWit":"P2WPKH",
            "source":"MNEMONIC"
            }"#;
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
            let param = r#"{
            "chainType":"LITECOIN",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"LTC-Wallet-1",
            "network":"MAINNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/2'/0'/0/0",
            "segWit":"NONE",
            "source":"MNEMONIC"
            }"#;
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
            let param = r#"{
            "chainType":"LITECOIN",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"LTC-Wallet-1",
            "network":"TESTNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/1'/0'/0/0",
            "segWit":"NONE",
            "source":"MNEMONIC"
            }"#;
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
            let param = r#"{
            "chainType":"LITECOIN",
            "mnemonic":"calm release clay imitate top extend close draw quiz refuse shuffle injury",
            "name":"LTC-Wallet-1",
            "network":"MAINNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/1'/0'/0/0",
            "segWit":"NONE",
            "source":"MNEMONIC"
            }"#;
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
            let param = r#"{
            "chainType":"TRON",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"TRX-Wallet-1",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/195'/0'/0/0",
            "source":"MNEMONIC"
            }"#;
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
    fn sign_trx_message_test() {
        run_test(|| {
            let param = r#"{
            "chainType":"TRON",
            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "name":"TRX-Wallet-1",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/195'/0'/0/0",
            "source":"MNEMONIC"
            }"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let ret_v = Value::from_str(ret).unwrap();

            let param = json!({
                "id": ret_v["id"].as_str().expect("wallet_id"),
                "chainType": "TRON",
                "password": "Insecure Password",
                "value": "0xaaaaaaaa",
                "isHex": true,
                "isTronHeader": true
            });
            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };

            assert_eq!("47fb89c1a3726de25f64b0d98dd8ca3c12079c12cec31a35ac71d7ce337cc4df02fec800ee1c149b9cb9f79e9f60f665a4a1bf00be20b7fbca7007f9a0076d731c", signed);

            let param = json!({
                "id": ret_v["id"].as_str().expect("wallet_id"),
                "chainType": "TRON",
                "password": "Insecure Password",
                "value": "aaaaaaaa",
                "isHex": true,
                "isTronHeader": true
            });
            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };

            assert_eq!("47fb89c1a3726de25f64b0d98dd8ca3c12079c12cec31a35ac71d7ce337cc4df02fec800ee1c149b9cb9f79e9f60f665a4a1bf00be20b7fbca7007f9a0076d731c", signed);

            let param = json!({
                "id": ret_v["id"].as_str().expect("wallet_id"),
                "chainType": "TRON",
                "password": "Insecure Password",
                "value": "abc",
                "isHex": false,
                "isTronHeader": true
            });
            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };

            assert_eq!("f61b5966ca46dd838586f96dddf3fe594980f04c783492c240edcb3a5dd6c49b5f9ca8172e222943a61e177debad0dc374f80d4fe90a0a52b8607a1447225fd21b", signed);

            let param = json!({
                "id": ret_v["id"].as_str().expect("wallet_id"),
                "chainType": "TRON",
                "password": "Insecure Password",
                "value": "abc",
                "isHex": false,
                "isTronHeader": false
            });
            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };

            assert_eq!("b256bb5fa285d981fb424f997c34ff9575eca7c0ec26f47141dfae058ecc7ada40f2ee3916c183fc8b3e0c810051756a9f1307d9f4e9b883a98a8b4ebce74ce51b", signed);

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
            assert_eq!(err, "password_incorrect");
        })
    }

    #[test]
    fn sign_transaction_test() {
        run_test(|| {
            let param = r#"
            {
                "id":"9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
                "password": "Insecure Password",
                "to": "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s",
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
                        "address": "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
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
            let param = r#"{
            "chainType":"LITECOIN",
            "mnemonic":"salute slush now script nest law admit achieve voice soda fruit field",
            "name":"LTC-Wallet-1",
            "network":"MAINNET",
            "overwrite":true,
            "password":"Insecure Password",
            "passwordHint":"",
            "path":"m/44'/1'/0'/0/0",
            "segWit":"NONE",
            "source":"MNEMONIC"
            }"#;
            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };

            let ret_v = Value::from_str(ret).unwrap();
            let imported_id = ret_v["id"].as_str().expect("wallet_id");
            let param = json!({
                "id": imported_id,
                "password": "Insecure Password"
            });

            let derived_key =
                unsafe { _to_str(get_derived_key(_to_c_char(param.to_string().as_str()))) };

            let param = json!({
                "id": imported_id,
                "derivedKey": derived_key
            });
            let ret =
                unsafe { _to_str(verify_derived_key(_to_c_char(param.to_string().as_str()))) };
            let ret_v: Value = serde_json::from_str(ret).unwrap();
            assert_eq!(derived_key, ret_v["derivedKey"].as_str().unwrap());

            let param = json!({
                "id": imported_id,
                "derivedKey": "1111111111111111111111111111111111111111111111111111111111111111"
            });
            let _ret =
                unsafe { _to_str(verify_derived_key(_to_c_char(param.to_string().as_str()))) };
            let err = unsafe { _to_str(get_last_err_message()) };
            assert_eq!("invalid_cached_derived_key", err);

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
            assert_eq!("password_incorrect", err);

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
    fn verify_password_test() {
        run_test(|| {
            let param = r#"
        {
            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
            "password": "Wrong Password"
        }
        "#;
            let _ = unsafe { _to_str(verify_password(_to_c_char(param))) };
            let err = unsafe { _to_str(get_last_err_message()) };
            assert_eq!(err, "password_incorrect");

            let param = r#"
        {
            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
            "password": "Insecure Password"
        }
        "#;
            let ret = unsafe { _to_str(verify_password(_to_c_char(param))) };
            let v: Value = serde_json::from_str(ret).unwrap();
            assert!(v["ok"].as_bool().unwrap())
        })
    }
}
