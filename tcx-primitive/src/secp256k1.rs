use super::{
    key_types, KeyError, KeyTypeId, Pair as TraitPair, Public as TraitPublic, Signer, TypedKey,
};

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{
    ChainCode, ChildNumber, Error as Bip32Error, ExtendedPrivKey, ExtendedPubKey, Fingerprint,
};
use secp256k1::recovery::RecoverableSignature;
use secp256k1::{Message, Signature};
use std::convert::TryInto;

use std::str::FromStr;

use crate::derive::*;
use crate::KeyError::{CannotDeriveKey, InvalidMessage};
use crate::Result;
use bitcoin::util::base58;
use bitcoin::util::base58::Error::InvalidLength;
use bitcoin::{PrivateKey, PublicKey};
use byteorder::BigEndian;
use byteorder::ByteOrder;
use core::fmt;
use lazy_static::lazy_static;
use std::io::Cursor;

use bitcoin::util::psbt::serialize::Serialize;
use std::convert::AsMut;
//use tcx::curve::PublicKey;

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

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
    ExtendedPubKey(ArbitraryNetworkExtendedPubKey),
    PublicKey(PublicKey),
}

enum PrivateType {
    ExtendedPrivKey(ArbitraryNetworkExtendedPrivKey),
    PrivateKey(PrivateKey),
}

#[derive(Clone, Copy, Debug)]
pub struct ArbitraryNetworkExtendedPubKey {
    pub network: [u8; 4],
    pub extended_pub_key: ExtendedPubKey,
}

#[derive(Clone, Copy, Debug)]
pub struct ArbitraryNetworkExtendedPrivKey {
    pub network: [u8; 4],
    pub extended_priv_key: ExtendedPrivKey,
}

impl fmt::Display for ArbitraryNetworkExtendedPubKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = [0; 78];
        ret[0..4].copy_from_slice(&self.network[..]);
        ret[4] = self.extended_pub_key.depth as u8;
        ret[5..9].copy_from_slice(&self.extended_pub_key.parent_fingerprint[..]);

        BigEndian::write_u32(
            &mut ret[9..13],
            u32::from(self.extended_pub_key.child_number),
        );

        ret[13..45].copy_from_slice(&self.extended_pub_key.chain_code[..]);
        ret[45..78].copy_from_slice(&self.extended_pub_key.public_key.key.serialize()[..]);
        fmt.write_str(&base58::check_encode_slice(&ret[..]))
    }
}

impl FromStr for ArbitraryNetworkExtendedPubKey {
    type Err = failure::Error;

    fn from_str(inp: &str) -> Result<ArbitraryNetworkExtendedPubKey> {
        let data = base58::from_check(inp)?;

        if data.len() != 78 {
            return Err(KeyError::InvalidBase58.into());
        }
        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
        //        let cn_int: u32 = Cursor::new(&data[9..13]).read_u32::<BigEndian>().unwrap();
        let child_number: ChildNumber = ChildNumber::from(cn_int);

        let epk = ExtendedPubKey {
            network: Network::Bitcoin,
            depth: data[4],
            parent_fingerprint: Fingerprint::from(&data[5..9]),
            child_number: child_number,
            chain_code: ChainCode::from(&data[13..45]),
            public_key: PublicKey::from_slice(&data[45..78])
                .map_err(|e| base58::Error::Other(e.to_string()))?,
        };
        Ok(ArbitraryNetworkExtendedPubKey {
            network: clone_into_array(&data[0..4]),
            extended_pub_key: epk,
        })
    }
}

impl fmt::Display for ArbitraryNetworkExtendedPrivKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = [0; 78];
        ret[0..4].copy_from_slice(&self.network[..]);
        ret[4] = self.extended_priv_key.depth as u8;
        ret[5..9].copy_from_slice(&self.extended_priv_key.parent_fingerprint[..]);

        BigEndian::write_u32(
            &mut ret[9..13],
            u32::from(self.extended_priv_key.child_number),
        );

        ret[13..45].copy_from_slice(&self.extended_priv_key.chain_code[..]);
        ret[45] = 0;
        ret[46..78].copy_from_slice(&self.extended_priv_key.private_key[..]);
        fmt.write_str(&base58::check_encode_slice(&ret[..]))
    }
}

impl FromStr for ArbitraryNetworkExtendedPrivKey {
    type Err = failure::Error;

