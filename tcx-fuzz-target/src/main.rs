#[macro_use]
extern crate afl;
extern crate tcx;
use std::ffi::{CStr, CString};

use std::os::raw::c_char;

fn _to_c_char(str: &str) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn main() {
    let init_bytes = hex::decode("0a0c2e2e2f746573742d646174611a203943304333303838394342434335453031414235423242423838373135373939").unwrap();
    tcx::handler::init_token_core_x(&init_bytes);

    fuzz!(|data: &[u8]| {
        let mut bytes = data.to_vec();
        let buf = tcx::wrap_buffer(&mut bytes);
        let _ = unsafe { tcx::call_tcx_api(buf) };
    });
}
