use crate::ecc::{PrivateKey as TraitPrivateKey, PublicKey as TraitPublicKey};
use crate::{FromHex, Result, ToHex};
use schnorrkel::{ExpansionMode, MiniSecretKey};

use sp_core::sr25519::{Pair, Public};
use sp_core::{Pair as TraitPair, Public as TraitPublic};

//use sp_core::crypto::Ss58Codec;

#[derive(Clone)]
pub struct Sr25519PublicKey(pub Public);

#[derive(Clone)]
pub struct Sr25519PrivateKey(pub Pair);

impl From<Public> for Sr25519PublicKey {
    fn from(pk: Public) -> Self {
        Sr25519PublicKey(pk)
    }
}

impl From<Pair> for Sr25519PrivateKey {
    fn from(sk: Pair) -> Self {
        Sr25519PrivateKey(sk)
    }
}

impl TraitPrivateKey for Sr25519PrivateKey {
    type PublicKey = Sr25519PublicKey;

    fn from_slice(data: &[u8]) -> Result<Self> {
        let mini_key: MiniSecretKey =
            MiniSecretKey::from_bytes(data).expect("32 bytes can always build a key; qed");

        let kp = mini_key.expand_to_keypair(ExpansionMode::Ed25519);
        Ok(Sr25519PrivateKey(Pair::from(kp)))
    }

    fn public_key(&self) -> Self::PublicKey {
        Sr25519PublicKey(self.0.public())
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(data).0.to_vec())
    }

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        // https://www.deadalnix.me/2017/02/17/schnorr-signatures-for-not-so-dummies/
        self.sign(data)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_raw_vec()
    }
}

impl std::fmt::Display for Sr25519PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TraitPublicKey for Sr25519PublicKey {
    fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(Sr25519PublicKey(Public::from_slice(data)))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl ToHex for Sr25519PublicKey {
    fn to_hex(&self) -> String {
        hex::encode(self.0.to_raw_vec())
    }
}

impl FromHex for Sr25519PublicKey {
    fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)?;
        let pk = Sr25519PublicKey::from_slice(bytes.as_slice())?;
        Ok(pk)
    }
}
