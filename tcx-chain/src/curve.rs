use bitcoin::network::constants::Network;
use secp256k1::{Message, Secp256k1, SecretKey};

use crate::Error;
use crate::Result;
use bip39::Seed;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

//
//pub trait PublicKey: Sized {
//    fn to_bytes(&self) -> Vec<u8>;
//    fn to_compressed(&self) -> Vec<u8>;
//    fn to_uncompressed(&self) -> Vec<u8>;
//
//    fn from_slice(data: &[u8]) -> Result<Self>;
//}
//
//pub trait PrivateKey: Sized {
//    type PublicKey: PublicKey;
//
//    fn is_valid(data: &[u8]) -> bool;
//    fn public_key(&self) -> Self::PublicKey;
//    fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;
//}
//
//impl PublicKey for bitcoin::PublicKey {
//    fn to_bytes(&self) -> Vec<u8> {
//        self.to_bytes()
//    }
//
//    fn to_compressed(&self) -> Vec<u8> {
//        self.key.serialize().to_vec()
//    }
//
//    fn to_uncompressed(&self) -> Vec<u8> {
//        self.key.serialize_uncompressed().to_vec()
//    }
//
//    fn from_slice(data: &[u8]) -> Result<bitcoin::PublicKey> {
//        if let Ok(key) = bitcoin::PublicKey::from_slice(data) {
//            Ok(key)
//        } else {
//            Err(Error::InvalidSecp256k1PublicKey.into())
//        }
//    }
//}
//
////pub type Secp256k1PublicKey = bitcoin::PublicKey;
//
//impl PrivateKey for bitcoin::PrivateKey {
//    type PublicKey = bitcoin::PublicKey;
//
//    fn is_valid(data: &[u8]) -> bool {
//        SecretKey::from_slice(data).is_ok()
//    }
//
//    fn public_key(&self) -> Self::PublicKey {
//        self.public_key(&secp256k1::Secp256k1::new())
//    }
//
//    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
//        let s = Secp256k1::new();
//        let msg = Message::from_slice(data)?;
//        let signature = s.sign(&msg, &self.key);
//        Ok(signature.serialize_der().to_vec())
//    }
//}
//
//pub type Secp256k1PrivateKey = bitcoin::PrivateKey;
//
//pub struct Secp256k1Curve {}
//
//impl Secp256k1Curve {
//    fn _extended_pri_key(path: &str, seed: &Seed) -> Result<ExtendedPrivKey> {
//        let s = Secp256k1::new();
//        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
//        let path = DerivationPath::from_str(path)?;
//        Ok(sk.derive_priv(&s, &path)?)
//    }
//
//    pub fn key_at_paths_with_seed(
//        paths: &[impl AsRef<str>],
//        seed: &Seed,
//    ) -> Result<Vec<impl PrivateKey>> {
//        let s = Secp256k1::new();
//        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())?;
//        let pks: Result<Vec<Secp256k1PrivateKey>> = paths
//            .iter()
//            .map(|path| {
//                let path = DerivationPath::from_str(path.as_ref())?;
//                let prv_key = sk.derive_priv(&s, &path)?;
//                Ok(prv_key.private_key)
//            })
//            .collect();
//        pks
//    }
//
//    pub fn extended_prv_key(path: &str, seed: &Seed) -> Result<DerivationInfo> {
//        let xprv = Self::_extended_pri_key(path, seed)?;
//
//        Ok(DerivationInfo::from(xprv))
//    }
//
//    pub fn extended_pub_key(path: &str, seed: &Seed) -> Result<DerivationInfo> {
//        let s = Secp256k1::new();
//        let xprv = Self::_extended_pri_key(path, seed)?;
//        let xpub = ExtendedPubKey::from_private(&s, &xprv);
//        Ok(DerivationInfo::from(xpub))
//    }
//
//    pub fn derive_pub_key_at_path(xpub: &str, child_path: &str) -> Result<bitcoin::PublicKey> {
//        let ext_pub_key = ExtendedPubKey::from_str(xpub)?;
//        let s = Secp256k1::new();
//        let child_nums = crate::bips::relative_path_to_child_nums(child_path)?;
//        let index_ext_pub_key = ext_pub_key.derive_pub(&s, &child_nums)?;
//        Ok(index_ext_pub_key.public_key)
//    }
//}

