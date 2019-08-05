use secp256k1::{Secp256k1, Message, SecretKey};
use bitcoin::network::constants::Network;
use bitcoin::PrivateKey as BtcPrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language, Seed};
use std::str::FromStr;
use crate::Result;
use crate::bips::DerivationInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurveType {
    SECP256k1              /* "secp256k1" */,
    ED25519                /* "ed25519" */,
    ED25519Blake2bNano     /* "ed25519-blake2b-nano" */,
    Curve25519             /* "curve25519" */,
    NIST256p1,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicKeyType {
    SECP256k1,
    SECP256k1Extended,
    NIST256p1,
    NIST256p1Extended,
    ED25519,
    ED25519Blake2b,
    CURVE25519,
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


pub struct Secp256k1PrivateKey {
    //    bytes: Vec<u8>,
//    curve: CurveType,
//    pub_key_type: PublicKeyType,
    pub prv_key: bitcoin::PrivateKey
}

impl PrivateKey for Secp256k1PrivateKey {
    type PublicKey = Secp256k1PubKey;

    fn public_key(&self) -> Self::PublicKey {
        Secp256k1PubKey {
            pub_key: self.prv_key.public_key(&secp256k1::Secp256k1::new())
        }
    }

    fn is_valid(data: &[u8]) -> bool {
        SecretKey::from_slice(data).is_ok()
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let msg = Message::from_slice(data)?;
        let signature = s.sign(&msg, &self.prv_key.key);
        Ok(signature.serialize_der().to_vec())
    }
}



pub struct Secp256k1PubKey {
    pub pub_key: bitcoin::PublicKey
}

impl PublicKey for Secp256k1PubKey {
    fn to_bytes(&self) -> Vec<u8> {
        self.pub_key.to_bytes()
    }

    fn to_compressed(&self) -> Vec<u8> {
        self.pub_key.key.serialize().to_vec()
    }

    fn to_uncompressed(&self) -> Vec<u8> {
        self.pub_key.key.serialize_uncompressed().to_vec()
    }

    fn from_slice(data: &[u8]) -> Result<Secp256k1PubKey> {
        if let Ok(key) = bitcoin::PublicKey::from_slice(data) {
            Ok(Secp256k1PubKey {
                pub_key: key
            })
        } else {
            Err(format_err!("{}", "invalid_public_key"))
        }
    }
}

pub struct Secp256k1Curve {}

impl Secp256k1Curve {

    fn _extended_pri_key(path: &str, seed: &Seed) -> Result<ExtendedPrivKey> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let path = DerivationPath::from_str(path)?;
        Ok(sk.derive_priv(&s, &path)?)
    }

    pub fn key_at_paths_with_seed(paths: &[impl AsRef<str>], seed: &Seed) -> Result<Vec<impl PrivateKey>> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let pks: Result<Vec<Secp256k1PrivateKey>> = paths.iter().map(|path| {
            let path = DerivationPath::from_str(path.as_ref())?;
            let prv_key = sk.derive_priv(&s, &path)?;
            Ok(Secp256k1PrivateKey {
                prv_key: prv_key.private_key
            })
        }).collect();
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

}

