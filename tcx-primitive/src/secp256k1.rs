use super::{key_types, KeyError, KeyTypeId, Pair as TraitPair, Public as TraitPublic, TypedKey};

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{
    ChainCode, ChildNumber, Error as Bip32Error, ExtendedPrivKey, ExtendedPubKey, Fingerprint,
};

use secp256k1::{Message, SecretKey};
use std::convert::TryInto;

use std::str::FromStr;

use crate::derive::*;
use crate::KeyError::{CannotDeriveKey, InvalidBase58, InvalidPublicKey};
use crate::{Result, Ss58Codec};
use bitcoin::util::base58;
use bitcoin::util::base58::Error::InvalidLength;
use bitcoin::PublicKey;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use core::fmt;
use lazy_static::lazy_static;

use bip39::Seed;
use bitcoin::consensus::encode;
use bitcoin::secp256k1::Secp256k1;
use std::fmt::Write;
use tcx_constants::{coin_from_xpub_prefix, network_from_coin, pub_version_from_prv_version};

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
        secp256k1::Error::NotEnoughMemory => KeyError::NotEnoughMemory,
    }
}

enum PublicType {
    ExtendedPubKey(ExtendedPubKey),
    PublicKey(PublicKey),
}

enum PrivateType {
    ExtendedPrivKey(ExtendedPrivKey),
    PrivateKey(PrivateKey),
}

#[derive(Clone)]
pub struct PrivateKey {
    //    pub network: u8,
    //    pub network: [u8; 4],
    pub compressed: bool,
    pub key: SecretKey,
}

//impl PrivateKey {
//    pub fn from_slice(slice: &[u8]) -> Result<Self> {
//        bitcoin::PrivateKey::from_wif()
//    }
//}

//impl ArbitraryNetworkPrivateKey {
//
//    pub fn public_key<C: secp256k1::Signing>(&self, secp: &Secp256k1<C>) -> PublicKey {
//        self.private_key.public_key(secp)
//    }
//
//    /// Serialize the private key to bytes
//    pub fn to_bytes(&self) -> Vec<u8> {
//        self.private_key.to_bytes()
//    }
//
//    /// Format the private key to WIF format.
//    pub fn fmt_wif(&self, fmt: &mut fmt::Write) -> fmt::Result {
//        let mut ret = [0; 34];
//        ret[0] = self.network;
//        ret[1..33].copy_from_slice(&self.private_key.key[..]);
//        let privkey = if self.private_key.compressed {
//            ret[33] = 1;
//            base58::check_encode_slice(&ret[..])
//        } else {
//            base58::check_encode_slice(&ret[..33])
//        };
//        fmt.write_str(&privkey)
//    }
//
//    /// Get WIF encoding of this private key.
//    pub fn to_wif(&self) -> String {
//        let mut buf = String::new();
//        buf.write_fmt(format_args!("{}", self)).unwrap();
//        buf.shrink_to_fit();
//        buf
//    }
//
//    /// Parse WIF encoded private key.
//    pub fn from_wif(wif: &str) -> Result<ArbitraryNetworkPrivateKey> {
//        let data = base58::from_check(wif)?;
//
//        let compressed = match data.len() {
//            33 => false,
//            34 => true,
//            _ => { return Err(KeyError::InvalidPrivateKey.into()); }
//        };
//
////        let network = match data[0] {
////            128 => Network::Bitcoin,
////            239 => Network::Testnet,
////            x   => { return Err(KeyError::InvalidPrivateKey.into()); }
////        };
//
//        let private_key = PrivateKey {
//            compressed: compressed,
//            network: Network::Bitcoin,
//            key: secp256k1::SecretKey::from_slice(&data[1..33])?,
//        };
//        Ok(ArbitraryNetworkPrivateKey{
//            network: data[0],
//            private_key
//        } )
//    }
//}
//

//
//impl fmt::Display for ArbitraryNetworkPrivateKey {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        self.fmt_wif(f)
//    }
//}
//
//impl fmt::Debug for ArbitraryNetworkPrivateKey {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "[private key data]")
//    }
//}

