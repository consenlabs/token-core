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
    SECP256k1 = 0,
    SECP256k1Extended = 1,
    NIST256p1 = 2,
    NIST256p1Extended = 3,
    ED25519 = 4,
    ED25519Blake2b = 5,
    CURVE25519 = 6,
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

//    fn public_key(&self) -> Result<PublicKey> {
//        match &self.pub_key_type {
//            PublicKeyType::SECP256k1 => {
//                let key = SecretKey::from_slice(&self.bytes)?;
//                let k1_pub_key = Secp256k1PubKey::from_secret_key(secp, &self.key);
//                Ok(PublicKey {
//                    compressed: true,
//                    bytes: k1_pub_key.serialize().to_vec(),
//                    pub_key_type: PublicKeyType::SECP256k1,
//                })
//            }
//            PublicKeyType::SECP256k1Extended => {
//                let key = SecretKey::from_slice(&self.bytes)?;
//                let k1_pub_key = Secp256k1PubKey::from_secret_key(secp, &self.key);
//                Ok(PublicKey {
//                    compressed: false,
//                    bytes: k1_pub_key.serialize_uncompressed().to_vec(),
//                    pub_key_type: PublicKeyType::SECP256k1Extended,
//                })
//            }
//            _ => Err(format_err!("{}", "unsupport_curve"))
//        }
//    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let msg = Message::from_slice(data)?;
//        let key = SecretKey::from_slice(&self.bytes)?;
        let signature = s.sign(&msg, &self.prv_key.key);
        Ok(signature.serialize_der().to_vec())
    }
}

//pub struct Secp256k1PublicKey {
//    pub compressed: bool,
//    pub bytes: Vec<u8>,
//    pub pub_key_type: PublicKeyType,
//}

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
//
//impl PrivateKey for bitcoin::PrivateKey {
//    type PublicKey = bitcoin::PublicKey;
//
//    fn public_key(&self) -> Self::PublicKey {
//        self.public_key(&secp256k1::Secp256k1::new())
//    }
//}

// todo: try to move Curve to crypto
pub trait Curve {
    fn sign(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn sign_canonical(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn sign_der(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn sign_schnorr(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn verify(signature: &[u8], msg: &[u8]) -> bool {
        unimplemented!();
    }
    fn verify_schnorr(signature: &[u8], msg: &[u8]) -> bool {
        unimplemented!();
    }

    fn key_at_path(path: &str, seed: &Seed) -> Result<Vec<u8>>;
    fn public_key(prv_key: &[u8]) -> Result<Vec<u8>> {
        unimplemented!();
    }
    fn compressed_public_key(prv_key: &[u8]) -> String {
        unimplemented!();
    }
    fn extended_prv_key(path: &str, seed: &Seed) -> Result<DerivationInfo>;
    fn extended_pub_key(path: &str, seed: &Seed) -> Result<DerivationInfo>;
}

pub struct Secp256k1Curve {}


impl Secp256k1Curve {
    pub fn new() -> Secp256k1Curve {
        Secp256k1Curve {}
    }

    fn _extended_pri_key(path: &str, seed: &Seed) -> Result<ExtendedPrivKey> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let path = DerivationPath::from_str(path)?;
        Ok(sk.derive_priv(&s, &path)?)
    }

    fn sign(pk: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let msg = Message::from_slice(bytes)?;
        let key = SecretKey::from_slice(pk)?;
        let signature = s.sign(&msg, &key);
        Ok(signature.serialize_compact().to_vec())
    }


    fn key_at_path(path: &str, seed: &Seed) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let path = DerivationPath::from_str(path)?;
        let key_at_path = sk.derive_priv(&s, &path)?;
        Ok(key_at_path.private_key.to_bytes())
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

    fn public_key(prv_key: &[u8]) -> Result<Vec<u8>> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(prv_key)?;
        let pub_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        Ok(pub_key.serialize().to_vec())
    }
}

