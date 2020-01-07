pub mod aes;
pub mod crypto;
pub mod hash;
pub mod numberic_util;

use core::result;
pub use crypto::{Crypto, EncPair, Key, Pbkdf2Params};
use parking_lot::RwLock;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum Error {
    #[fail(display = "kdf_params_invalid")]
    KdfParamsInvalid,
    #[fail(display = "password_incorrect")]
    PasswordIncorrect,
    #[fail(display = "invalid_key_iv_length")]
    InvalidKeyIvLength,
    #[fail(display = "invalid_ciphertext")]
    InvalidCiphertext,
    #[fail(display = "cached_dk_feature_not_support")]
    CachedDkFeatureNotSupport,
}

lazy_static! {
    pub static ref XPUB_COMMON_KEY_128: RwLock<String> =
        RwLock::new("B888D25EC8C12BD5043777B1AC49F872".to_string());
    pub static ref XPUB_COMMON_IV: RwLock<String> =
        RwLock::new("9C0C30889CBCC5E01AB5B2BB88715799".to_string());
    pub static ref KDF_ROUNDS: RwLock<i32> = RwLock::new(262144);
}
