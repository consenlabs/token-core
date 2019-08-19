pub mod aes;
pub mod crypto;
pub mod numberic_util;

use core::result;
pub use crypto::{Crypto, EncPair, Pbkdf2Params};

#[macro_use]
extern crate failure;
#[macro_use]
extern crate hex_literal;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "kdf_params_invalid")]
    KdfParamsInvalid,
    #[fail(display = "invalid_password")]
    InvalidPassword,
    #[fail(display = "invalid_key_iv_length")]
    InvalidKeyIvLength,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
