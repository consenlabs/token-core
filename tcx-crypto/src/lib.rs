pub mod aes;
pub mod crypto;
pub mod error;
pub mod numberic_util;

pub use crypto::{Crypto, EncPair, Pbkdf2Params};

#[macro_use]
extern crate failure;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
