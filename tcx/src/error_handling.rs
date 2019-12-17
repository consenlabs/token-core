use core::result;
use failure::{Backtrace, Error};
use std::{cell::RefCell, mem, panic};

pub type Result<T> = result::Result<T, Error>;

thread_local! {
    pub static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
    pub static LAST_BACKTRACE: RefCell<Option<(Option<String>, Backtrace)>> = RefCell::new(None);
}

#[allow(irrefutable_let_patterns)]
fn notify_err(err: Error) {
    if let _backtrace = err.backtrace() {
        LAST_BACKTRACE.with(|e| {
            *e.borrow_mut() = Some((None, Backtrace::new()));
        });
    }
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(err);
    });
}

/// catch any error and format to string
/// ref: <https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/controlling-panics-with-std-panic.html>
pub unsafe fn landingpad<F: FnOnce() -> Result<T> + panic::UnwindSafe, T>(f: F) -> T {
    match panic::catch_unwind(f) {
        Ok(rv) => rv.map_err(notify_err).unwrap_or_else(|_| mem::zeroed()),
        Err(err) => {
            use std::any::Any;
            let err = &*err as &dyn Any;
            let msg = match err.downcast_ref::<&str>() {
                Some(s) => *s,
                None => match err.downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };
            notify_err(format_err!("{}", msg));
            mem::zeroed()
        }
    }
}