//impl FromStr for ArbitraryNetworkPrivateKey {
//    type Err = failure::Error;
//    fn from_str(s: &str) -> Result<ArbitraryNetworkPrivateKey> {
//        ArbitraryNetworkPrivateKey::from_wif(s)
//    }
//}

//#[derive(Clone, Debug)]
//pub struct ArbitraryNetworkExtendedPubKey {
////    pub coin: Option<String>,
//        pub network: [u8; 4],
//    pub extended_pub_key: ExtendedPubKey,
//}
//
//impl ArbitraryNetworkExtendedPubKey {
//    pub fn derive(&self, child_path: &str) -> Result<ArbitraryNetworkExtendedPubKey> {
//        let child_nums = relative_path_to_child_nums(child_path)?;
//        let index_ext_pub_key = self
//            .extended_pub_key
//            .derive_pub(&SECP256K1_ENGINE, &child_nums)?;
//        Ok(ArbitraryNetworkExtendedPubKey {
//            network: self.network,
//            extended_pub_key: index_ext_pub_key,
//        })
//    }
//
//    pub fn public_key(&self) -> bitcoin::PublicKey {
//        self.extended_pub_key.public_key
//    }
//
//
//}
//
//#[derive(Clone, Debug)]
//pub struct ArbitraryNetworkExtendedPrivKey {
//    pub network: [u8; 4],
//    pub extended_priv_key: ExtendedPrivKey,
//}
//
//impl ArbitraryNetworkExtendedPrivKey {
//    pub fn private_key(&self) -> PrivateKey {
//        PrivateKey {
//            compressed: self.extended_priv_key.private_key.compressed,
//            key: self.extended_priv_key.private_key.clone().key
//        }
//    }
//}
//
//impl fmt::Display for ArbitraryNetworkExtendedPubKey {
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
////        if let Some(coin) = &self.coin {
////            if let Some(network) = network_from_coin(coin) {
//                let mut ret = [0; 78];
//                ret[0..4].copy_from_slice(&self.network[..]);
//                ret[4] = self.extended_pub_key.depth as u8;
//                ret[5..9].copy_from_slice(&self.extended_pub_key.parent_fingerprint[..]);
//
//                BigEndian::write_u32(
//                    &mut ret[9..13],
//                    u32::from(self.extended_pub_key.child_number),
//                );
//
//                ret[13..45].copy_from_slice(&self.extended_pub_key.chain_code[..]);
//                ret[45..78].copy_from_slice(&self.extended_pub_key.public_key.key.serialize()[..]);
//                return write!(fmt, "{}", &base58::check_encode_slice(&ret[..]));
////            }
////        }
////        write!(fmt, "{}", "invalid_coin")?;
////        Err(fmt::Error)
//    }
//}
//
//impl FromStr for ArbitraryNetworkExtendedPubKey {
//    type Err = failure::Error;
//
//    fn from_str(inp: &str) -> Result<ArbitraryNetworkExtendedPubKey> {
//        let data = base58::from_check(inp)?;
//
//        if data.len() != 78 {
//            return Err(KeyError::InvalidBase58.into());
//        }
//        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
//        let child_number: ChildNumber = ChildNumber::from(cn_int);
//
//        let epk = ExtendedPubKey {
//            network: Network::Bitcoin,
//            depth: data[4],
//            parent_fingerprint: Fingerprint::from(&data[5..9]),
//            child_number,
//            chain_code: ChainCode::from(&data[13..45]),
//            public_key: PublicKey::from_slice(&data[45..78])
//                .map_err(|e| base58::Error::Other(e.to_string()))?,
//        };
////        let coin = coin_from_xpub_prefix(&data[0..4]);
//        let mut network = [0; 4];
//        network.copy_from_slice(&data[0..4]);
//        Ok(ArbitraryNetworkExtendedPubKey {
//            network,
//            extended_pub_key: epk,
//        })
//    }
//}
//
//impl fmt::Display for ArbitraryNetworkExtendedPrivKey {
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
////        if let Some(coin) = &self.coin {
////            if let Some(network) = network_from_coin(coin) {
//                let mut ret = [0; 78];
//
//                ret[0..4].copy_from_slice(&self.network[..]);
//                ret[4] = self.extended_priv_key.depth as u8;
//                ret[5..9].copy_from_slice(&self.extended_priv_key.parent_fingerprint[..]);
//
//                BigEndian::write_u32(
//                    &mut ret[9..13],
//                    u32::from(self.extended_priv_key.child_number),
//                );
//
//                ret[13..45].copy_from_slice(&self.extended_priv_key.chain_code[..]);
//                ret[45] = 0;
//                ret[46..78].copy_from_slice(&self.extended_priv_key.private_key[..]);
//                return write!(fmt, "{}", &base58::check_encode_slice(&ret[..]));
////            }
////        }
////        write!(fmt, "{}", "invalid_network")?;
////        Err(fmt::Error)
//    }
//}
//
//impl FromStr for ArbitraryNetworkExtendedPrivKey {
//    type Err = failure::Error;
//
//    fn from_str(inp: &str) -> Result<ArbitraryNetworkExtendedPrivKey> {
//        let data = base58::from_check(inp)?;
//
//        if data.len() != 78 {
//            return Err(InvalidLength(data.len()).into());
//        }
//
//        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
//        let child_number: ChildNumber = ChildNumber::from(cn_int);
//
//        let network = Network::Bitcoin;
//        let epk = ExtendedPrivKey {
//            network,
//            depth: data[4],
//            parent_fingerprint: Fingerprint::from(&data[5..9]),
//            child_number,
//            chain_code: ChainCode::from(&data[13..45]),
//            private_key: bitcoin::PrivateKey {
//                compressed: true,
//                network,
//                key: secp256k1::SecretKey::from_slice(&data[46..78])
//                    .map_err(|e| base58::Error::Other(e.to_string()))?,
//            },
//        };
////        let coin = coin_from_xprv_prefix(&data[0..4]);
//        let mut network = [0; 4];
//        network.copy_from_slice(&data[0..4]);
//        Ok(ArbitraryNetworkExtendedPrivKey {
//            network,
//            extended_priv_key: epk,
//        })
//    }
//
//
//}

