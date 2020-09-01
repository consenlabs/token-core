use failure::Fail;

mod address;
mod signer;
mod transaction;
mod utils;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_curve_type")]
    InvalidCurveType,
}
