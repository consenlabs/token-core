#[macro_use]
extern crate afl;
extern crate tcx;
use std::ffi::{CStr, CString};

use std::os::raw::c_char;


fn _to_c_char(str: &str) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn main() {
   fuzz!(|data: &[u8]| {
       if let Ok(s) = std::str::from_utf8(data) {
           let _ = unsafe {
               tcx::init_token_core_x(_to_c_char(s));
           };
       }
   });
}