    fn from_str(inp: &str) -> Result<ArbitraryNetworkExtendedPrivKey> {
        let data = base58::from_check(inp)?;

        if data.len() != 78 {
            return Err(InvalidLength(data.len()).into());
        }

        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
        let child_number: ChildNumber = ChildNumber::from(cn_int);

        let network = Network::Bitcoin;
        let epk = ExtendedPrivKey {
            network: network,
            depth: data[4],
            parent_fingerprint: Fingerprint::from(&data[5..9]),
            child_number: child_number,
            chain_code: ChainCode::from(&data[13..45]),
            private_key: PrivateKey {
                compressed: true,
                network: network,
                key: secp256k1::SecretKey::from_slice(&data[46..78])
                    .map_err(|e| base58::Error::Other(e.to_string()))?,
            },
        };
        Ok(ArbitraryNetworkExtendedPrivKey {
            network: clone_into_array(&data[0..4]),
            extended_priv_key: epk,
        })
    }
}

pub type Seed = Vec<u8>;

pub struct Public(PublicType);

pub struct Pair(PrivateType);

impl Public {
    pub fn public_key(&self) -> &PublicKey {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => &r.extended_pub_key.public_key,
            PublicType::PublicKey(r) => &r,
        }
    }

    pub fn to_compressed(&self) -> Vec<u8> {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => r.extended_pub_key.public_key.key.serialize().to_vec(),
            PublicType::PublicKey(r) => r.key.serialize().to_vec(),
        }
    }

    pub fn to_uncompressed(&self) -> Vec<u8> {
        match &self.0 {
            PublicType::ExtendedPubKey(r) => r
                .extended_pub_key
                .public_key
                .key
                .serialize_uncompressed()
                .to_vec(),
            PublicType::PublicKey(r) => r.key.serialize_uncompressed().to_vec(),
        }
    }
}

impl Pair {
    pub fn private_key(&self) -> &PrivateKey {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => &r.extended_priv_key.private_key,
            PrivateType::PrivateKey(r) => &r,
        }
    }

    pub fn extended_pub_key(&self) -> Result<ArbitraryNetworkExtendedPubKey> {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let extended_pub_key =
                    ExtendedPubKey::from_private(&SECP256K1_ENGINE, &r.extended_priv_key);
                Ok(ArbitraryNetworkExtendedPubKey {
                    network: r.network,
                    extended_pub_key,
                })
            }
            _ => Err(CannotDeriveKey.into()),
        }
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
                let mut extended_key: ArbitraryNetworkExtendedPubKey = r.clone();

                for j in path {
                    let child_number = j.try_into()?;

                    match extended_key
                        .extended_pub_key
                        .ckd_pub(&SECP256K1_ENGINE, child_number)
                    {
                        Ok(r) => {
                            extended_key = ArbitraryNetworkExtendedPubKey {
                                network: extended_key.network,
                                extended_pub_key: r,
                            }
                        }
                        Err(e) => {
                            return Err(transform_bip32_error(e).into());
                        }
                    }
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
        match self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let mut extended_key = r;

                for j in path {
                    let child_number = j.try_into()?;

                    match extended_key
                        .extended_priv_key
                        .ckd_priv(&SECP256K1_ENGINE, child_number)
                    {
                        Ok(r) => {
                            extended_key = ArbitraryNetworkExtendedPrivKey {
                                network: extended_key.network,
                                extended_priv_key: r,
                            }
                        }
                        Err(e) => {
                            return Err(transform_bip32_error(e).into());
                        }
                    }
                }

                Ok(Pair(PrivateType::ExtendedPrivKey(extended_key)))
            }
            _ => Err(KeyError::CannotDeriveKey.into()),
        }
    }
}

impl FromStr for Pair {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self> {
        match ArbitraryNetworkExtendedPrivKey::from_str(s) {
            Ok(r) => Ok(Pair(PrivateType::ExtendedPrivKey(r))),
            Err(_e) => Err(KeyError::InvalidBase58.into()),
        }
    }
}

impl Pair {
    /// Construct a new master key from a seed value
    pub fn new_pair(seed: &[u8]) -> Result<Pair> {
        match ExtendedPrivKey::new_master(Network::Bitcoin, seed) {
            Ok(r) => Ok(Pair(PrivateType::ExtendedPrivKey(
                ArbitraryNetworkExtendedPrivKey {
                    network: [0, 0, 0, 0],
                    extended_priv_key: r,
                },
            ))),
            Err(e) => Err(transform_bip32_error(e).into()),
        }
    }
}

