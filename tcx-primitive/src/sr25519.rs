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
        //        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        //        let signature = SECP256K1_ENGINE.sign(&msg, &self.0.key);
        //        Ok(signature.serialize_der().to_vec())
    }

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        //
        //        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        //        let signature = SECP256K1_ENGINE.sign_recoverable(&msg, &self.0.key);
        //        let (recover_id, sign) = signature.serialize_compact();
        //        let signed_bytes = [sign[..].to_vec(), vec![(recover_id.to_i32()) as u8]].concat();
        //        Ok(signed_bytes)
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
//
//impl Ss58Codec for Sr25519PublicKey {
//    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)> {
//        // todo: address from
//        let (addr, ver) = Public::from_ss58check_with_version(s)
//            .map_err(|_| format_err!("parse address error"))?;
//        Ok((Sr25519PublicKey::from(addr), vec!(ver)))
//    }
//
//    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
////        sp_core::crypto::Ss58AddressFormat::frx;
//        // todo: unwrap()
//        let ver = Ss58AddressFormat::Custom(version.first().unwrap().clone());
//        self.0.to_ss58check_with_version(ver)
//    }
//}

//impl Ss58Codec for Sr25519PrivateKey {
//    fn from_ss58check_with_version(wif: &str) -> Result<(Self, Vec<u8>)> {
////        let data = base58::from_check(wif)?;
////
////        let compressed = match data.len() {
////            33 => false,
////            34 => true,
////            _ => {
////                return Err(KeyError::InvalidPrivateKey.into());
////            }
////        };
////
////        let pk = Secp256k1PrivateKey(PrivateKey {
////            key: secp256k1::SecretKey::from_slice(&data[1..33])?,
////            compressed,
////            network: Network::Bitcoin,
////        });
//
//        Ok((pk, vec![data[0]]))
//    }
//
//    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
//        let mut ret = [0; 34];
//        ret[0..1].copy_from_slice(&version[0..]);
//        ret[1..33].copy_from_slice(&self.0.key[..]);
//        if self.0.compressed {
//            ret[33] = 1;
//            base58::check_encode_slice(&ret[..]).to_string()
//        } else {
//            base58::check_encode_slice(&ret[..33]).to_string()
//        }
//    }
//}
//
