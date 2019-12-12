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

pub mod api;
use crate::api::{Response, TcxAction};
pub mod error_handling;
pub mod handler;
use crate::error_handling::{landingpad, Result, LAST_BACKTRACE, LAST_ERROR};
use crate::handler::{
    encode_message, hd_store_create, hd_store_derive, hd_store_export, hd_store_import,
    keystore_common_accounts, keystore_common_delete, keystore_common_exists,
    keystore_common_verify, private_key_store_export, private_key_store_import, sign_tx,
    tron_sign_message, Buffer,
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
            landingpad(|| keystore_common_accounts(&action.param.unwrap().value))
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

    use crate::api::{InitTokenCoreXParam, KeystoreCommonExistsResult, WalletResult};
    use crate::init_token_core_x;
    use bytes::BytesMut;
    use prost::Message;
    use tcx_chain::HdKeystore;

    static WALLET_ID: &'static str = "7719d1e3-3f67-439f-a18e-d9ae413e00e1";

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
    //    fn test_call_tcx_api() {
    //        run_test(|| {
    //            let param_hex = "0a0f68645f73746f72655f64657269766512730a166170692e486453746f7265446572697665506172616d12590a2437626262656262662d636565662d343761622d386639372d30373239623861316132616312093132333132333132331a260a084c495445434f494e120f6d2f3434272f32272f30272f302f301a074d41494e4e45542a00";
    //            let mut param_bytes = hex::decode(param_hex).unwrap();
    //            let param_buf = Buffer {
    //                data: param_bytes.as_mut_ptr(),
    //                len: param_bytes.len(),
    //            };
    //            let ret_buf = unsafe { call_tcx_api(param_buf) };
    //            let ret_bytes = unsafe { Vec::from_raw_parts(ret_buf.data, ret_buf.len, ret_buf.len) };
    //            let ret: WalletResult = WalletResult::decode(ret_bytes).unwrap();
    //            assert_eq!(
    //                "LRB53mz8PmBPDBH8HFp3f5bVHxJ9Bqx8PH",
    //                ret.accounts.first().unwrap().address
    //            );
    //        });
    //    }

    #[test]
    fn test_encode_empty_struct() {
        //        let param: KeystoreCommonExistsResult = KeystoreCommonExistsResult {
        //            is_exists: false,
        //            id: "".to_string()
        //        };
        //        let hex_value = hex::encode(encode_message(param).unwrap());
        //        assert_eq!("08001200", hex_value);
        let bytes = hex::decode("1211756e737570706f727465645f636861696e").unwrap();
        let rsp = Response::decode(bytes);
        println!("{:?}", rsp);
        //        let param: KeystoreCommonExistsResult = KeystoreCommonExistsResult::decode(bytes).unwrap();
        //        let param2: KeystoreCommonExistsResult =
        //            KeystoreCommonExistsResult::decode(vec![]).unwrap();
        //        assert_eq!(param.is_exists, param2.is_exists);
    }
}
