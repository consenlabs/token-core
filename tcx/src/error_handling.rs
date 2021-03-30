use crate::filemanager::KEYSTORE_MAP;
use core::result;
use failure::Error;
use std::{cell::RefCell, panic};
pub type Result<T> = result::Result<T, Error>;
use log::error;
use crate::api::Response;
use crate::encode_message;

fn lock_all_keystore() {
    let mut map = KEYSTORE_MAP.write();
    for ks in map.values_mut() {
        ks.lock();
    }
}

/// catch any error and format to string
/// ref: <https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/controlling-panics-with-std-panic.html>
#[cfg_attr(tarpaulin, skip)]
pub unsafe fn landingpad<F: FnOnce() -> Result<Vec<u8>> + panic::UnwindSafe>(f: F) -> Vec<u8> {
    match panic::catch_unwind(f) {
        Ok(rv) => {
            lock_all_keystore();
            match rv {
                Ok(v) => {
                    let res = Response {
                        is_success: true,
                        error: "".to_string(),
                        value: Some(::prost_types::Any {
                            type_url: "bool_wallet".to_string(),
                            value: v
                        })
                    };
                    encode_message(res).unwrap()
                },
                Err(e) => {
                    let err_msg = format!("Error: {}", e.to_string());
                    let res = Response {
                        is_success: false,
                        error: err_msg,
                        value: None
                    };
                    encode_message(res).unwrap()
                }
            }
        }
        Err(err) => {
            lock_all_keystore();
            use std::any::Any;
            let err = &*err as &dyn Any;
            let msg = match err.downcast_ref::<&str>() {
                Some(s) => *s,
                None => match err.downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };
            let msg = format!("Error: {}", msg);
            let res = Response {
                is_success: false,
                error: msg,
                value: None
            };
            encode_message(res).unwrap()
        }
    }
}
