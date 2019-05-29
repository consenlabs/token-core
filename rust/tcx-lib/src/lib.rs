use std::ffi::{CString, CStr};
use libc::{size_t, c_int};
use std::os::raw::c_char;
use log::Level;
use std::fs::File;
use std::io::Read;

// #[link(name = "TrezorCrypto")]
// extern {
//     fn mnemonic_generate(strength: c_int, mnemonic: *mut c_char) -> c_int;
// }

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn readFile(file_path: *const c_char) -> *const c_char {
    let c_str = unsafe { CStr::from_ptr(file_path) };
    let file_path = c_str.to_str().unwrap();
    // let filePath: String = env.get_string(filePath).expect("Couldn't get java string!").into();
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    CString::new(contents).unwrap().into_raw()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
