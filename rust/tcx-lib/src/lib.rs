use std::ffi::{CString, CStr};
use libc::{size_t, c_int};
use std::os::raw::c_char;
use log::Level;
use std::fs::File;
use std::io::Read;
use utils::Error;

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


ffi_fn! {
    /// Creates a symcache from bytes
    unsafe fn read_file_error(bytes: *const u8, len: usize) -> Result<*const c_char> {
        Err(Error::Msg { msg: String::from("read file error")})
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
