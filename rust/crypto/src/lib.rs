
pub mod crypto;
pub mod aes;
pub mod numberic_util;
pub mod error;
pub mod curve;

pub use crypto::{Crypto, Pbkdf2Params, EncPair};

#[macro_use] extern crate failure;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
