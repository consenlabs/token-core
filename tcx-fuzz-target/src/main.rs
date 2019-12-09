#[macro_use]
extern crate afl;
extern crate tcx;
use std::ffi::{CStr, CString};
use tcx::api::InitTokenCoreXParam;

use std::os::raw::c_char;

fn _to_c_char(str: &str) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn main() {
    //    let init_bytes = hex::decode("0a17696e69745f746f6b656e5f636f72655f785f706172616d12530a176170692e496e6974546f6b656e436f726558506172616d12380a142f746d702f696d746f6b656e2f77616c6c6574731a203943304333303838394342434335453031414235423242423838373135373939").unwrap();
    let param = InitTokenCoreXParam {
        file_dir: "/tmp/imtoken/wallets".to_string(),
        xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
        xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
    };

    tcx::handler::init_token_core_x(&tcx::handler::encode_message(param).unwrap());

    //    fuzz!(|data: &[u8]| {
    ////        println!("{}", hex::encode(data));
    //        let mut bytes = data.to_vec();
    //        let buf = tcx::wrap_buffer(bytes);
    //        let _ = unsafe { tcx::call_tcx_api(buf) };
    //    });
}
