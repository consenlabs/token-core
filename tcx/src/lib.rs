use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::Read;
use std::os::raw::c_char;
use std::path::Path;

use prost::Message;
use serde_json::Value;

use tcx_bch::BchAddress;
use tcx_btc_fork::{address::BtcForkAddress, ExternalAddress};
use tcx_chain::TransactionSigner;
use tcx_chain::{HdKeystore, MessageSigner, Metadata, Source};
use tcx_chain::{Keystore, KeystoreGuard};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_primitive::verify_private_key;
use tcx_tron::TrxAddress;

mod api;
use crate::api::{Response, TcxAction};
pub mod error_handling;
pub mod handler;
use crate::error_handling::{landingpad, Result, LAST_BACKTRACE, LAST_ERROR};
use crate::handler::{
    encode_message, hd_store_create, hd_store_derive, hd_store_export, hd_store_import,
    keystore_common_delete, keystore_common_exists, keystore_common_verify,
    private_key_store_export, private_key_store_import, sign_tx, tron_sign_message, Buffer,
};
mod filemanager;
use crate::filemanager::{
    cache_keystore, delete_keystore_file, find_keystore_id_by_address, flush_keystore,
    KEYSTORE_MAP, WALLET_FILE_DIR,
};

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

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

#[no_mangle]
pub unsafe extern "C" fn free_buf(buf: Buffer) {
    let s = std::slice::from_raw_parts_mut(buf.data, buf.len);
    let s = s.as_mut_ptr();
    Box::from_raw(s);
}

fn parse_arguments(json_str: *const c_char) -> Value {
    let json_c_str = unsafe { CStr::from_ptr(json_str) };
    let json_str = json_c_str.to_str().expect("parse_arguments to_str");
    serde_json::from_str(json_str).expect("parse_arguments serde_json")
}

/// dispatch protobuf rpc call
#[no_mangle]
pub unsafe extern "C" fn call_tcx_api(buf: Buffer) -> Buffer {
    let data = std::slice::from_raw_parts_mut(buf.data, buf.len);
    let action: TcxAction = TcxAction::decode(data).expect("decode tcx api");
    let mut reply: Vec<u8> = match action.method.to_lowercase().as_str() {
        "init_token_core_x" => landingpad(|| {
            handler::init_token_core_x(&action.param.unwrap().value);
            Ok(vec![])
        }),
        "hd_store_create" => landingpad(|| hd_store_create(&action.param.unwrap().value)),
        "hd_store_import" => landingpad(|| hd_store_import(&action.param.unwrap().value)),
        "hd_store_export" => landingpad(|| hd_store_export(&action.param.unwrap().value)),
        "hd_store_derive" => landingpad(|| hd_store_derive(&action.param.unwrap().value)),

        "private_key_store_import" => {
            landingpad(|| private_key_store_import(&action.param.unwrap().value))
        }
        "private_key_store_export" => {
            landingpad(|| private_key_store_export(&action.param.unwrap().value))
        }
        "keystore_common_verify" => {
            landingpad(|| keystore_common_verify(&action.param.unwrap().value))
        }
        "keystore_common_delete" => {
            landingpad(|| keystore_common_delete(&action.param.unwrap().value))
        }
        "keystore_common_exists" => {
            landingpad(|| keystore_common_exists(&action.param.unwrap().value))
        }
        "keystore_common_accounts" => {
            landingpad(|| keystore_common_exists(&action.param.unwrap().value))
        }

        "sign_tx" => landingpad(|| sign_tx(&action.param.unwrap().value)),

        "tron_sign_msg" => landingpad(|| tron_sign_message(&action.param.unwrap().value)),
        _ => landingpad(|| {
            encode_message(Response {
                is_success: false,
                error: "unsupported_method".to_string(),
            })
        }),
    };

    wrap_buffer(reply)
}

