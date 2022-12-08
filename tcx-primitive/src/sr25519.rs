use crate::ecc::{KeyError, PrivateKey as TraitPrivateKey, PublicKey as TraitPublicKey};
use crate::{FromHex, Result, ToHex};
use schnorrkel::SecretKey;
use std::convert::TryFrom;

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
        // let mini_key: MiniSecretKey =
        //     MiniSecretKey::from_bytes(data).expect("32 bytes can always build a key; qed");
        //
        // let kp = mini_key.expand_to_keypair(ExpansionMode::Ed25519);
        let pk = SecretKey::from_ed25519_bytes(data).map_err(|_| KeyError::InvalidSr25519Key)?;
        Ok(Sr25519PrivateKey(Pair::from(pk)))
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
        Ok(Sr25519PublicKey(
            Public::try_from(data).expect("gen sr25519 public key error"),
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl ToHex for Sr25519PublicKey {
    fn to_hex(&self) -> String {
        hex::encode(self.0 .0)
    }
}

impl FromHex for Sr25519PublicKey {
    fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)?;
        let pk = Sr25519PublicKey::from_slice(bytes.as_slice())?;
        Ok(pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_key_from_slice() {
        let pk_bytes: Vec<u8> =
            hex::decode("00ea01b0116da6ca425c477521fd49cc763988ac403ab560f4022936a18a4341016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd")
                .unwrap();
        let pk: Sr25519PrivateKey = Sr25519PrivateKey::from_slice(&pk_bytes).unwrap();
        assert_eq!(
            &hex::encode(pk.to_bytes())[64..],
            "016e7df1f5020068c9b150e0722fea65a264d5fbb342d4af4ddf2f1cdbddf1fd"
        );
        let public_key: Sr25519PublicKey = pk.public_key();
        assert_eq!(
            "fc581c897af481b10cf846d88754f1d115e486e5b7bcc39c0588c01b0a9b7a11",
            public_key.to_hex()
        );
        assert_eq!(
            "5Hma6gDS9yY7gPTuAFvmMDNcxPf9JqMZdPsaihfXiyw5NRnQ",
            format!("{}", public_key)
        );
    }

    #[test]
    fn test_sign_sr25519() {
        // use integration test
    }
}
