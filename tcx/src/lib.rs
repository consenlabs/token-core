use std::ffi::{CStr, CString};

use std::os::raw::c_char;

use std::fs::File;
use std::io::{Read, Write};
use utils::Result;
use utils::LAST_BACKTRACE;
use utils::LAST_ERROR;

use crate::utils::landingpad;

use crate::utils::set_panic_hook;
use core::borrow::Borrow;
use serde_json::json;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;
use tcx_bch::{BchAddress, BitcoinCashTransaction, ExtendedPubKeyExtra, Utxo};
use tcx_chain::{Account, CoinInfo, CurveType, HdKeystore, Metadata, Source};
// #[link(name = "TrezorCrypto")]
// extern {
//     fn mnemonic_generate(strength: c_int, mnemonic: *mut c_char) -> c_int;
// }

#[macro_use]
extern crate failure;

#[macro_use]
pub mod utils;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref KEYSTORE_MAP: RwLock<HashMap<String, HdKeystore>> = RwLock::new(HashMap::new());
    static ref WALLET_FILE_DIR: RwLock<String> = RwLock::new("../test-data".to_string());
    static ref XPubCommonKey128: RwLock<String> = RwLock::new(String::new());
    static ref XPubCommonIv: RwLock<String> = RwLock::new(String::new());
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
        if iter.find(|a| a.address == address).is_some() {
            k_id = Some(id.to_string());
            break;
        }
    }
    k_id
}

fn _coin_info_from_symbol(symbol: &str) -> Result<CoinInfo> {
    match symbol {
        "BCH" => Ok(CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: "m/44'/145'/0'".to_string(),
            curve: CurveType::SECP256k1,
        }),
        _ => Err(format_err!("unsupptored_chain")),
    }
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn free_const_string(s: *const c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CStr::from_ptr(s)
    };
}

fn parse_arguments(json_str: *const c_char) -> Value {
    let json_c_str = unsafe { CStr::from_ptr(json_str) };
    let json_str = json_c_str.to_str().unwrap();
    serde_json::from_str(json_str).unwrap()
}

pub unsafe extern "C" fn create_wallet(json_str: *const c_char) -> *const c_char {
    let json_c_str = unsafe { CStr::from_ptr(json_str) };
    let json_str = json_c_str.to_str().unwrap();
    let v: Value = serde_json::from_str(json_str).unwrap();
    let json = landingpad(|| _create_wallet(&v));
    CString::new(json).unwrap().into_raw()
}

fn _create_wallet(v: &Value) -> Result<String> {
    let meta: Metadata = serde_json::from_value(v.clone())?;
    let password = v["password"].as_str().unwrap();
    let keystore = HdKeystore::new(password, meta);
    let json = keystore.json();
    _flush_keystore(&keystore);
    cache_keystore(keystore);
    Ok(json)
}

#[no_mangle]
pub unsafe extern "C" fn init_token_core_x(json_str: *const c_char) {
    let v = parse_arguments(json_str);
    // !!! warning !!! just set_panic_hook when debug
    // set_panic_hook();
    crate::utils::landingpad(|| _init_token_core_x(&v));
    ()
}

fn _init_token_core_x(v: &Value) -> Result<()> {
    let file_dir = v["fileDir"].as_str().unwrap();
    let xpub_common_key = v["xpubCommonKey128"].as_str().expect("XPubCommonKey128");
    let xpub_common_iv = v["xpubCommonIv"].as_str().expect("xpubCommonIv");
    *WALLET_FILE_DIR.write().unwrap() = file_dir.to_string();
    *XPubCommonKey128.write().unwrap() = xpub_common_key.to_string();
    *XPubCommonIv.write().unwrap() = xpub_common_iv.to_string();

    let p = Path::new(file_dir);
    let walk_dir = std::fs::read_dir(p).unwrap();
    for entry in walk_dir {
        let entry = entry.unwrap();
        let fp = entry.path();
        let mut f = File::open(fp).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents);
        let v: Value = serde_json::from_str(&contents).unwrap();

        let version = v["version"].as_i64().unwrap();
        if version != HdKeystore::VERSION as i64 {
            continue;
        }
        let keystore: HdKeystore = serde_json::from_str(&contents)?;
        cache_keystore(keystore);
    }
    Ok(())
}

//
#[no_mangle]
pub unsafe extern "C" fn find_wallet_by_mnemonic(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = crate::utils::landingpad(|| _find_wallet_by_mnemonic(&v));
    CString::new(json).unwrap().into_raw()
}

