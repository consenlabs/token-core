use failure::Error;
use std::fmt;
use std::result;

#[derive(Fail, Debug)]
pub enum CryptoError {
    #[fail(display = "invalid_password")]
    InvalidPassword,
}

pub type Result<T> = result::Result<T, Error>;

//#[derive(Debug)]
//pub struct TokenError {
//    err: &'static str,
//}
////
////#[derive(Debug)]
////enum CliError {
////    InvalidMnemonic(),
////    Parse(num::ParseIntError),
////}
//
//
//impl fmt::Display for TokenError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.err)
//    }
//}
//
//impl Error for TokenError {
//    fn description(&self) -> &str {
//        return self.err;
//    }
//    fn cause(&self) -> Option<&Error> {
//        None
//    }
//}
//
//
//
//impl TokenError {
//    pub fn from(msg: &'static str) -> TokenError {
//        return TokenError {
//            err: msg
//        }
//    }
//}
//
//impl From<secp256k1::Error> for TokenError {
//    fn from(err: secp256k1::Error) -> TokenError {
//        TokenError::from("secp error")
//    }
//}
//
//
//
//
//type Result<T> = result::Result<T, TokenError>;

