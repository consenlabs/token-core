pub mod crypto;
pub mod aes;
pub mod numberic_util;
pub mod error;
pub mod key;

use failure::Error;
use core::result;

pub use crypto::{Crypto, Pbkdf2Params, EncPair};

pub type Result<T> = result::Result<T, Error>;

pub type H256 = [u8; 32];
pub type H128 = [u8; 16];
pub type H160 = [u8; 20];

#[macro_use] extern crate failure;


