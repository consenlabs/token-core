use std::mem;
use std::panic;
use std::thread;
use std::cell::RefCell;
use core::result;
use failure::Fail;
use failure::Error;
use failure::Backtrace;
use core::borrow::BorrowMut;
//use std::error::Error;

// ref: https://github.com/getsentry/symbolic/blob/10d3f31057/cabi/src/utils.rs
//
//#[derive(Debug, Fail)]
//pub enum Error {
//    #[fail(display = "invalid toolchain name: {}", name)]
//    InvalidToolchainName {
//        name: String,
//    },
//    #[fail(display = "unknown toolchain version: {}", version)]
//    UnknownToolchainVersion {
//        version: String,
//    },
//    #[fail(display = "{}", msg)]
//    Msg {
//        msg: String
//    }
//
//}

pub type Result<T> = result::Result<T, Error>;

thread_local! {
    pub static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
    pub static LAST_BACKTRACE: RefCell<Option<(Option<String>, Backtrace)>> = RefCell::new(None);
}


fn notify_err(err: Error) {
    if let backtrace = err.backtrace() {
        LAST_BACKTRACE.with(|e| {
            *e.borrow_mut() = Some((None, Backtrace::new()));
        });
    }
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(err);
    });
}

pub unsafe fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::new();
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");

        let msg = match info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                }
            }
        };

        let panic_info = match info.location() {
            Some(location) => {
                format!("thread '{}' panicked with '{}' at {}:{}",
                        thread, msg, location.file(),
                        location.line())
            }
            None => {
                format!("thread '{}' panicked with '{}'", thread, msg)
            }
        };

        LAST_BACKTRACE.with(|e| {
            *e.borrow_mut() = Some((Some(panic_info), backtrace));
        });
    }));
}

pub unsafe fn landingpad<F: FnOnce() -> Result<T> + panic::UnwindSafe, T>(
    f: F) -> T
{
    match panic::catch_unwind(f) {
        Ok(rv) => rv.map_err(|err| notify_err(err)).unwrap_or(mem::zeroed()),
        Err(err) => {
            use std::any::Any;
            let err = &*err as &Any;
            let msg = match err.downcast_ref::<&str>() {
                Some(s) => *s,
                None => {
                    match err.downcast_ref::<String>() {
                        Some(s) => &**s,
                        None => "Box<Any>",
                    }
                }
            };
            notify_err(format_err!("{}", msg));
            mem::zeroed()
        }
    }
}

macro_rules! ffi_fn (
    // a function that catches patnics and returns a result (err goes to tls)
    (
        $(#[$attr:meta])*
        unsafe fn $name:ident($($aname:ident: $aty:ty),* $(,)*) -> Result<$rv:ty> $body:block
    ) => (
        #[no_mangle]
        $(#[$attr])*
        pub unsafe extern "C" fn $name($($aname: $aty,)*) -> $rv
        {
            $crate::utils::landingpad(|| $body)
        }
    );

    // a function that catches patnics and returns nothing (err goes to tls)
    (
        $(#[$attr:meta])*
        unsafe fn $name:ident($($aname:ident: $aty:ty),* $(,)*) $body:block
    ) => {
        #[no_mangle]
        $(#[$attr])*
        pub unsafe extern "C" fn $name($($aname: $aty,)*)
        {
            // this silences panics and stuff
            $crate::utils::landingpad(|| { $body; Ok(0 as ::std::os::raw::c_int) });
        }
    }
);