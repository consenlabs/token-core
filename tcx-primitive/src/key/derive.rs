use crate::key::KeyError;
use bitcoin::util::bip32::ChildNumber;
use std::convert::TryInto;

pub enum DeriveJunction {
    Soft(u32),
    Hard(u32),
}

pub trait Derive: Sized {
    type Error;

    fn derive<Iter: Iterator<Item=DeriveJunction>>(&self, path: Iter) -> Result<Self, Self::Error> ;
}

// TODO add parity string derivation path
#[allow(dead_code)]
impl DeriveJunction {
    pub fn soft(index: u32) -> Self {
        DeriveJunction::Soft(index)
    }

    pub fn hard(index: u32) -> Self {
        DeriveJunction::Hard(index)
    }

    pub fn is_soft(&self) -> bool {
        match *self {
            DeriveJunction::Soft(_) => true,
            _ => false
        }
    }

    pub fn is_hard(&self) -> bool {
        match *self {
            DeriveJunction::Hard(_) => true,
            _ => false
        }
    }
}

impl TryInto<ChildNumber> for DeriveJunction {
    type Error = KeyError;

    fn try_into(self) -> Result<ChildNumber, Self::Error> {
        if let Ok(num) = match self {
            DeriveJunction::Soft(index) => ChildNumber::from_normal_idx(index),
            DeriveJunction::Hard(index) => ChildNumber::from_hardened_idx(index),
        } {
            Ok(num)
        } else {
            Err(KeyError::InvalidChildNumber)
        }
    }
}

