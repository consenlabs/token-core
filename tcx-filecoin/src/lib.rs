use failure::Fail;

mod address;
mod signer;
mod transaction;
mod utils;

pub use crate::address::FilecoinAddress;
pub use crate::transaction::{SignedMessage, UnsignedMessage};

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_curve_type")]
    InvalidCurveType,

    #[fail(display = "cannot_found_account")]
    CannotFoundAccount,
}
