use super::{KeyTypeId, key_types, TypedKey, Public as TraitPublic, Pair as TraitPair, KeyError};

use secp256k1::{Secp256k1, Message};
use std::fmt;
use std::str::FromStr;
use bitcoin::util::bip32::{ExtendedPubKey, ExtendedPrivKey, Error as Bip32Error};
use bitcoin::network::constants::Network;
use std::convert::TryInto;

use lazy_static::lazy_static;
use crate::key::derive::*;

lazy_static! {
    /// Lazily initialized secp256k1 engine
    static ref SECP256K1_ENGINE: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
}

fn transform_bip32_error(err: Bip32Error) -> KeyError {
    match err {
        Bip32Error::Ecdsa(_) => KeyError::InvalidEcdsa,
        Bip32Error::RngError(_) => KeyError::OverflowChildNumber,
        Bip32Error::CannotDeriveFromHardenedKey => KeyError::CannotDeriveFromHardenedKey,
        Bip32Error::InvalidChildNumber(_) => KeyError::InvalidChildNumber,
        Bip32Error::InvalidChildNumberFormat => KeyError::InvalidChildNumber,
        Bip32Error::InvalidDerivationPathFormat => KeyError::InvalidDerivationPathFormat,
    }
}

pub type Seed = Vec<u8>;

pub struct Public(ExtendedPubKey);

pub struct Pair(ExtendedPrivKey);

impl Derive for Public {
    type Error = KeyError;

    fn derive<Iter: Iterator<Item=DeriveJunction>>(&self, path: Iter) -> Result<Self, Self::Error> {
        let mut extended_key = self.0;

        for j in path {
            let child_number = j.try_into()?;

            match extended_key.ckd_pub(&SECP256K1_ENGINE, child_number) {
                Ok(r) => extended_key = r,
                Err(e) => {
                    return Err(transform_bip32_error(e));
                },
            }
        }

        Ok(Public(extended_key))
    }
}

impl Derive for Pair {
    type Error = KeyError;

    fn derive<T: Iterator<Item=DeriveJunction>>(&self, path: T) -> Result<Self, Self::Error> {
        let mut extended_key= self.0;

        for j in path {
            let child_number = j.try_into()?;

            match extended_key.ckd_priv(&SECP256K1_ENGINE, child_number) {
                Ok(r) => extended_key = r,
                Err(e) => {
                    return Err(transform_bip32_error(e));
                },
            }
        }

        Ok(Pair(extended_key))
    }
}

impl FromStr for Pair {
    type Err = KeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ExtendedPrivKey::from_str(s) {
            Ok(r) => Ok(Pair(r)),
            Err(e) => Err(KeyError::InvalidBase58),
        }
    }
}

impl Pair {
    /// Construct a new master key from a seed value
    pub fn new_pair(network: Network, seed: &[u8]) -> Result<Pair, KeyError> {
        match ExtendedPrivKey::new_master(network, seed) {
            Ok(r) => Ok(Pair(r)),
            Err(e) => Err(transform_bip32_error(e))
        }
    }
}

impl TraitPair for Pair {
    type Public = Public;
    type Seed = Seed;
    type Error = KeyError;

    fn from_slice(_: &[u8]) -> Result<Self, KeyError> {
        unimplemented!()
    }

    fn from_seed(seed: &Seed) -> Result<Pair, KeyError> {
        Self::from_seed_slice(&seed[..])
    }

    fn from_seed_slice(seed: &[u8]) -> Result<Pair, KeyError> {
        Self::new_pair(Network::Bitcoin, seed)
    }

    fn public(&self) -> Public {
        let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE,&self.0);

        Public(pub_key)
    }
}

impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}

impl std::fmt::Debug for Public {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}

impl TraitPublic for Public {
    type Error = KeyError;

    fn from_slice(data: &[u8]) -> Result<Self, Self::Error> {
        //TODO from
        unimplemented!()
    }

    fn as_slice(&self) -> &[u8] {
        let r: &[u8] = self.as_ref();
        &r[..]
    }
}

impl FromStr for Public {
    type Err = KeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ExtendedPubKey::from_str(s) {
            Ok(r) => Ok(Public(r)),
            Err(e) => Err(KeyError::InvalidBase58),
        }
    }
}

impl AsRef<[u8]> for Pair {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl AsRef<[u8]> for Public {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl TypedKey for Public {
    const KEY_TYPE: KeyTypeId = key_types::SECP256K1;
}

impl TypedKey for Pair {
    const KEY_TYPE: KeyTypeId = key_types::SECP256K1;
}

#[cfg(test)]
mod tests {

    //TODO add more test
}