fn _find_wallet_by_mnemonic(v: &Value) -> Result<String> {
    let mnemonic = v["mnemonic"].as_str().unwrap();
    let path = v["path"].as_str().unwrap();
    let chain_type = v["chainType"].as_str().unwrap();

    let (acc, _) = match chain_type {
        "BCH" => {
            let mut coin_info = _coin_info_from_symbol("BCH")?;
            coin_info.derivation_path = path.to_string();
            HdKeystore::mnemonic_to_account::<BchAddress, ExtendedPubKeyExtra>(&coin_info, mnemonic)
        }
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;
    let address = acc.address;
    let kid = find_keystore_id_by_address(&address);
    if let Some(id) = kid {
        let map = KEYSTORE_MAP.read().unwrap();
        let ks = map.get(&id).unwrap();
        Ok(ks.json())
    } else {
        Ok("{}".to_owned())
    }
}

#[no_mangle]
pub unsafe extern "C" fn import_wallet_from_mnemonic(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = crate::utils::landingpad(|| _import_wallet_from_mnemonic(&v));
    CString::new(json).unwrap().into_raw()
}

fn _import_wallet_from_mnemonic(v: &Value) -> Result<String> {
    let password = v["password"].as_str().unwrap();
    let mnemonic = v["mnemonic"].as_str().unwrap();
    let _path = v["path"].as_str().unwrap();
    let overwrite = v["overwrite"].as_bool().unwrap();

    let chain_type = v["chainType"].as_str().unwrap();
    let meta: Metadata = serde_json::from_value(v.clone())?;
    let mut ks = HdKeystore::from_mnemonic(mnemonic, password, meta);
    let mut pw = Map::new();

    let (account, extra) = match chain_type {
        "BCH" => {
            let coin_info = _coin_info_from_symbol("BCH")?;
            ks.derive_coin::<BchAddress, ExtendedPubKeyExtra>(&coin_info, password)
        }
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }?;

    let exist_kid_opt = find_keystore_id_by_address(&account.address);
    if exist_kid_opt.is_some() {
        if !overwrite {
            return Err(format_err!("{}", "wallet_exists"));
        } else {
            ks.id = exist_kid_opt.unwrap();
        }
    }

    _flush_keystore(&ks)?;
    let account = &ks.account(chain_type).expect("account");
    let mut pw = _presented_wallet(&ks, &account)?;

    match chain_type {
        "BCH" => {
            let key = XPubCommonKey128.read().unwrap();
            let iv = XPubCommonIv.read().unwrap();
            let enc_xpub = extra.enc_xpub(&*key, &*iv)?;
            pw.insert("encXPub".to_string(), json!(enc_xpub));

            let external_address = extra.calc_external_address::<BchAddress>(0)?;
            pw.insert("externalAddress".to_string(), json!(external_address));
        }
        _ => {}
    }

    let json = serde_json::to_string(&pw)?;
    cache_keystore(ks);

    Ok(json)
}

struct PresentedWallet {
    id: String,
    address: String,
    created_at: i64,
    source: Source,
    chain_type: String,
}

fn _presented_wallet(keystore: &HdKeystore, acc: &Account) -> Result<Map<String, Value>> {
    let mut pw = Map::new();
    pw.insert("id".to_string(), json!(keystore.id.to_string()));
    pw.insert("address".to_string(), json!(acc.address.to_string()));
    pw.insert("createdAt".to_string(), json!(keystore.meta.timestamp));
    pw.insert("source".to_string(), json!(keystore.meta.source));
    pw.insert("chainType".to_string(), json!(acc.coin.to_string()));

    Ok(pw)
}

fn _flush_keystore(ks: &HdKeystore) -> Result<()> {
    let json = ks.json();

    let file_dir = WALLET_FILE_DIR.read().unwrap();
    let ks_path = format!("{}/{}.json", file_dir, ks.id);
    let path = Path::new(&ks_path);
    let mut file = File::create(path)?;
    file.write_all(&json.as_bytes());
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn export_mnemonic(json_str: *const c_char) -> *const c_char {
    let v: Value = parse_arguments(json_str);
    let json = crate::utils::landingpad(|| _export_mnemonic(&v));
    CString::new(json).unwrap().into_raw()
}

fn _export_mnemonic(v: &Value) -> Result<String> {
    let wid = v["id"].as_str().unwrap();
    let password = v["password"].as_str().unwrap();

    let map = KEYSTORE_MAP.read().unwrap();
    let keystore = match map.get(wid) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;
    keystore.mnemonic(password)
}

//
#[no_mangle]
pub unsafe extern "C" fn sign_transaction(json_str: *const c_char) -> *const c_char {
    let json_c_str = unsafe { CStr::from_ptr(json_str) };
    let json_str = json_c_str.to_str().unwrap();

    let json = crate::utils::landingpad(|| _sign_transaction(json_str));
    CString::new(json).unwrap().into_raw()
}

fn _sign_transaction(json_str: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json_str).unwrap();
    let w_id = v["id"].as_str().expect("wid");
    let chain_type = v["chainType"].as_str().expect("chainType");
    let password = v["password"].as_str().expect("password");

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    match chain_type {
        "BCH" => _sign_bch_transaction(json_str, keystore, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }
}

fn _sign_bch_transaction(json: &str, keystore: &HdKeystore, password: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json).expect("sign_transaction_json");
    let unspents: Vec<Utxo> = serde_json::from_value(v["outputs"].clone()).expect("outputs");
    let internal_used = v["internalUsed"].as_i64().expect("internalUsed");
    let change_idx = internal_used + 1;
    let to = v["to"].as_str().expect("to");
    let amount = v["amount"]
        .as_str()
        .expect("amount")
        .parse::<i64>()
        .unwrap();
    let fee = v["fee"].as_str().expect("fee").parse::<i64>().unwrap();
    let account = keystore
        .account(&"BCH")
        .ok_or(format_err!("account_not_found"))?;
    let path = &account.derivation_path;
    let extra = ExtendedPubKeyExtra::from(account.extra.clone());
    let bch_tran = BitcoinCashTransaction {
        to: to.to_owned(),
        amount,
        unspents,
        memo: "".to_string(),
        fee,
        change_idx: change_idx as u32,
    };
    let paths = bch_tran.collect_prv_keys_paths(path)?;
    let priv_keys = &keystore.key_at_paths("BCH", &paths, password)?;

    let ret = bch_tran.sign_transaction(&priv_keys, &extra.xpub)?;
    Ok(serde_json::to_string(&ret)?)
}

#[no_mangle]
pub unsafe extern "C" fn calc_external_address(json_str: *const c_char) -> *const c_char {
    let v = parse_arguments(json_str);
    let json = crate::utils::landingpad(|| _calc_external_address(&v));
    CString::new(json).unwrap().into_raw()
}

fn _calc_external_address(v: &Value) -> Result<String> {
    let w_id = v["id"].as_str().unwrap();
    let external_id = v["externalIdx"].as_i64().expect("external_id");
    let network = v["network"].as_str().unwrap();
    let chain_type = v["chainType"].as_str().unwrap();

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(w_id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let account = keystore
        .account(&chain_type)
        .ok_or(format_err!("account_not_found, chainType: {}", &chain_type))?;

    let extra = ExtendedPubKeyExtra::from(account.extra.clone());
    let external_addr = extra.calc_external_address::<BchAddress>(external_id)?;
    Ok(serde_json::to_string(&external_addr)?)
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
    use crate::utils::LAST_ERROR;
    use crate::XPubCommonIv;
    use crate::XPubCommonKey128;
    use crate::WALLET_FILE_DIR;
    use crate::{_find_wallet_by_mnemonic, _import_wallet_from_mnemonic};
    use serde_json::Value;
    use std::str::FromStr;
    use std::sync::RwLock;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn path() {
        let file_dir = "/Users/xyz/Library/Developer/CoreSimulator/Devices/1C6326AE-C550-43D5-A1A7-CF791B4A04CA/data/Containers/Data/Application/BC076852-DF07-42EA-82B1-2FA8C5CEE9EE/Documents/wallets/";
        let id = "ec9298f7-7f2b-4483-90af-cc440a411d82";

        let ks_path = format!("{}{}.json", file_dir, id);
        assert_eq!("/Users/xyz/Library/Developer/CoreSimulator/Devices/1C6326AE-C550-43D5-A1A7-CF791B4A04CA/data/Containers/Data/Application/BC076852-DF07-42EA-82B1-2FA8C5CEE9EE/Documents/wallets/ec9298f7-7f2b-4483-90af-cc440a411d82.json", ks_path);
    }

    #[test]
    fn find_wallet_by_mnemonic() {
        let param = r#"{"chainType":"BCH","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","network":"MAINNET","path":"m/44'/145'/0'/0/0","segWit":"P2WPKH"}"#;
        let v = Value::from_str(param).expect("param");
        let wallet = _find_wallet_by_mnemonic(&v);
        assert!(wallet.is_ok());
        assert_eq!("{}", wallet.unwrap_or_default());
    }

    #[test]
    fn import_wallet_from_mnemonic() {
        *XPubCommonKey128.write().unwrap() = "B888D25EC8C12BD5043777B1AC49F872".to_string();
        *XPubCommonIv.write().unwrap() = "9C0C30889CBCC5E01AB5B2BB88715799".to_string();
        let param = r#"{"chainType":"BCH","mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","name":"BCH-Wallet-1","network":"MAINNET","overwrite":true,"password":"imtoken1","passwordHint":"","path":"m/44'/145'/0'/0/0","segWit":"P2WPKH","source":"MNEMONIC"}"#;
        let v = Value::from_str(param).expect("param");
        let wallet = _import_wallet_from_mnemonic(&v);
        let expected = r#"{"address":"bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885","chainType":"BCH","createdAt":1565689742,"encXPub":"wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==","externalAddress":{"type":"EXTERNAL","address":"bitcoincash:qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r","derivedPath":"0/0"},"id":"0e85b1ea-91f5-4389-aec4-f6706493cc80","source":"MNEMONIC"}"#;
        let expected_v = Value::from_str(expected).expect("from expected");

        assert!(wallet.is_ok());
        let ret_v =
            Value::from_str(&wallet.unwrap_or_default()).expect("from import_wallet_by_mnemonic");
        assert_eq!(expected_v["address"], ret_v["address"]);
        assert_eq!(expected_v["chainType"], ret_v["chainType"]);
        assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
        assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);
    }

}
