use super::{
    key_types, EcdsaRecoverableSigner as TraitEcdsaRecoverableSigner,
    EcdsaSigner as TraitEcdsaSigner, KeyError, KeyTypeId, Pair as TraitPair, Public as TraitPublic,
    TypedKey,
};

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{Error as Bip32Error, ExtendedPrivKey, ExtendedPubKey};
use secp256k1::{Message, RecoverableSignature, Secp256k1, Signature};
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use crate::key::derive::*;
use lazy_static::lazy_static;

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

fn transform_secp256k1_error(err: secp256k1::Error) -> KeyError {
    match err {
        secp256k1::Error::IncorrectSignature => KeyError::InvalidSignature,
        secp256k1::Error::InvalidMessage => KeyError::InvalidMessage,
        secp256k1::Error::InvalidPublicKey => KeyError::InvalidPublicKey,
        secp256k1::Error::InvalidSignature => KeyError::InvalidSignature,
        secp256k1::Error::InvalidSecretKey => KeyError::InvalidPrivateKey,
        secp256k1::Error::InvalidRecoveryId => KeyError::InvalidRecoveryId,
        secp256k1::Error::InvalidTweak => KeyError::InvalidTweak,
    }
}

pub type Seed = Vec<u8>;

pub struct Public(pub ExtendedPubKey);

pub struct Pair(pub ExtendedPrivKey);

impl Derive for Public {
    type Error = KeyError;

    fn derive<Iter: Iterator<Item = DeriveJunction>>(
        &self,
        path: Iter,
    ) -> Result<Self, Self::Error> {
        let mut extended_key = self.0;

        for j in path {
            let child_number = j.try_into()?;

            match extended_key.ckd_pub(&SECP256K1_ENGINE, child_number) {
                Ok(r) => extended_key = r,
                Err(e) => {
                    return Err(transform_bip32_error(e));
                }
            }
        }

        Ok(Public(extended_key))
    }
}

impl Derive for Pair {
    type Error = KeyError;

    fn derive<T: Iterator<Item = DeriveJunction>>(&self, path: T) -> Result<Self, Self::Error> {
        let mut extended_key = self.0;

        for j in path {
            let child_number = j.try_into()?;

            match extended_key.ckd_priv(&SECP256K1_ENGINE, child_number) {
                Ok(r) => extended_key = r,
                Err(e) => {
                    return Err(transform_bip32_error(e));
                }
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
            Err(e) => Err(transform_bip32_error(e)),
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
        let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE, &self.0);

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

pub struct EcdsaSigner(Pair);

impl TraitEcdsaSigner for EcdsaSigner {
    type Error = KeyError;

    fn sign(&self, data: &[u8]) -> Result<Signature, Self::Error> {
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;

        Ok(SECP256K1_ENGINE.sign(&msg, &(self.0).0.private_key.key))
    }
}

pub struct EcdsaRecoverableSigner(Pair);

impl TraitEcdsaRecoverableSigner for EcdsaRecoverableSigner {
    type Error = KeyError;

    fn sign(&self, data: &[u8]) -> Result<RecoverableSignature, Self::Error> {
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;

        Ok(SECP256K1_ENGINE.sign_recoverable(&msg, &(self.0).0.private_key.key))
    }
}

#[cfg(test)]
mod tests {

    //TODO add more test
}