pub struct Public(PublicType);

pub struct Pair(PrivateType);

impl Public {
    pub fn public_key(&self) -> PublicKey {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => r.public_key.clone(),
            PublicType::PublicKey(r) => r.clone(),
        }
    }

    pub fn to_compressed(&self) -> Vec<u8> {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => r.public_key.key.serialize().to_vec(),
            PublicType::PublicKey(r) => r.key.serialize().to_vec(),
        }
    }

    pub fn to_uncompressed(&self) -> Vec<u8> {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => r.public_key.key.serialize_uncompressed().to_vec(),
            PublicType::PublicKey(r) => r.key.serialize_uncompressed().to_vec(),
        }
    }
}

impl Pair {
    pub fn private_key(&self) -> PrivateKey {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let bitcoin_pk = r.private_key;
                PrivateKey {
                    compressed: bitcoin_pk.compressed,
                    key: bitcoin_pk.key,
                }
            }
            PrivateType::PrivateKey(r) => r.clone(),
        }
    }

    pub fn extended_pub_key(&self) -> Result<ExtendedPubKey> {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                //                let extended_pub_key =
                Ok(ExtendedPubKey::from_private(&SECP256K1_ENGINE, r))
                //
                //                Ok(ArbitraryNetworkExtendedPubKey {
                //                    network: pub_version_from_prv_version(&r.network).expect("find pub version from prv"),
                //                    extended_pub_key,
                //                })
            }
            _ => Err(CannotDeriveKey.into()),
        }
    }

    pub fn extended_priv_key(&self) -> Result<ExtendedPrivKey> {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                //                let extended_priv_key = r.extended_priv_key;
                //                Ok(ArbitraryNetworkExtendedPrivKey {
                //                    network: r.network,
                //                    extended_priv_key,
                //                })
                Ok(r.clone())
            }
            _ => Err(CannotDeriveKey.into()),
        }
    }

    pub fn from_wif(wif: &str) -> Result<Self> {
        let pk = PrivateKey::from_ss58check_with_version(wif)?.0;
        Ok(Pair(PrivateType::PrivateKey(pk)))
    }

    pub fn from_extended(xprv: &str) -> Result<Self> {
        //        let mut ext_priv_key = ArbitraryNetworkExtendedPrivKey::from_str(xprv)?;
        let (ext_priv_key, version) = ExtendedPrivKey::from_ss58check_with_version(xprv)?;
        Ok(Pair(PrivateType::ExtendedPrivKey(ext_priv_key)))
    }
}

