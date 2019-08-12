pub mod aes;
pub mod crypto;
pub mod numberic_util;

use core::result;
pub use crypto::{Crypto, EncPair, Pbkdf2Params};

#[macro_use]
extern crate failure;

pub type Result<T> = result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
