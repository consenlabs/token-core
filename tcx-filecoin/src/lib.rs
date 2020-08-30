use failure::Fail;

mod address;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_curve_type")]
    InvalidCurveType,
}