#[cfg(test)]
mod tests {
    //    use crate::{PrivateKey, PublicKey};
    //    use crate::{Secp256k1Curve, Secp256k1PrivateKey};
    //    use bip39::{Language, Mnemonic, Seed};
    //    use bitcoin::util::misc::hex_bytes;
    //    use bitcoin::PublicKey as Secp256k1PubKey;
    //    use bitcoin_hashes::hex::ToHex;
    //    use bitcoin_hashes::Hash;
    //
    //    #[test]
    //    fn test_secp256k1_pub_key() {
    //        let ret = Secp256k1PubKey::from_slice(&[0]);
    //        assert_eq!(
    //            "length 1 invalid for this base58 type",
    //            format!("{}", ret.err().unwrap())
    //        );
    //
    //        let ret = Secp256k1PubKey::from_slice(&[0, 1, 2, 3]);
    //        assert_eq!(
    //            "length 4 invalid for this base58 type",
    //            format!("{}", ret.err().unwrap())
    //        );
    //
    //        let pub_bytes = hex_bytes("04506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76").unwrap();
    //
    //        let ret = Secp256k1PubKey::from_slice(&pub_bytes).unwrap();
    //        let bytes = ret.to_bytes();
    //        assert_eq!(bytes, pub_bytes);
    //
    //        let compressed = bitcoin::PublicKey::to_compressed(&ret);
    //        assert_eq!(
    //            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba",
    //            compressed.to_hex()
    //        );
    //
    //        let uncompressed = bitcoin::PublicKey::to_uncompressed(&ret);
    //        assert_eq!("04506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76", uncompressed.to_hex());
    //    }
    //
    //    #[test]
    //    fn test_secp256k1_prv_key() {
    //        assert!(!Secp256k1PrivateKey::is_valid(&[0, 1, 2, 3]));
    //
    //        let maximum_valid_pk_bytes =
    //            hex_bytes("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364140").unwrap();
    //        assert!(Secp256k1PrivateKey::is_valid(&maximum_valid_pk_bytes));
    //
    //        assert!(!Secp256k1PrivateKey::is_valid(&[0]));
    //
    //        let invalid_pk_bytes =
    //            hex_bytes("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141").unwrap();
    //        assert!(!Secp256k1PrivateKey::is_valid(&invalid_pk_bytes));
    //
    //        let prv_key =
    //            Secp256k1PrivateKey::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
    //                .unwrap();
    //        let _expected_pub_key_bytes = hex_bytes("00").unwrap();
    //        let pub_key = PrivateKey::public_key(&prv_key);
    //        assert_eq!(
    //            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba",
    //            pub_key.to_bytes().to_hex()
    //        );
    //    }
    //
    //    #[test]
    //    fn test_secp256k1_sign() {
    //        let prv_key =
    //            Secp256k1PrivateKey::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
    //                .unwrap();
    //        let msg = "TokenCoreX";
    //        let hash = bitcoin_hashes::sha256::Hash::hash(msg.as_bytes());
    //        let signed_bytes = prv_key.sign(&hash.into_inner()).unwrap();
    //        assert_eq!("304402202514266dc7d807ecd69f6d5d03dae7d68619b2c562d8ac77f60e186f4fde4f2202207fbedf5642b095e4a37e71432c99e2b1144f8b9d73a0018be04e6d5ddbd26146", signed_bytes.to_hex());
    //
    //        let wrong_signed = prv_key.sign(&[0, 1, 2, 3]);
    //        assert_eq!(
    //            format!("{}", wrong_signed.err().unwrap()),
    //            "secp: message was not 32 bytes (do you need to hash?)"
    //        )
    //    }
    //
    //    fn default_seed() -> Seed {
    //        let mn = Mnemonic::from_phrase(
    //            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
    //            Language::English,
    //        )
    //        .unwrap();
    //        Seed::new(&mn, "")
    //    }
    //
    //    #[test]
    //    fn test_key_at_paths_with_seed() {
    //        let seed = default_seed();
    //        let paths = vec![
    //            "m/44'/0'/0'/0/0",
    //            "m/44'/0'/0'/0/1",
    //            "m/44'/0'/0'/1/0",
    //            "m/44'/0'/0'/1/1",
    //        ];
    //        let prv_keys = Secp256k1Curve::key_at_paths_with_seed(&paths, &seed).unwrap();
    //        let pub_keys = prv_keys
    //            .iter()
    //            .map(|prv| prv.public_key().to_bytes().to_hex())
    //            .collect::<Vec<String>>();
    //        let expected_pub_keys = vec![
    //            "026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868",
    //            "024fb7df3961e08f01025e434ea19708a4317d2fe59775cddd38df6e8a2d30697d",
    //            "0352470ace48f25b01b9c341e3b0e033fc32a203fb7a81a0453f97d94eca819a35",
    //            "022f4c38f7bbaa00fc886db62f975b34201c2bfed146e98973caf03268941801db",
    //        ];
    //        assert_eq!(pub_keys, expected_pub_keys);
    //    }
    //
    //    #[test]
    //    fn extended_key_test() {
    //        let main_network_xpub_version: [u8; 4] = [0x04, 0x88, 0xb2, 0x1e];
    //        let main_network_xprv_version: [u8; 4] = [0x04, 0x88, 0xad, 0xe4];
    //
    //        let seed = default_seed();
    //        let derivation_info = Secp256k1Curve::extended_pub_key("m/44'/0'/0'", &seed).unwrap();
    //        let xpub = derivation_info.encode_with_network(main_network_xpub_version);
    //        assert_eq!(xpub, "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8");
    //
    //        let derivation_info = Secp256k1Curve::extended_prv_key("m/44'/0'/0'", &seed).unwrap();
    //        let xprv = derivation_info.encode_with_network(main_network_xprv_version);
    //        assert_eq!(xprv, "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ");
    //    }
    //
    //    #[test]
    //    fn derive_pub_key_test() {
    //        let xpub = "xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8";
    //        let pub_key = Secp256k1Curve::derive_pub_key_at_path(xpub, "0/0").unwrap();
    //        assert_eq!(
    //            pub_key.to_bytes().to_hex(),
    //            "026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868"
    //        );
    //
    //        let err = Secp256k1Curve::derive_pub_key_at_path("invalid_xpub", "0/0")
    //            .err()
    //            .unwrap();
    //        assert_eq!(format!("{}", err), "invalid base58 character 0x6c");
    //    }

}