impl TraitPair for Pair {
    type Public = Public;
    type Seed = Seed;

    fn from_slice(data: &[u8]) -> Result<Self> {
        let private_key = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: secp256k1::SecretKey::from_slice(data).map_err(transform_secp256k1_error)?,
        };

        Ok(Pair(PrivateType::PrivateKey(private_key)))
    }

    fn from_seed(seed: &Seed) -> Result<Pair> {
        Self::from_seed_slice(&seed[..])
    }

    fn from_seed_slice(seed: &[u8]) -> Result<Pair> {
        Self::new_pair(seed)
    }

    fn public(&self) -> Public {
        match self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE, &r.extended_priv_key);

                Public(PublicType::ExtendedPubKey(ArbitraryNetworkExtendedPubKey {
                    network: r.network,
                    extended_pub_key: pub_key,
                }))
            }
            PrivateType::PrivateKey(r) => {
                let pub_key = PublicKey::from_private_key(&SECP256K1_ENGINE, &r);
                Public(PublicType::PublicKey(pub_key))
            }
        }
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let pk = match self.0 {
            PrivateType::ExtendedPrivKey(epk) => epk.extended_priv_key.private_key,
            PrivateType::PrivateKey(prv) => prv,
        };
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign(&msg, &pk.key);
        Ok(signature.serialize_der().to_vec())
    }

    fn public_key(&self) -> Self::Public {
        match self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE, &r.extended_priv_key);
                Public(PublicType::PublicKey(pub_key.public_key))
            }
            PrivateType::PrivateKey(r) => {
                let pub_key = PublicKey::from_private_key(&SECP256K1_ENGINE, &r);
                Public(PublicType::PublicKey(pub_key))
            }
        }
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
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}

impl std::fmt::Debug for Public {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}

impl TraitPublic for Public {
    fn from_slice(_data: &[u8]) -> core::result::Result<Self, Self::Error> {
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

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match ArbitraryNetworkExtendedPubKey::from_str(s) {
            Ok(r) => Ok(Public(PublicType::ExtendedPubKey(r))),
            Err(_e) => Err(KeyError::InvalidBase58),
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

impl Signer<Signature> for Pair {
    type Error = failure::Error;

    fn sign<T: AsRef<[u8]>>(&self, data: T) -> core::result::Result<Signature, Self::Error> {
        let msg = Message::from_slice(data.as_ref()).map_err(transform_secp256k1_error)?;

        Ok(SECP256K1_ENGINE.sign(&msg, &self.private_key().key))
    }
}

impl Signer<RecoverableSignature> for Pair {
    type Error = failure::Error;

    fn sign<T: AsRef<[u8]>>(
        &self,
        data: T,
    ) -> core::result::Result<RecoverableSignature, Self::Error> {
        let msg = Message::from_slice(data.as_ref()).map_err(transform_secp256k1_error)?;

        Ok(SECP256K1_ENGINE.sign_recoverable(&msg, &(self.private_key().key)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArbitraryNetworkExtendedPrivKey, ArbitraryNetworkExtendedPubKey};

    //TODO add more test
    #[test]
    fn it_works() {}

    #[test]
    fn test_encode_with_network() {
        let main_network_xpub_version: [u8; 4] = [0x04, 0x88, 0xb2, 0x1e];
        let main_network_xprv_version: [u8; 4] = [0x04, 0x88, 0xad, 0xe4];

        let xpub = "tpubDDDcs8o1LaKXKXaPTEVBUZJYTgNAte4xj24MtFCMsfrHku93ZZjy87CGyz93dcocR6x6JHdusHodD9EVcSQuDbmkAWznWZtvyqyMDqS6VK4";
        let epk = ArbitraryNetworkExtendedPubKey::from_str(xpub).unwrap();
        let derivation_info = DerivationInfo::from(epk);
        let ret = derivation_info.encode_with_network(main_network_xpub_version);
        assert_eq!("xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8", ret);

        let xprv = "tprv8g8UWPRHxaNWXZN3uoaiNpyYyaDr2j5Dvcj1vxLxKcEF653k7xcN9wq9eT73wBM1HzE9hmWJbAPXvDvaMXqGWm81UcVpHnmATfH2JJrfhGg";
        let epk = ArbitraryNetworkExtendedPrivKey::from_str(xprv).unwrap();
        let derivation_info = DerivationInfo::from(epk);
        let ret = derivation_info.encode_with_network(main_network_xprv_version);
        assert_eq!("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS", ret);
    }
}