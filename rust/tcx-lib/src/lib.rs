use std::ffi::{CString, CStr};
use libc::{size_t, c_int};
use std::os::raw::c_char;
use log::Level;
use std::fs::File;
use std::io::Read;
use utils::Error;
use utils::Result;
use utils::LAST_BACKTRACE;
use utils::LAST_ERROR;
use failure::Fail;

// #[link(name = "TrezorCrypto")]
// extern {
//     fn mnemonic_generate(strength: c_int, mnemonic: *mut c_char) -> c_int;
// }
//pub mod utils;

#[macro_use] extern crate failure;
#[macro_use] pub mod utils;



#[no_mangle]
pub extern fn read_file(file_path: *const c_char) -> *const c_char {
    let c_str = unsafe { CStr::from_ptr(file_path) };
    let file_path = c_str.to_str().unwrap();
    // let filePath: String = env.get_string(filePath).expect("Couldn't get java string!").into();
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    CString::new(contents).unwrap().into_raw()
}

#[no_mangle]
pub extern fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern fn free_const_string(s: *const c_char) {
    unsafe {
        if s.is_null() { return }
        CStr::from_ptr(s)
    };
}

#[no_mangle]
pub unsafe extern "C" fn read_file_error() -> *const c_char {
    crate::utils::landingpad(||
        {
            Err(Error::Msg{msg:
            String::from("read file error"),})
        })
}

#[no_mangle]
pub unsafe extern "C" fn get_last_err_message() -> *const c_char{
    use std::fmt::Write;
    use std::error::Error;
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            let mut msg = err.to_string();
            let mut cause = err.cause();
            while let Some(the_cause) = cause {
                write!(&mut msg, "\n  caused by: {}", the_cause).ok();
                cause = the_cause.cause();
            }
            return CString::new(msg).unwrap().into_raw();
        } else {
//            Default::default()
            return CString::new("no error").unwrap().into_raw();
        }
    })
}
//
//ffi_fn! {
//    /// Creates a symcache from bytes
//    unsafe fn read_file_error() -> Result<*const c_char> {
//        Err(Error::Msg { msg: String::from("read file error")})
//    }
//}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
