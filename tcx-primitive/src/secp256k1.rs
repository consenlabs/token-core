use super::{key_types, KeyError, KeyTypeId, Pair as TraitPair, Public as TraitPublic, TypedKey};

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{
    ChainCode, ChildNumber, Error as Bip32Error, ExtendedPrivKey, ExtendedPubKey, Fingerprint,
};

use secp256k1::Message;
use std::convert::TryInto;

use std::str::FromStr;

use crate::derive::*;
use crate::KeyError::{CannotDeriveKey, InvalidPublicKey};
use crate::Result;
use bitcoin::util::base58;
use bitcoin::util::base58::Error::InvalidLength;
use bitcoin::{PrivateKey, PublicKey};
use byteorder::BigEndian;
use byteorder::ByteOrder;
use core::fmt;
use lazy_static::lazy_static;

use bip39::Seed;
use tcx_constants::{coin_from_xprv_prefix, coin_from_xpub_prefix, network_from_coin};

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

#[derive(Clone, Debug)]
pub struct ArbitraryNetworkExtendedPubKey {
    pub coin: Option<String>,
    //    pub network: [u8; 4],
    pub extended_pub_key: ExtendedPubKey,
}

impl ArbitraryNetworkExtendedPubKey {
    pub fn derive(&self, child_path: &str) -> Result<ArbitraryNetworkExtendedPubKey> {
        let child_nums = relative_path_to_child_nums(child_path)?;
        let index_ext_pub_key = self
            .extended_pub_key
            .derive_pub(&SECP256K1_ENGINE, &child_nums)?;
        Ok(ArbitraryNetworkExtendedPubKey {
            coin: self.coin.clone(),
            extended_pub_key: index_ext_pub_key,
        })
    }

    pub fn public_key(&self) -> bitcoin::PublicKey {
        self.extended_pub_key.public_key
    }
}

#[derive(Clone, Debug)]
pub struct ArbitraryNetworkExtendedPrivKey {
    pub coin: Option<String>,
    pub extended_priv_key: ExtendedPrivKey,
}

impl fmt::Display for ArbitraryNetworkExtendedPubKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(coin) = &self.coin {
            if let Some(network) = network_from_coin(coin) {
                let mut ret = [0; 78];
                ret[0..4].copy_from_slice(&network.xpub_prefix[..]);
                ret[4] = self.extended_pub_key.depth as u8;
                ret[5..9].copy_from_slice(&self.extended_pub_key.parent_fingerprint[..]);

                BigEndian::write_u32(
                    &mut ret[9..13],
                    u32::from(self.extended_pub_key.child_number),
                );

                ret[13..45].copy_from_slice(&self.extended_pub_key.chain_code[..]);
                ret[45..78].copy_from_slice(&self.extended_pub_key.public_key.key.serialize()[..]);
                return write!(fmt, "{}", &base58::check_encode_slice(&ret[..]));
            }
        }
        write!(fmt, "{}", "invalid_coin");
        Err(fmt::Error)
        //        fmt.write_str(&base58::check_encode_slice(&ret[..]))
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
        let coin = coin_from_xpub_prefix(&data[0..4]);
        Ok(ArbitraryNetworkExtendedPubKey {
            coin,
            extended_pub_key: epk,
        })
    }
}

