use secp256k1::{Secp256k1, Message, SecretKey};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address as BtcAddress;
use bitcoin::PrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language, Seed};
use std::str::FromStr;
use crate::Result;

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
    fn verify_schnorr(signature: &[u8], msg: &[u8]) ->bool {
        unimplemented!();
    }

    fn key_at_path(&self, path: &str, seed: &Seed) -> Result<Vec<u8>>;
    fn public_key(prv_key: &[u8]) -> String {
        unimplemented!();
    }
    fn compressed_public_key(prv_key: &[u8]) -> String {
        unimplemented!();
    }
    fn extended_prv_key(&self, path: &str, seed: &Seed) -> Result<String>;
    fn extended_pub_key(&self, path: &str, seed: &Seed) -> Result<String>;

}

pub struct Secp256k1Curve {

}



impl Secp256k1Curve {
    pub fn new() -> Secp256k1Curve {
        Secp256k1Curve{}
    }

    fn _extended_pri_key(&self, path: &str, seed: &Seed) -> Result<ExtendedPrivKey> {
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

    // todo: network
    fn key_at_path(&self, path: &str, seed: &Seed) -> Result<Vec<u8>> {
        let s = Secp256k1::new();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
        let path = DerivationPath::from_str(path)?;
        let main_address_pk = sk.derive_priv(&s, &path)?;
        Ok(main_address_pk.private_key.to_bytes())
    }

    fn extended_prv_key(&self, path: &str, seed: &Seed) -> Result<String> {
        let xprv = self._extended_pri_key(path, seed)?;
        Ok(xprv.to_string())
    }

    fn extended_pub_key(&self, path: &str, seed: &Seed) -> Result<String> {
        let s = Secp256k1::new();
        let xprv = self._extended_pri_key(path, seed)?;
        let xpub = ExtendedPubKey::from_private(&s, &xprv);
        Ok(xpub.to_string())
    }

    
}