impl Derive for Public {
    type Error = failure::Error;

    fn derive<Iter: Iterator<Item = DeriveJunction>>(
        &self,
        path: Iter,
    ) -> core::result::Result<Self, Self::Error> {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => {
                let mut extended_key = r.clone();

                for j in path {
                    let child_number = j.try_into()?;

                    extended_key = extended_key
                        .ckd_pub(&SECP256K1_ENGINE, child_number)
                        .map_err(transform_bip32_error)?;
                    //                    {
                    //                        Ok(r) => {
                    //                            extended_key = ArbitraryNetworkExtendedPubKey {
                    //                                network: network,
                    //                                extended_pub_key: r,
                    //                            }
                    //                        }
                    //                        Err(e) => {
                    //                            return Err(transform_bip32_error(e).into());
                    //                        }
                    //                    }
                }

                Ok(Public(PublicType::ExtendedPubKey(extended_key)))
            }
            _ => Err(KeyError::CannotDeriveKey.into()),
        }
    }
}

impl Derive for Pair {
    type Error = failure::Error;

    fn derive<T: Iterator<Item = DeriveJunction>>(
        &self,
        path: T,
    ) -> core::result::Result<Self, Self::Error> {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let mut extended_key = r.clone();

                for j in path {
                    let child_number = j.try_into()?;

                    extended_key = extended_key
                        .ckd_priv(&SECP256K1_ENGINE, child_number)
                        .map_err(transform_bip32_error)?;
                }

                Ok(Pair(PrivateType::ExtendedPrivKey(extended_key)))
            }
            _ => Err(KeyError::CannotDeriveKey.into()),
        }
    }
}

//impl FromStr for Pair {
//    type Err = failure::Error;
//
//    fn from_str(s: &str) -> Result<Self> {

//        match ArbitraryNetworkExtendedPrivKey::from_str(s) {
//            Ok(r) => Ok(Pair(PrivateType::ExtendedPrivKey(r))),
//            Err(_e) => Err(KeyError::InvalidBase58.into()),
//        }
//    }
//}

impl Public {
    pub fn from_extended(extended: &str) -> Result<Self> {
        let (epk, version) = ExtendedPubKey::from_ss58check_with_version(extended)?;
        Ok(Public(PublicType::ExtendedPubKey(epk)))
    }
}

impl Pair {
    /// Construct a new master key from a seed value
    pub fn new_pair(seed: &[u8]) -> Result<Pair> {
        let epk =
            ExtendedPrivKey::new_master(Network::Bitcoin, seed).map_err(transform_bip32_error)?;
        Ok(Pair(PrivateType::ExtendedPrivKey(epk)))
    }
}

impl TraitPair for Pair {
    type Public = Public;

    fn from_slice(data: &[u8]) -> Result<Self> {
        let pk = PrivateKey {
            compressed: true,
            key: secp256k1::SecretKey::from_slice(data).map_err(transform_secp256k1_error)?,
        };

        Ok(Pair(PrivateType::PrivateKey(pk)))
    }

    fn from_seed(seed: &Seed) -> Result<Pair> {
        Self::from_seed_slice(&seed.as_bytes())
    }

    fn from_seed_slice(seed: &[u8]) -> Result<Pair> {
        Self::new_pair(seed)
    }

    fn extended_public_key(&self) -> Result<Public> {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE, &r);