impl fmt::Display for ArbitraryNetworkExtendedPrivKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(coin) = &self.coin {
            if let Some(network) = network_from_coin(coin) {
                let mut ret = [0; 78];

                ret[0..4].copy_from_slice(&network.xprv_prefix[..]);
                ret[4] = self.extended_priv_key.depth as u8;
                ret[5..9].copy_from_slice(&self.extended_priv_key.parent_fingerprint[..]);

                BigEndian::write_u32(
                    &mut ret[9..13],
                    u32::from(self.extended_priv_key.child_number),
                );

                ret[13..45].copy_from_slice(&self.extended_priv_key.chain_code[..]);
                ret[45] = 0;
                ret[46..78].copy_from_slice(&self.extended_priv_key.private_key[..]);
                return write!(fmt, "{}", &base58::check_encode_slice(&ret[..]));
            }
        }
        write!(fmt, "{}", "invalid_network");
        Err(fmt::Error)
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
            network,
            depth: data[4],
            parent_fingerprint: Fingerprint::from(&data[5..9]),
            child_number,
            chain_code: ChainCode::from(&data[13..45]),
            private_key: PrivateKey {
                compressed: true,
                network,
                key: secp256k1::SecretKey::from_slice(&data[46..78])
                    .map_err(|e| base58::Error::Other(e.to_string()))?,
            },
        };
        let coin = coin_from_xprv_prefix(&data[0..4]);
        Ok(ArbitraryNetworkExtendedPrivKey {
            coin,
            extended_priv_key: epk,
        })
    }
}

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
                    coin: r.coin.clone(),
                    extended_pub_key,
                })
            }
            _ => Err(CannotDeriveKey.into()),
        }
    }

    pub fn extended_priv_key(&self) -> Result<ArbitraryNetworkExtendedPrivKey> {
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let extended_priv_key = r.extended_priv_key;
                Ok(ArbitraryNetworkExtendedPrivKey {
                    coin: r.coin.to_owned(),
                    extended_priv_key,
                })
            }
            _ => Err(CannotDeriveKey.into()),
        }
    }

    pub fn from_wif(wif: &str) -> Result<Self> {
        let pk = bitcoin::PrivateKey::from_wif(wif)?;
        Ok(Pair(PrivateType::PrivateKey(pk)))
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
                                coin: extended_key.coin,
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
        match &self.0 {
            PrivateType::ExtendedPrivKey(r) => {
                let mut extended_key = r.clone();

                for j in path {
                    let child_number = j.try_into()?;

                    match extended_key
                        .extended_priv_key
                        .ckd_priv(&SECP256K1_ENGINE, child_number)
                    {
                        Ok(r) => {
                            extended_key = ArbitraryNetworkExtendedPrivKey {
                                coin: extended_key.coin,
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
                    coin: None,
                    extended_priv_key: r,
                },
            ))),
            Err(e) => Err(transform_bip32_error(e).into()),
        }
    }
}

impl TraitPair for Pair {
    type Public = Public;

    fn from_slice(data: &[u8]) -> Result<Self> {
        let private_key = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: secp256k1::SecretKey::from_slice(data).map_err(transform_secp256k1_error)?,
        };

        Ok(Pair(PrivateType::PrivateKey(private_key)))
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
                let pub_key = ExtendedPubKey::from_private(&SECP256K1_ENGINE, &r.extended_priv_key);

                Ok(Public(PublicType::ExtendedPubKey(
                    ArbitraryNetworkExtendedPubKey {
                        coin: r.coin.clone(),
                        extended_pub_key: pub_key,
                    },
                )))
            }
            PrivateType::PrivateKey(_) => Err(CannotDeriveKey.into()),
        }
    }

    fn public_key(&self) -> Self::Public {
        match &self.0 {
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

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let pk = match &self.0 {
            PrivateType::ExtendedPrivKey(epk) => epk.extended_priv_key.private_key,
            PrivateType::PrivateKey(prv) => prv.clone(),
        };
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign(&msg, &pk.key);
        Ok(signature.serialize_der().to_vec())
    }

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        let pk = match &self.0 {
            PrivateType::ExtendedPrivKey(epk) => epk.extended_priv_key.private_key,
            PrivateType::PrivateKey(prv) => prv.clone(),
        };
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign_recoverable(&msg, &pk.key);
        let (recover_id, sign) = signature.serialize_compact();
        let signed_bytes = [sign[..].to_vec(), vec![recover_id.to_i32() as u8]].concat();
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

impl FromStr for Public {
    type Err = KeyError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        //TODO How to distinguish whether to import from XPub or import from PublicKey
        match ArbitraryNetworkExtendedPubKey::from_str(s) {
            Ok(r) => Ok(Public(PublicType::ExtendedPubKey(r))),
            Err(_e) => Err(KeyError::InvalidBase58),
        }
    }
}
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

