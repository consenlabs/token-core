#![cfg(target_os="android")]
#![allow(non_snake_case)]

extern crate libc;
#[macro_use] extern crate log;
extern crate android_logger;

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString, JClass};
use jni::sys::{jstring, jint};
use libc::{size_t, c_int};
use std::os::raw::c_char;

#[link(name = "TrezorCrypto")]
extern {
    fn mnemonic_generate(strength: c_int, mnemonic: *mut c_char) -> c_int;
}

use log::Level;
use android_logger::{Config,FilterBuilder};

#[no_mangle]
pub unsafe extern fn Java_com_consenlabs_android_tokencoreexample_MainActivity_initLog(env: JNIEnv, _: JClass)  {
    // android_logger::init_once(
    //     Config::default()
    //         .with_min_level(Level::Trace) // limit log level
    //         .with_tag("mytag") // logs will show under mytag tag
    //         .with_filter( // configure messages for specific crate
    //             FilterBuilder::new()
    //                 .parse("debug,hello::crate=error")
    //                 .build())
    // );

    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    
}


#[no_mangle]
pub unsafe extern fn Java_com_consenlabs_android_tokencoreexample_MainActivity_generateMnemonic(env: JNIEnv, _: JClass, strength: jint) -> jstring {
    debug!("calling hello");
    let strength = strength as i32;
    // let mut dst: Vec<u8> = Vec::with_capacity(240);
    // let pdst = dst.as_mut_ptr();
    let c_string = CString::new("").expect("CString::new failed");
    let ptr = c_string.into_raw();
    mnemonic_generate(strength, ptr);
    let s = CString::from_raw(ptr).to_string_lossy().into_owned();
    debug!("result: {}", s);
    
    
    let output = env.new_string(format!("{}", s)).expect("Couldn't create java string!");
    std::mem::forget(output);
    output.into_inner()
}

