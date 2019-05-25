use std::error::Error;
use std::fmt;

///
#[derive(Debug)]
pub struct TokenError {
    err: &'static str,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl Error for TokenError {
    fn description(&self) -> &str {
        return self.err;
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}



impl TokenError {
    pub fn from(msg: &'static str) -> TokenError {
        return TokenError {
            err: msg
        }
    }
}
