use bitcoin::network::constants::Network;
use secp256k1::{Message, Secp256k1, SecretKey};

use crate::bips::DerivationInfo;
use crate::Result;
use bip39::Seed;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CurveType {
    SECP256k1,          /* "secp256k1" */
    ED25519,            /* "ed25519" */
    ED25519Blake2bNano, /* "ed25519-blake2b-nano" */
    Curve25519,         /* "curve25519" */
    NIST256p1,
}

pub trait PublicKey: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_compressed(&self) -> Vec<u8>;
    fn to_uncompressed(&self) -> Vec<u8>;

    fn from_slice(data: &[u8]) -> Result<Self>;
}

pub trait PrivateKey {
    type PublicKey: PublicKey;

    fn is_valid(data: &[u8]) -> bool;
    fn public_key(&self) -> Self::PublicKey;
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;
}

impl PublicKey for bitcoin::PublicKey {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    #[warn(unconditional_recursion)]
    fn to_compressed(&self) -> Vec<u8> {
        bitcoin::PublicKey::to_compressed(self)
    }

    #[warn(unconditional_recursion)]
    fn to_uncompressed(&self) -> Vec<u8> {
        bitcoin::PublicKey::to_uncompressed(&self)
    }

    fn from_slice(data: &[u8]) -> Result<bitcoin::PublicKey> {
        if let Ok(key) = bitcoin::PublicKey::from_slice(data) {
            Ok(key)
        } else {
            Err(format_err!("invalid_secp256k1_public_key"))
        }
    }
}

pub type Secp256k1PublicKey = bitcoin::PublicKey;

impl PrivateKey for bitcoin::PrivateKey {
    type PublicKey = bitcoin::PublicKey;

    fn is_valid(data: &[u8]) -> bool {
        SecretKey::from_slice(data).is_ok()
    }

    fn public_key(&self) -> Self::PublicKey {
        self.public_key(&secp256k1::Secp256k1::new())
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let msg = Message::from_slice(data)?;
        let signature = s.sign(&msg, &self.key);
        Ok(signature.serialize_der().to_vec())
    }
}

pub type Secp256k1PrivateKey = bitcoin::PrivateKey;

pub struct Secp256k1Curve {}

impl Secp256k1Curve {
    fn _extended_pri_key(path: &str, seed: &Seed) -> Result<ExtendedPrivKey> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let path = DerivationPath::from_str(path)?;
        Ok(sk.derive_priv(&s, &path)?)
    }

    pub fn key_at_paths_with_seed(
        paths: &[impl AsRef<str>],
        seed: &Seed,
    ) -> Result<Vec<impl PrivateKey>> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let pks: Result<Vec<Secp256k1PrivateKey>> = paths
            .iter()
            .map(|path| {
                let path = DerivationPath::from_str(path.as_ref())?;
                let prv_key = sk.derive_priv(&s, &path)?;
                Ok(prv_key.private_key)
            })
            .collect();
        pks
    }

    pub fn extended_prv_key(path: &str, seed: &Seed) -> Result<DerivationInfo> {
        let xprv = Self::_extended_pri_key(path, seed)?;

        Ok(DerivationInfo::from(xprv))
    }

    pub fn extended_pub_key(path: &str, seed: &Seed) -> Result<DerivationInfo> {
        let s = Secp256k1::new();
        let xprv = Self::_extended_pri_key(path, seed)?;
        let xpub = ExtendedPubKey::from_private(&s, &xprv);
        Ok(DerivationInfo::from(xpub))
    }

    pub fn derive_pub_key_at_path(xpub: &str, child_path: &str) -> Result<bitcoin::PublicKey> {
        let ext_pub_key = ExtendedPubKey::from_str(xpub)?;
        let s = Secp256k1::new();
        let child_nums = crate::bips::relative_path_to_child_nums(child_path)?;
        let index_ext_pub_key = ext_pub_key.derive_pub(&s, &child_nums)?;
        Ok(index_ext_pub_key.public_key)
    }
}
