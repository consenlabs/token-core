use std::str::FromStr;
use std::fmt::{self, Write};

use super::Result;
use std::clone::Clone;

use super::H256;

#[derive(Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub x: H256,
    pub y: H256,
    pub compressed: bool,
}

impl PublicKey {
   pub fn as_bytes(&self) -> &[u8] {
       &self.x
   }
}

impl fmt::Display for PublicKey {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", hex::encode(self.as_bytes()));

       Ok(())
   }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_bytes()));

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub key: H256,
    pub compressed: bool,
}

impl PrivateKey {
    pub fn as_bytes(&self) -> &H256 {
        &self.key
    }

    pub fn from_slice(data: &H256) -> Result<PrivateKey> {
        //TODO
        unimplemented!()
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.key));

        Ok(())
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[private key data]");

        Ok(())
    }
}