                Ok(Public(PublicType::ExtendedPubKey(pub_key)))
            }
            PrivateType::PrivateKey(_) => Err(CannotDeriveKey.into()),
        }
    }

    fn public_key(&self) -> Self::Public {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE, &r);
                Public(PublicType::PublicKey(pub_key.public_key))
            }
            PrivateType::PrivateKey(r) => {
                //                r.p
                let pub_key = PublicKey {
                    compressed: r.compressed,
                    key: secp256k1::PublicKey::from_secret_key(&SECP256K1_ENGINE, &r.key),
                };
                //                let pub_key = PublicKey::from_private_key(&SECP256K1_ENGINE, &r.key);
                Public(PublicType::PublicKey(pub_key))
            }
        }
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let pk = match &self.0 {
            PrivateType::ExtendedPrivKey(epk) => &epk.private_key.key,
            PrivateType::PrivateKey(prv) => &prv.key,
        };
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign(&msg, pk);
        Ok(signature.serialize_der().to_vec())
    }

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        let pk = match &self.0 {
            PrivateType::ExtendedPrivKey(epk) => &epk.private_key.key,
            PrivateType::PrivateKey(prv) => &prv.key,
        };
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign_recoverable(&msg, pk);
        let (recover_id, sign) = signature.serialize_compact();
        let signed_bytes = [sign[..].to_vec(), vec![(recover_id.to_i32()) as u8]].concat();
        Ok(signed_bytes)
    }

    fn is_extendable(&self) -> bool {
        match self.0 {
            PrivateType::ExtendedPrivKey(_) => true,
            PrivateType::PrivateKey(_) => false,
        }
    }
}

impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match &self.0 {
            PublicType::ExtendedPubKey(epk) => epk.fmt(f),
            PublicType::PublicKey(pub_key) => pub_key.fmt(f),
        }
    }
}

impl TraitPublic for Public {
    fn from_slice(_data: &[u8]) -> core::result::Result<Self, Self::Error> {
        //TODO How to distinguish whether to import from XPub or import from PublicKey
        let pub_key = bitcoin::PublicKey::from_slice(_data)?;
        Ok(Public(PublicType::PublicKey(pub_key)))
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        //TODO How to distinguish whether to export to XPub or export to PublicKey
        match &self.0 {
            PublicType::PublicKey(pub_key) => Ok(pub_key.to_bytes()),
            // todo: throw error
            PublicType::ExtendedPubKey(_epk) => Err(InvalidPublicKey.into()),
        }
    }
}

//impl FromStr for Public {
//    type Err = KeyError;
//
//    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
//        //TODO How to distinguish whether to import from XPub or import from PublicKey
//        match ArbitraryNetworkExtendedPubKey::from_str(s) {
//            Ok(r) => Ok(Public(PublicType::ExtendedPubKey(r))),
//            Err(_e) => Err(KeyError::InvalidBase58),
//        }
//    }
//}
//
//impl AsRef<[u8]> for Pair {
//    fn as_ref(&self) -> &[u8] {
//        unimplemented!()
//    }
//}
//
//impl AsRef<[u8]> for Public {
//    fn as_ref(&self) -> &[u8] {
//        unimplemented!()
//    }
//}

impl TypedKey for Public {
    const KEY_TYPE: KeyTypeId = key_types::SECP256K1;
}

