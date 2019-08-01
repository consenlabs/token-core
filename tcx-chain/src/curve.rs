use secp256k1::{Secp256k1, Message, SecretKey, PublicKey as Secp256k1PubKey};
use bitcoin::network::constants::Network;
use bitcoin::PrivateKey as BtcPrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language, Seed};
use std::str::FromStr;
use crate::Result;
use crate::bips::DerivationInfo;

#[derive(Debug, Copy)]
pub enum CurveType {
    SECP256k1              /* "secp256k1" */,
    ED25519                /* "ed25519" */,
    ED25519Blake2bNano     /* "ed25519-blake2b-nano" */,
    Curve25519             /* "curve25519" */,
    NIST256p1,
}

#[derive(Debug, Copy)]
pub enum PublicKeyType {
    SECP256k1 = 0,
    SECP256k1Extended = 1,
    NIST256p1 = 2,
    NIST256p1Extended = 3,
    ED25519 = 4,
    ED25519Blake2b = 5,
    CURVE25519 = 6,
}

pub struct PrivateKey {
    bytes: Vec<u8>,
    curve: CurveType,
    pub_key_type: PublicKeyType,
}

impl PrivateKey {
    pub fn is_valid(data: &[u8], curve: CurveType) -> bool {
        match curve {
            CurveType::SECP256k1 => {
                SecretKey::from_slice(data).is_ok()
            }
            _ => false
        }
    }

    pub fn public_key(&self) -> Result<PublicKey> {
        match &self.pub_key_type {
            PublicKeyType::SECP256k1 => {
                let key = SecretKey::from_slice(&self.bytes)?;
                let k1_pub_key = Secp256k1PubKey::from_secret_key(secp, &self.key);
                Ok(PublicKey {
                    compressed: true,
                    bytes: k1_pub_key.serialize().to_vec(),
                    pub_key_type: PublicKeyType::SECP256k1,
                })
            }
            PublicKeyType::SECP256k1Extended => {
                let key = SecretKey::from_slice(&self.bytes)?;
                let k1_pub_key = Secp256k1PubKey::from_secret_key(secp, &self.key);
                Ok(PublicKey {
                    compressed: false,
                    bytes: k1_pub_key.serialize_uncompressed().to_vec(),
                    pub_key_type: PublicKeyType::SECP256k1Extended,
                })
            }
            _ => Err(format_err!("{}", "unsupport_curve"))
        }
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        match &self.curve {
            CurveType::SECP256k1 => {
                let s = Secp256k1::new();
                let msg = Message::from_slice(data)?;
                let key = SecretKey::from_slice(&self.bytes)?;
                let signature = s.sign(&msg, &key);
                Ok(signature.serialize_compact().to_vec())
            }
            _ => {
                Err(format_err!("{}", "unsupport_curve"))
            }
        }
    }
}

pub struct PublicKey {
    pub compressed: bool,
    pub bytes: Vec<u8>,
    pub pub_key_type: PublicKeyType,
}

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
}

impl Curve for Secp256k1Curve {
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

    fn extended_prv_key(path: &str, seed: &Seed) -> Result<DerivationInfo> {
        let xprv = Self::_extended_pri_key(path, seed)?;

        Ok(DerivationInfo::from(xprv))
    }

    fn extended_pub_key(path: &str, seed: &Seed) -> Result<DerivationInfo> {
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