#[cfg(test)]
mod tests {
    use crate::derive::Derive;

    use crate::Secp256k1Pair;
    use crate::{
        ArbitraryNetworkExtendedPrivKey, ArbitraryNetworkExtendedPubKey, DerivePath, Pair, Public,
    };
    use bip39::{Language, Mnemonic, Seed};

    use bitcoin_hashes::hex::ToHex;
    use bitcoin_hashes::Hash;
    use std::str::FromStr;

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
        index_xpub_key.coin = Some("BITCOIN".to_owned());
        let xpub = index_xpub_key.to_string();
        assert_eq!(xpub, "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8");
        let pair = Secp256k1Pair::from_seed(&seed).unwrap();
        let mut xprv_key = pair
            .derive(DerivePath::from_str("m/44'/0'/0'").unwrap().into_iter())
            .unwrap()
            .extended_priv_key()
            .unwrap();
        //        let mut account_xprv_key = xprv_key
        xprv_key.coin = Some("BITCOIN".to_owned());
        let xprv = xprv_key.to_string();
        assert_eq!(xprv, "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ");
    }

    #[test]
    fn derive_pub_key_test() {
        let xpub = "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8";
        let xpub_key = ArbitraryNetworkExtendedPubKey::from_str(xpub).unwrap();

        let index_pub_key = xpub_key.derive("0/0").unwrap();

        assert_eq!(
            index_pub_key
                .extended_pub_key
                .public_key
                .to_bytes()
                .to_hex(),
            "026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868"
        );

        let err = ArbitraryNetworkExtendedPubKey::from_str("invalid_xpub")
            .err()
            .unwrap();
        assert_eq!(format!("{}", err), "invalid base58 character 0x6c");
    }

    #[test]
    fn test_encode_with_network() {
        let xpub = "tpubDDDcs8o1LaKXKXaPTEVBUZJYTgNAte4xj24MtFCMsfrHku93ZZjy87CGyz93dcocR6x6JHdusHodD9EVcSQuDbmkAWznWZtvyqyMDqS6VK4";
        let mut xpub_key = ArbitraryNetworkExtendedPubKey::from_str(xpub).unwrap();
        xpub_key.coin = Some("BITCOIN".to_owned());
        let ret = xpub_key.to_string();
        assert_eq!("xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8", ret);

        let xprv = "tprv8g8UWPRHxaNWXZN3uoaiNpyYyaDr2j5Dvcj1vxLxKcEF653k7xcN9wq9eT73wBM1HzE9hmWJbAPXvDvaMXqGWm81UcVpHnmATfH2JJrfhGg";
        let mut xprv_key = ArbitraryNetworkExtendedPrivKey::from_str(xprv).unwrap();
        //        xprv_key.network = main_network_xprv_version;
        xprv_key.coin = Some("BITCOIN".to_owned());
        let ret = xprv_key.to_string();
        assert_eq!("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS", ret);
    }

    #[test]
    fn pair_private_key() {
        let pair = Secp256k1Pair::from_str("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS").unwrap();
        assert!(pair.is_extendable());
        let wif = pair.private_key().to_wif();
        assert_eq!("L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD", wif);

        let pair = Secp256k1Pair::from_wif("L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD")
            .unwrap();
        assert_eq!(
            "L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD",
            pair.private_key().to_wif()
        );
        assert!(!pair.is_extendable());
    }

    #[test]
    fn extended_pub_key_test() {
        let pair = Secp256k1Pair::from_str("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS").unwrap();
        let xpub = pair.extended_pub_key().unwrap();
        assert_eq!("xpub6CSt8ZdrPg6j9ECzMGGDaKJJDUe8Cfm4xHjVrvL7PyGkBGdog8asBznBaZQiYbRtCdWRUAKGpKcbyUYMUUwgmiNt7mPs1QCUMhyHB6rBURT", xpub.to_string());
    }

    #[test]
    fn xpub_from_str_test() {
        let xpub = crate::secp256k1::Public::from_str("xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8").unwrap();
        assert_eq!("xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8", xpub.to_string());
    }
}