impl TypedKey for Pair {
    const KEY_TYPE: KeyTypeId = key_types::SECP256K1;
}
//
//impl Signer<Signature> for Pair {
//    type Error = failure::Error;
//
//    fn sign<T: AsRef<[u8]>>(&self, data: T) -> core::result::Result<Signature, Self::Error> {
//        let msg = Message::from_slice(data.as_ref()).map_err(transform_secp256k1_error)?;
//
//        Ok(SECP256K1_ENGINE.sign(&msg, &self.private_key().key))
//    }
//}
//
//impl Signer<RecoverableSignature> for Pair {
//    type Error = failure::Error;
//
//    fn sign<T: AsRef<[u8]>>(
//        &self,
//        data: T,
//    ) -> core::result::Result<RecoverableSignature, Self::Error> {
//        let msg = Message::from_slice(data.as_ref()).map_err(transform_secp256k1_error)?;
//
//        Ok(SECP256K1_ENGINE.sign_recoverable(&msg, &(self.private_key().key)))
//    }
//}
impl Ss58Codec for PrivateKey {
    fn from_ss58check_with_version(wif: &str) -> Result<(Self, Vec<u8>)> {
        let data = base58::from_check(wif)?;

        let compressed = match data.len() {
            33 => false,
            34 => true,
            _ => {
                return Err(KeyError::InvalidPrivateKey.into());
            }
        };

        //        let network = match data[0] {
        //            128 => Network::Bitcoin,
        //            239 => Network::Testnet,
        //            x   => { return Err(KeyError::InvalidPrivateKey.into()); }
        //        };
        let key = secp256k1::SecretKey::from_slice(&data[1..33])?;
        let pk = PrivateKey { compressed, key };
        Ok((pk, vec![data[0]]))
    }

    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
        let mut ret = [0; 34];
        ret[0..1].copy_from_slice(&version[0..]);
        ret[1..33].copy_from_slice(&self.key[..]);
        if self.compressed {
            ret[33] = 1;
            base58::check_encode_slice(&ret[..]).to_string()
        } else {
            base58::check_encode_slice(&ret[..33]).to_string()
        }
    }
}

impl Ss58Codec for ExtendedPrivKey {
    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)> {
        let data = base58::from_check(s)?;

        if data.len() != 78 {
            return Err(InvalidLength(data.len()).into());
        }

        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
        let child_number: ChildNumber = ChildNumber::from(cn_int);

        let network = Network::Bitcoin;
        let epk = ExtendedPrivKey {
            network,
            depth: data[4],
            parent_fingerprint: Fingerprint::from(&data[5..9]),
            child_number,
            chain_code: ChainCode::from(&data[13..45]),
            private_key: bitcoin::PrivateKey {
                compressed: true,
                network,
                key: secp256k1::SecretKey::from_slice(&data[46..78])
                    .map_err(|e| base58::Error::Other(e.to_string()))?,
            },
        };
        //        let coin = coin_from_xprv_prefix(&data[0..4]);
        let mut network = [0; 4];
        network.copy_from_slice(&data[0..4]);
        Ok((epk, network.to_vec()))
    }

    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
        let mut ret = [0; 78];

        ret[0..4].copy_from_slice(&version[..]);
        ret[4] = self.depth as u8;
        ret[5..9].copy_from_slice(&self.parent_fingerprint[..]);

        BigEndian::write_u32(&mut ret[9..13], u32::from(self.child_number));

        ret[13..45].copy_from_slice(&self.chain_code[..]);
        ret[45] = 0;
        ret[46..78].copy_from_slice(&self.private_key[..]);
        base58::check_encode_slice(&ret[..])
    }
}

impl Ss58Codec for ExtendedPubKey {
    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)> {
        let data = base58::from_check(s)?;

        if data.len() != 78 {
            return Err(KeyError::InvalidBase58.into());
        }
        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
        let child_number: ChildNumber = ChildNumber::from(cn_int);

        let epk = ExtendedPubKey {
            network: Network::Bitcoin,
            depth: data[4],
            parent_fingerprint: Fingerprint::from(&data[5..9]),
            child_number,
            chain_code: ChainCode::from(&data[13..45]),
            public_key: PublicKey::from_slice(&data[45..78])
                .map_err(|e| base58::Error::Other(e.to_string()))?,
        };
        //        let coin = coin_from_xpub_prefix(&data[0..4]);
        let mut network = [0; 4];
        network.copy_from_slice(&data[0..4]);
        Ok((epk, network.to_vec()))
    }

    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
        let mut ret = [0; 78];
        ret[0..4].copy_from_slice(&version[..]);
        ret[4] = self.depth as u8;
        ret[5..9].copy_from_slice(&self.parent_fingerprint[..]);

        BigEndian::write_u32(&mut ret[9..13], u32::from(self.child_number));

        ret[13..45].copy_from_slice(&self.chain_code[..]);
        ret[45..78].copy_from_slice(&self.public_key.key.serialize()[..]);
        base58::check_encode_slice(&ret[..])
    }
}

