use bitcoin::util::bip32::Error as BtcError;
use crate::errors::Error::Bip32Error;
use core::result;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid_mnemonic")]
    InvalidMnemonic,
    #[fail(display = "bip32_error")]
    Bip32Error,
    #[fail(display = "{}", msg)]
    Msg {
        msg: String
    }

}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Self {
        Error::InvalidMnemonic
    }
}

impl From<bitcoin::util::bip32::Error> for Error {
    fn from(err: bitcoin::util::bip32::Error) -> Self {
        Error::Bip32Error
    }
}

pub type Result<T> = result::Result<T, Error>;