pub fn wrap_buffer(to_wrap: Vec<u8>) -> Buffer {
    let mut to_wrap = to_wrap;
    let data = to_wrap.as_mut_ptr();
    let len = to_wrap.len();
    std::mem::forget(to_wrap);
    Buffer { data, len }
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

        let mut f = fs::File::open(fp).expect("open file");
        let mut contents = String::new();

        let _ = f.read_to_string(&mut contents);
        let v: Value = serde_json::from_str(&contents).expect("read json from content");

        let version = v["version"].as_i64().expect("version");
        if version != i64::from(HdKeystore::VERSION) {
            continue;
        }
        //        let keystore: HdKeystore = serde_json::from_str(&contents)?;
        let keystore = Keystore::from_json(&contents)?;
        cache_keystore(keystore);
    }
    Ok(())
}

// get_derived_key and cache_derived_key functions are one way to speed decrypt data,
// you should cache the derived_key in some secure place like keystore in iOS, and protect it by biometric.
//#[no_mangle]
//pub unsafe extern "C" fn get_derived_key(json_str: *const c_char) -> *const c_char {
//    let v = parse_arguments(json_str);
//    let json = landingpad(|| get_derived_key_internal(&v));
//    CString::new(json).expect("ret json").into_raw()
//}
//
//fn get_derived_key_internal(v: &Value) -> Result<String> {
//    let w_id = v["id"].as_str().expect("wallet_id");
//    let password = v["password"].as_str().expect("password");
//
//    let map = KEYSTORE_MAP.read().unwrap();
//    let keystore: &HdKeystore = match map.get(w_id) {
//        Some(keystore) => Ok(keystore),
//        _ => Err(format_err!("{}", "wallet_not_found")),
//    }?;
//
//    let derived_key = keystore.crypto.generate_derived_key(password)?;
//
//    Ok(hex::encode(derived_key))
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn verify_derived_key(json_str: *const c_char) -> *const c_char {
//    let v = parse_arguments(json_str);
//    let json = landingpad(|| verify_derived_key_internal(&v));
//    CString::new(json).expect("ret json").into_raw()
//}
//
//fn verify_derived_key_internal(v: &Value) -> Result<String> {
//    let w_id = v["id"].as_str().expect("wallet_id");
//    let derived_key = v["derivedKey"].as_str().expect("derivedKey");
//
//    let map = KEYSTORE_MAP.read().unwrap();
//    let keystore: &HdKeystore = match map.get(w_id) {
//        Some(keystore) => Ok(keystore),
//        _ => Err(format_err!("{}", "wallet_not_found")),
//    }?;
//    let derived_key_bytes: Vec<u8> = hex::decode(derived_key)?;
//    if !keystore.crypto.verify_derived_key(&derived_key_bytes) {
//        Err(format_err!("{}", "invalid_cached_derived_key"))
//    } else {
//        Ok(serde_json::to_string(
//            &json!({ "id": w_id, "derivedKey": derived_key }),
//        )?)
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn cache_derived_key(json_str: *const c_char) -> *const c_char {
//    let v = parse_arguments(json_str);
//    let json = landingpad(|| cache_derived_key_internal(&v));
//    CString::new(json).expect("ret json").into_raw()
//}
//
//fn cache_derived_key_internal(v: &Value) -> Result<String> {
//    let w_id = v["id"].as_str().expect("wallet_id");
//    let derived_key = v["derivedKey"].as_str().expect("derivedKey");
//    let tmp_password = v["tempPassword"].as_str().expect("tempPassword");
//
//    let mut map = KEYSTORE_MAP.write().unwrap();
//    let keystore: &mut HdKeystore = match map.get_mut(w_id) {
//        Some(keystore) => Ok(keystore),
//        _ => Err(format_err!("{}", "wallet_not_found")),
//    }?;
//    let derived_key_bytes: Vec<u8> = hex::decode(derived_key)?;
//    if !keystore.crypto.verify_derived_key(&derived_key_bytes) {
//        Err(format_err!("{}", "invalid_cached_derived_key"))
//    } else {
//        keystore
//            .crypto
//            .cache_derived_key(tmp_password, &derived_key_bytes);
//        Ok(serde_json::to_string(
//            &json!({ "id": w_id, "derivedKey": derived_key }),
//        )?)
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn clear_derived_key() -> *const c_char {
//    //    let v = parse_arguments(json_str);
//    let json = landingpad(clear_derived_key_internal);
//    CString::new(json).expect("ret json").into_raw()
//}
//
//fn clear_derived_key_internal() -> Result<String> {
//    let map: &mut HashMap<String, HdKeystore> = &mut KEYSTORE_MAP.write().unwrap();
//    map.values_mut()
//        .map(|keystore| {
//            keystore.crypto.clear_cache_derived_key();
//        })
//        .collect::<()>();
//    Ok(serde_json::to_string(&json!({ "ok": true }))?)
//}

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
pub unsafe extern "C" fn get_last_err() -> Buffer {
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            let rsp = Response {
                is_success: false,
                error: err.to_string(),
            };
            let mut rsp_bytes = encode_message(rsp).expect("encode error");
            wrap_buffer(rsp_bytes)
        } else {
            let mut rsp: Vec<u8> = vec![];
            wrap_buffer(rsp)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KEYSTORE_MAP, WALLET_FILE_DIR};
    use serde_json::Value;
    use std::ffi::{CStr, CString};
    use std::fs::remove_file;
    use std::os::raw::c_char;
    use std::panic;
    use std::path::Path;
    use std::str::FromStr;

    use crate::api::{InitTokenCoreXParam, WalletResult};
    use crate::init_token_core_x;
    use bytes::BytesMut;
    use prost::Message;
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
            let ks: &Keystore = map.get(WALLET_ID).unwrap();
            assert_eq!(ks.id(), WALLET_ID);
        });
    }

    //    #[test]
    //    fn init_token_core_x_pb_test() {
    //        run_test(|| {
    //            let hex = "0a0c2e2e2f746573742d646174611a203943304333303838394342434335453031414235423242423838373135373939";
    //            let mut hex_bytes = hex::decode(hex).unwrap();
    //            //            let mut buf = vec![0; 512].into_boxed_slice();
    //            //            let data = buf.as_mut_ptr();
    //            //            let len = buf.len();
    //            //            std::mem::forget(buf);
    //            //            Buffer { data, len }
    //            let param = Buffer {
    //                data: hex_bytes.as_mut_ptr(),
    //                len: hex_bytes.len(),
    //            };
    //            unsafe {
    //                init_token_core_x(param);
    //            }
    //
    //            let map = KEYSTORE_MAP.read().unwrap();
    //            let ks: &HdKeystore = map.get(WALLET_ID).unwrap();
    //            assert_eq!(ks.id, WALLET_ID);
    //        });
    //    }

    //    #[test]
    //    fn create_wallet_test() {
    //        run_test(|| {
    //            let params = r#"
    //        {
    //            "name": "createWalletTest",
    //            "password": "Insecure Password",
    //            "passwordHint": "Insecure Password",
    //            "source": "MNEMONIC"
    //        }
    //        "#;
    //            let json = _to_str(create_wallet(_to_c_char(params)));
    //            let v = Value::from_str(json).unwrap();
    //            let _expected = Value::from_str(params).unwrap();
    //            let id = v["id"].as_str().expect("wallet_id");
    //            assert_eq!(v["source"].as_str().unwrap(), "MNEMONIC");
    //            let map = KEYSTORE_MAP.read().unwrap();
    //            assert!(map.get(id).is_some());
    //            remove_created_wallet(id);
    //        })
    //    }
    //
    //    #[test]
    //    fn find_wallet_by_mnemonic_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"BITCOINCASH",
    //            "mnemonic":"blind gravity card grunt basket expect garment tilt organ concert great critic",
    //            "network":"MAINNET",
    //            "path":"m/44'/145'/0'/0/0",
    //            "segWit":"NONE"
    //            }"#;
    //            let ret = unsafe { _to_str(find_wallet_by_mnemonic(_to_c_char(param))) };
    //            assert_eq!("{}", ret);
    //
    //            let param = r#"{
    //            "chainType":"BITCOINCASH",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "network":"MAINNET",
    //            "path":"m/44'/145'/0'/0/0",
    //            "segWit":"NONE"
    //            }"#;
    //            let ret = unsafe { _to_str(find_wallet_by_mnemonic(_to_c_char(param))) };
    //            let v = Value::from_str(ret).expect("find wallet");
    //            assert_eq!(v["address"], "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r");
    //        })
    //    }
    //
    //    #[test]
    //    fn import_wallet_from_mnemonic_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"BITCOINCASH",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"BCH-Wallet-1",
    //            "network":"MAINNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/145'/0'/0/0",
    //            "segWit":"NONE",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let expected = r#"
    //            {
    //                "address": "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
    //                "chainType": "BITCOINCASH",
    //                "createdAt": 1566455834,
    //                "encXPub": "wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==",
    //                "externalAddress": {
    //                    "address": "qzyrtfn4a7cdkn7sp60tw7hl8zndt0tk0sst3p6qr5",
    //                    "derivedPath": "0/1",
    //                    "type": "EXTERNAL"
    //                },
    //                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
    //                "name": "BCH-Wallet-1",
    //                "passwordHint": "",
    //                "source": "MNEMONIC"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            let ret_v = Value::from_str(ret).unwrap();
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
    //            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
    //            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);
    //
    //            let imported_id = ret_v["id"].as_str().unwrap();
    //            let param = json!({
    //                "id": imported_id,
    //                "chainType": "BITCOINCASH",
    //                "externalIdx": 2
    //            });
    //
    //            let ret = unsafe {
    //                _to_str(calc_external_address(_to_c_char(
    //                    param.to_string().as_str(),
    //                )))
    //            };
    //            let ret_v: Value = Value::from_str(ret).unwrap();
    //            let expected = r#"
    //            {
    //                "address": "qzhsz3s4hr0f3x0v00zdn6w50tdpa9zgryp4kxgx49",
    //                "derivedPath": "0/2",
    //                "type": "EXTERNAL"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            assert_eq!(expected_v["derivedPath"], ret_v["derivedPath"]);
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            remove_created_wallet(imported_id);
    //        });
    //    }
    //
    //    #[test]
    //    fn import_wallet_from_mnemonic_testnet() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"BITCOINCASH",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"BCH-Wallet-1",
    //            "network":"TESTNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/1'/0'/0/0",
    //            "segWit":"NONE",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //            let expected = r#"
    //            {
    //                "address": "qqurlwqukz3lcujttcyvlzaagppnd4c37chrtrylmc",
    //                "chainType": "BITCOINCASH",
    //                "createdAt": 1566455834,
    //                "encXPub": "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==",
    //                "externalAddress": {
    //                    "address": "qqn4as4zx0jmy02rlgv700umavxt8xtpzus6u7flzk",
    //                    "derivedPath": "0/1",
    //                    "type": "EXTERNAL"
    //                },
    //                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
    //                "name": "BCH-Wallet-1",
    //                "passwordHint": "",
    //                "source": "MNEMONIC"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            let ret_v = Value::from_str(ret).unwrap();
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
    //            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
    //            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);
    //
    //            let imported_id = ret_v["id"].as_str().unwrap();
    //            let param = json!({
    //                "id": imported_id,
    //                "chainType": "BITCOINCASH",
    //                "network": "TESTNET",
    //                "externalIdx": 2
    //            });
    //
    //            let ret = unsafe {
    //                _to_str(calc_external_address(_to_c_char(
    //                    param.to_string().as_str(),
    //                )))
    //            };
    //            let ret_v: Value = Value::from_str(ret).unwrap();
    //            let expected = r#"
    //            {
    //                "address": "qqrhpq50f5n5sdgj0ehwz8qtrc3m6dnazghh3aj0ag",
    //                "derivedPath": "0/2",
    //                "type": "EXTERNAL"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            assert_eq!(expected_v["derivedPath"], ret_v["derivedPath"]);
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            remove_created_wallet(imported_id);
    //        });
    //    }

    /*
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
    */

    /*
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
    */
    //
    //    #[test]
    //    fn import_ltc_wallet_from_mnemonic_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"LITECOIN",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"LTC-Wallet-1",
    //            "network":"TESTNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/2'/0'/0/0",
    //            "segWit":"P2WPKH",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let expected = r#"
    //            {
    //                "address": "QLfctE6KMv3ZzQod6UA37w3EPTuLS4tg1T",
    //                "chainType": "LITECOIN",
    //                "createdAt": 1566455834,
    //                "encXPub": "k4GbrxWCcsrGokCos50O69Wg9reixsDqPHkciU4xeUi9dpICotcOMQSgTgRd7XtGXXjdV/SUuTBkPXNQikqORvvW2CnHNe7+iJsTdHebynq2Y3ZXMFUWt8WJkgB5NotqkjOik89LvJBKYKvnon2B0g==",
    //                "externalAddress": {
    //                    "address": "QPvKbnvZxAF1KVk5LfXbqtfnkwTymMf2Xu",
    //                    "derivedPath": "0/1",
    //                    "type": "EXTERNAL"
    //                },
    //                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
    //                "name": "LTC-Wallet-1",
    //                "passwordHint": "",
    //                "source": "MNEMONIC"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            let ret_v = Value::from_str(ret).unwrap();
    //
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
    //            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
    //            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);
    //
    //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
    //        });
    //    }

    //    #[test]
    //    fn import_legacy_ltc_wallet_from_mnemonic_mainnet() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"LITECOIN",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"LTC-Wallet-1",
    //            "network":"MAINNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/2'/0'/0/0",
    //            "segWit":"NONE",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let expected = r#"
    //            {
    //                "address": "Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP",
    //                "chainType": "LITECOIN",
    //                "createdAt": 1566455834,
    //                "encXPub": "MwDMFXVWDEuWvBogeW1v/MOMFDnGnnflm2JAPvJaJZO4HXp8fCsWETA7u8MzOW3KaPksglpUHLN3xkDr2QWMEQq0TewFZoZ3KsjmLW0KGMRN7XQKqo/omkSEsPfalVnp9Zxm2lpxVmIacqvlernVSg==",
    //                "externalAddress": {
    //                    "address": "LavE5eHDvw9VDiNifbraR7GyY8MRvcQSLQ",
    //                    "derivedPath": "0/1",
    //                    "type": "EXTERNAL"
    //                },
    //                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
    //                "name": "LTC-Wallet-1",
    //                "passwordHint": "",
    //                "source": "MNEMONIC"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            let ret_v = Value::from_str(ret).unwrap();
    //
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
    //            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
    //            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);
    //
    //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
    //        });
    //    }

    //    #[test]
    //    fn import_legacy_ltc_wallet_from_mnemonic_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"LITECOIN",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"LTC-Wallet-1",
    //            "network":"TESTNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/1'/0'/0/0",
    //            "segWit":"NONE",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let expected = r#"
    //            {
    //                "address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
    //                "chainType": "LITECOIN",
    //                "createdAt": 1566455834,
    //                "encXPub": "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==",
    //                "externalAddress": {
    //                    "address": "mj78AbVtQ9SWnvbU7pcrueyE1krMmZtoUU",
    //                    "derivedPath": "0/1",
    //                    "type": "EXTERNAL"
    //                },
    //                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
    //                "name": "LTC-Wallet-1",
    //                "passwordHint": "",
    //                "source": "MNEMONIC"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            let ret_v = Value::from_str(ret).unwrap();
    //
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
    //            assert_eq!(expected_v["encXPub"], ret_v["encXPub"]);
    //            assert_eq!(expected_v["externalAddress"], ret_v["externalAddress"]);
    //
    //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
    //        });
    //    }

    //    #[test]
    //    fn remove_wallet_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"LITECOIN",
    //            "mnemonic":"calm release clay imitate top extend close draw quiz refuse shuffle injury",
    //            "name":"LTC-Wallet-1",
    //            "network":"MAINNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/1'/0'/0/0",
    //            "segWit":"NONE",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let ret_v = Value::from_str(ret).unwrap();
    //            let imported_id = ret_v["id"].as_str().expect("wallet_id");
    //            let param = json!({
    //                "id": imported_id,
    //                "password": "Insecure Password"
    //            });
    //            let param = serde_json::to_string(&param).unwrap();
    //            let ret = unsafe { _to_str(remove_wallet(_to_c_char(&param))) };
    //            let ret_v = Value::from_str(ret).unwrap();
    //            assert_eq!(ret_v["id"], imported_id);
    //
    //            //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
    //        });
    //    }
    //
    //    #[test]
    //    fn import_trx_wallet_from_mnemonic_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"TRON",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"TRX-Wallet-1",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/195'/0'/0/0",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let expected = r#"
    //            {
    //                "address": "TY2uroBeZ5trA9QT96aEWj32XLkAAhQ9R2",
    //                "chainType": "TRON",
    //                "createdAt": 1566455834,
    //                "id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
    //                "name": "LTC-Wallet-1",
    //                "passwordHint": "",
    //                "source": "MNEMONIC"
    //            }
    //            "#;
    //            let expected_v = Value::from_str(expected).expect("from expected");
    //            let ret_v = Value::from_str(ret).unwrap();
    //
    //            assert_eq!(expected_v["address"], ret_v["address"]);
    //            assert_eq!(expected_v["chainType"], ret_v["chainType"]);
    //
    //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
    //        });
    //    }

    //    #[test]
    //    fn sign_trx_message_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"TRON",
    //            "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            "name":"TRX-Wallet-1",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/195'/0'/0/0",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let ret_v = Value::from_str(ret).unwrap();
    //
    //            let param = json!({
    //                "id": ret_v["id"].as_str().expect("wallet_id"),
    //                "chainType": "TRON",
    //                "password": "Insecure Password",
    //                "value": "0xaaaaaaaa",
    //                "isHex": true,
    //                "isTronHeader": true
    //            });
    //            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };
    //
    //            assert_eq!("47fb89c1a3726de25f64b0d98dd8ca3c12079c12cec31a35ac71d7ce337cc4df02fec800ee1c149b9cb9f79e9f60f665a4a1bf00be20b7fbca7007f9a0076d731c", signed);
    //
    //            let param = json!({
    //                "id": ret_v["id"].as_str().expect("wallet_id"),
    //                "chainType": "TRON",
    //                "password": "Insecure Password",
    //                "value": "aaaaaaaa",
    //                "isHex": true,
    //                "isTronHeader": true
    //            });
    //            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };
    //
    //            assert_eq!("47fb89c1a3726de25f64b0d98dd8ca3c12079c12cec31a35ac71d7ce337cc4df02fec800ee1c149b9cb9f79e9f60f665a4a1bf00be20b7fbca7007f9a0076d731c", signed);
    //
    //            let param = json!({
    //                "id": ret_v["id"].as_str().expect("wallet_id"),
    //                "chainType": "TRON",
    //                "password": "Insecure Password",
    //                "value": "abc",
    //                "isHex": false,
    //                "isTronHeader": true
    //            });
    //            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };
    //
    //            assert_eq!("f61b5966ca46dd838586f96dddf3fe594980f04c783492c240edcb3a5dd6c49b5f9ca8172e222943a61e177debad0dc374f80d4fe90a0a52b8607a1447225fd21b", signed);
    //
    //            let param = json!({
    //                "id": ret_v["id"].as_str().expect("wallet_id"),
    //                "chainType": "TRON",
    //                "password": "Insecure Password",
    //                "value": "abc",
    //                "isHex": false,
    //                "isTronHeader": false
    //            });
    //            let signed = unsafe { _to_str(sign_message(_to_c_char(&param.to_string()))) };
    //
    //            assert_eq!("b256bb5fa285d981fb424f997c34ff9575eca7c0ec26f47141dfae058ecc7ada40f2ee3916c183fc8b3e0c810051756a9f1307d9f4e9b883a98a8b4ebce74ce51b", signed);
    //
    //            remove_created_wallet(ret_v["id"].as_str().expect("wallet_id"));
    //        });
    //    }

    //    #[test]
    //    fn export_mnemonic_test() {
    //        run_test(|| {
    //            let param = r#"
    //        {
    //            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
    //            "password": "Insecure Password"
    //        }
    //        "#;
    //            unsafe { clear_err() }
    //            let exported_mnemonic = unsafe { _to_str(export_mnemonic(_to_c_char(param))) };
    //            let _err = unsafe { _to_str(get_last_err_message()) };
    //            let expected_v = Value::from_str(r#"{"mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch","ok":true}"#).unwrap();
    //            let actual_v = Value::from_str(exported_mnemonic).unwrap();
    //            assert_eq!(actual_v, expected_v);
    //
    //            let param = r#"
    //        {
    //            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
    //            "password": "Wrong Password"
    //        }
    //        "#;
    //            unsafe { clear_err() }
    //            let _exported_mnemonic = unsafe { _to_str(export_mnemonic(_to_c_char(param))) };
    //            let err = unsafe { _to_str(get_last_err_message()) };
    //            assert_eq!(err, "password_incorrect");
    //        })
    //    }

    //    #[test]
    //    fn sign_transaction_test() {
    //        run_test(|| {
    //            let param = r#"
    //            {
    //                "id":"9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
    //                "password": "Insecure Password",
    //                "to": "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s",
    //                "amount": "93454",
    //                "fee": "6000",
    //                "internalUsed": 0,
    //                "chainType": "BITCOINCASH",
    //                "chainId": "145",
    //                "segWit":"NONE",
    //                "outputs": [
    //                    {
    //                        "txHash": "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2",
    //                        "vout": 0,
    //                        "amount": "100000",
    //                        "address": "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
    //                        "scriptPubKey": "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac",
    //                        "derivedPath": "0/0"
    //                    }
    //                ]
    //            }
    //            "#;
    //
    //            unsafe { clear_err() }
    //            let ret = unsafe { _to_str(sign_transaction(_to_c_char(param))) };
    //            let ret_v = Value::from_str(ret).unwrap();
    //            let expected = r#"{"sign":"0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006b483045022100b3d91f406cdc33eb4d8f2b56491e6c87da2372eb83f1f384fc3f02f81a5b21b50220324dd7ecdc214721c542db252078473f9e7172bf592fa55332621c3e348be45041210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a9148835a675efb0db4fd00e9eb77aff38a6d5bd767c88ac00000000","hash":"4d43cc66e9763a4e263fdb592591b9f19a6915ac821c92896d13f95beaca3b28","wtxId":""}"#;
    //            let expected_v = Value::from_str(expected).unwrap();
    //            assert_eq!(ret_v, expected_v);
    //        })
    //    }

    //    #[test]
    //    fn cache_derived_key_test() {
    //        run_test(|| {
    //            let param = r#"{
    //            "chainType":"LITECOIN",
    //            "mnemonic":"salute slush now script nest law admit achieve voice soda fruit field",
    //            "name":"LTC-Wallet-1",
    //            "network":"MAINNET",
    //            "overwrite":true,
    //            "password":"Insecure Password",
    //            "passwordHint":"",
    //            "path":"m/44'/1'/0'/0/0",
    //            "segWit":"NONE",
    //            "source":"MNEMONIC"
    //            }"#;
    //            let ret = unsafe { _to_str(import_wallet_from_mnemonic(_to_c_char(param))) };
    //
    //            let ret_v = Value::from_str(ret).unwrap();
    //            let imported_id = ret_v["id"].as_str().expect("wallet_id");
    //            let param = json!({
    //                "id": imported_id,
    //                "password": "Insecure Password"
    //            });
    //
    //            let derived_key =
    //                unsafe { _to_str(get_derived_key(_to_c_char(param.to_string().as_str()))) };
    //
    //            let param = json!({
    //                "id": imported_id,
    //                "derivedKey": derived_key
    //            });
    //            let ret =
    //                unsafe { _to_str(verify_derived_key(_to_c_char(param.to_string().as_str()))) };
    //            let ret_v: Value = serde_json::from_str(ret).unwrap();
    //            assert_eq!(derived_key, ret_v["derivedKey"].as_str().unwrap());
    //
    //            let param = json!({
    //                "id": imported_id,
    //                "derivedKey": "1111111111111111111111111111111111111111111111111111111111111111"
    //            });
    //            let _ret =
    //                unsafe { _to_str(verify_derived_key(_to_c_char(param.to_string().as_str()))) };
    //            let err = unsafe { _to_str(get_last_err_message()) };
    //            assert_eq!("invalid_cached_derived_key", err);
    //
    //            let param: Value =
    //                json!({"id": imported_id, "tempPassword": "88888888", "derivedKey": derived_key});
    //            unsafe { _to_str(cache_derived_key(_to_c_char(param.to_string().as_str()))) };
    //
    //            let param = json!({
    //                "id": imported_id,
    //                "password": "888888"
    //            });
    //
    //            unsafe {
    //                clear_err();
    //            }
    //            let _ = unsafe { export_mnemonic(_to_c_char(param.to_string().as_str())) };
    //            let err = unsafe { _to_str(get_last_err_message()) };
    //            assert_eq!("password_incorrect", err);
    //
    //            let param = json!({
    //                "id": imported_id,
    //                "password": "88888888"
    //            });
    //
    //            unsafe {
    //                clear_err();
    //            }
    //            let exported_mnemonic =
    //                unsafe { _to_str(export_mnemonic(_to_c_char(param.to_string().as_str()))) };
    //            assert_eq!(
    //                r#"{"mnemonic":"salute slush now script nest law admit achieve voice soda fruit field","ok":true}"#,
    //                exported_mnemonic
    //            );
    //            unsafe { clear_derived_key() };
    //
    //            remove_created_wallet(imported_id);
    //        })
    //    }
    //
    //    #[test]
    //    fn verify_password_test() {
    //        run_test(|| {
    //            let param = r#"
    //        {
    //            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
    //            "password": "Wrong Password"
    //        }
    //        "#;
    //            let _ = unsafe { _to_str(verify_password(_to_c_char(param))) };
    //            let err = unsafe { _to_str(get_last_err_message()) };
    //            assert_eq!(err, "password_incorrect");
    //
    //            let param = r#"
    //        {
    //            "id": "9c6cbc21-1c43-4c8b-bb7a-5e538f908819",
    //            "password": "Insecure Password"
    //        }
    //        "#;
    //            let ret = unsafe { _to_str(verify_password(_to_c_char(param))) };
    //            let v: Value = serde_json::from_str(ret).unwrap();
    //            assert!(v["ok"].as_bool().unwrap())
    //        })
    //    }
    //
    //    #[test]
    //    fn test_proto() {
    //        let param = InitTokenCoreXParam {
    //            file_dir: "aaa".to_string(),
    //            xpub_common_key: "aaa".to_string(),
    //            xpub_common_iv: "aaa".to_string(),
    //        };
    //        let mut buf = BytesMut::with_capacity(1024);
    //        param.encode(&mut buf).unwrap();
    //        let param: InitTokenCoreXParam = InitTokenCoreXParam::decode(&buf).unwrap();
    //        assert_eq!("aaa", param.file_dir)
    //    }

    #[test]
    fn test_call_tcx_api() {
        run_test(|| {
            let param_hex = "0a0f68645f73746f72655f696d706f727412bb010a166170692e486453746f7265496d706f7274506172616d12a0010a084c495445434f494e124573616c75746520736c757368206e6f7720736372697074206e657374206c61772061646d6974206163686965766520766f69636520736f6461206672756974206669656c641a11496e7365637572652050617373776f7264220f6d2f3434272f31272f30272f302f302a084d4e454d4f4e4943320c4c54432d57616c6c65742d313a074d41494e4e455442044e4f4e454a005001";
            let mut param_bytes = hex::decode(param_hex).unwrap();
            let param_buf = Buffer {
                data: param_bytes.as_mut_ptr(),
                len: param_bytes.len(),
            };
            let ret_buf = unsafe { call_tcx_api(param_buf) };
            let ret_bytes = unsafe { Vec::from_raw_parts(ret_buf.data, ret_buf.len, ret_buf.len) };
            let ret: WalletResult = WalletResult::decode(ret_bytes).unwrap();
            assert_eq!(
                "LRB53mz8PmBPDBH8HFp3f5bVHxJ9Bqx8PH",
                ret.accounts.first().unwrap().address
            );
        });
    }
}