#[cfg(test)]
mod tests {
    use crate::derive::Derive;

    use crate::{DerivePath, Pair, Public};
    use crate::{Secp256k1Pair, Secp256k1PublicKey, Ss58Codec};
    use bip39::{Language, Mnemonic, Seed};

    use bitcoin_hashes::hex::ToHex;
    use bitcoin_hashes::Hash;
    use std::str::FromStr;
    //    use crate::secp256k1::ArbitraryNetworkPrivateKey;
    use crate::secp256k1::PrivateKey;
    use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};

    #[test]
    fn test_secp256k1_prv_key() {
        let pair = Secp256k1Pair::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
            .unwrap();
        let _expected_pub_key_bytes = hex::decode("00").unwrap();
        let pub_key = pair.public_key().to_bytes().unwrap().to_hex();
        assert_eq!(
            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba",
            pub_key
        );
    }

    #[test]
    fn test_secp256k1_sign() {
        let prv_key =
            Secp256k1Pair::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
                .unwrap();
        let msg = "TokenCoreX";
        let hash = bitcoin_hashes::sha256::Hash::hash(msg.as_bytes());
        let signed_bytes = prv_key.sign(&hash.into_inner()).unwrap();
        assert_eq!("304402202514266dc7d807ecd69f6d5d03dae7d68619b2c562d8ac77f60e186f4fde4f2202207fbedf5642b095e4a37e71432c99e2b1144f8b9d73a0018be04e6d5ddbd26146", signed_bytes.to_hex());

        let wrong_signed = prv_key.sign(&[0, 1, 2, 3]);
        assert_eq!(
            format!("{}", wrong_signed.err().unwrap()),
            "invalid_message"
        )
    }

    fn default_seed() -> Seed {
        let mn = Mnemonic::from_phrase(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            Language::English,
        )
        .unwrap();
        Seed::new(&mn, "")
    }

    #[test]
    fn test_key_at_paths_with_seed() {
        let seed = default_seed();
        let paths = vec![
            "m/44'/0'/0'/0/0",
            "m/44'/0'/0'/0/1",
            "m/44'/0'/0'/1/0",
            "m/44'/0'/0'/1/1",
        ];
        let pair = Secp256k1Pair::from_seed(&seed).unwrap();
        let pub_keys = paths
            .iter()
            .map(|path| {
                pair.derive(DerivePath::from_str(path).unwrap().into_iter())
                    .unwrap()
                    .public_key()
                    .to_compressed()
                    .to_hex()
            })
            .collect::<Vec<String>>();
        let expected_pub_keys = vec![
            "026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868",
            "024fb7df3961e08f01025e434ea19708a4317d2fe59775cddd38df6e8a2d30697d",
            "0352470ace48f25b01b9c341e3b0e033fc32a203fb7a81a0453f97d94eca819a35",
            "022f4c38f7bbaa00fc886db62f975b34201c2bfed146e98973caf03268941801db",
        ];
        assert_eq!(pub_keys, expected_pub_keys);
    }

    #[test]
    fn extended_key_test() {
        let seed = default_seed();
        let pair = Secp256k1Pair::from_seed(&seed).unwrap();

        let _xpub_key = pair.extended_pub_key().unwrap();
        let mut index_xpub_key = pair
            .derive(DerivePath::from_str("m/44'/0'/0'").unwrap().into_iter())
            .unwrap()
            .extended_pub_key()
            .unwrap();
        //        index_xpub_key.coin = Some("BITCOIN".to_owned());
        let xpub = index_xpub_key.to_string();
        assert_eq!(xpub, "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8");
        let pair = Secp256k1Pair::from_seed(&seed).unwrap();
        let mut xprv_key = pair
            .derive(DerivePath::from_str("m/44'/0'/0'").unwrap().into_iter())
            .unwrap()
            .extended_priv_key()
            .unwrap();
        //        let mut account_xprv_key = xprv_key
        //        xprv_key.coin = Some("BITCOIN".to_owned());
        let xprv = xprv_key.to_string();
        assert_eq!(xprv, "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ");
    }

    #[test]
    fn derive_pub_key_test() {
        let xpub = "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8";
        let xpub_key = Secp256k1PublicKey::from_extended(xpub).unwrap();

        let path = DerivePath::from_str("0/0").unwrap();
        let index_pub_key = xpub_key.derive(path.into_iter()).unwrap();

        assert_eq!(
            index_pub_key.public_key().to_bytes().to_hex(),
            "026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868"
        );

        let err = ExtendedPubKey::from_ss58check_with_version("invalid_xpub")
            .err()
            .unwrap();
        assert_eq!(format!("{}", err), "invalid base58 character 0x6c");
    }

    #[test]
    fn test_encode_with_network() {
        let xpub = "tpubDDDcs8o1LaKXKXaPTEVBUZJYTgNAte4xj24MtFCMsfrHku93ZZjy87CGyz93dcocR6x6JHdusHodD9EVcSQuDbmkAWznWZtvyqyMDqS6VK4";
        let (xpub_key, version) = ExtendedPubKey::from_ss58check_with_version(xpub).unwrap();
        //        xpub_key.coin = Some("BITCOIN".to_owned());
        let ret = xpub_key.to_ss58check_with_version(&[0x04, 0x88, 0xB2, 0x1E]);
        assert_eq!("xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8", ret);

        let xprv = "tprv8g8UWPRHxaNWXZN3uoaiNpyYyaDr2j5Dvcj1vxLxKcEF653k7xcN9wq9eT73wBM1HzE9hmWJbAPXvDvaMXqGWm81UcVpHnmATfH2JJrfhGg";
        let (xprv_key, version) = ExtendedPrivKey::from_ss58check_with_version(xprv).unwrap();
        //        xprv_key.network = main_network_xprv_version;
        //        xprv_key.coin = Some("BITCOIN".to_owned());
        let ret = xprv_key.to_ss58check_with_version(&[0x04, 0x88, 0xAD, 0xE4]);
        assert_eq!("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS", ret);
    }

    #[test]
    fn pair_private_key() {
        let pair = Secp256k1Pair::from_extended("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS").unwrap();
        assert!(pair.is_extendable());
        let wif = pair.private_key().to_ss58check_with_version(&[0x80]);
        assert_eq!("L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD", wif);

        let pair = Secp256k1Pair::from_wif("L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD")
            .unwrap();
        assert_eq!(
            "L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD",
            pair.private_key().to_ss58check_with_version(&[0x80])
        );
        assert!(!pair.is_extendable());
    }

    #[test]
    fn wif_with_version() {
        let (pk, version) = PrivateKey::from_ss58check_with_version(
            "T8XwS9GfbPi73xQtwyQWLF2qXxFCkEtfdHNkrVrjXJijx8qEkHj9",
        )
        .unwrap();
        assert_eq!(
            "T8XwS9GfbPi73xQtwyQWLF2qXxFCkEtfdHNkrVrjXJijx8qEkHj9",
            pk.to_ss58check_with_version(&version)
        );
        assert_eq!(
            "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
            pk.to_ss58check_with_version(&[0x80])
        )
    }

    #[test]
    fn ypub_test() {
        let (epk, version) = ExtendedPrivKey::from_ss58check_with_version("uprv91G7gZkzehuMVxDJTYE6tLivdF8e4rvzSu1LFfKw3b2Qx1Aj8vpoFnHdfUZ3hmi9jsvPifmZ24RTN2KhwB8BfMLTVqaBReibyaFFcTP1s9n").unwrap();
        assert_eq!("uprv91G7gZkzehuMVxDJTYE6tLivdF8e4rvzSu1LFfKw3b2Qx1Aj8vpoFnHdfUZ3hmi9jsvPifmZ24RTN2KhwB8BfMLTVqaBReibyaFFcTP1s9n", epk.to_ss58check_with_version(&version));
    }
}
