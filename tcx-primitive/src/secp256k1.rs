use crate::constant::SECP256K1_ENGINE;
use crate::ecc::{KeyError, PrivateKey as TraitPrivateKey, PublicKey as TraitPublicKey};

use bitcoin::Network;

use bitcoin::util::key::{PrivateKey, PublicKey};

use crate::{Result, Ss58Codec};
use bitcoin::util::base58;

use bitcoin::secp256k1::Message;
use tcx_constants::{network_from_coin, CoinInfo};

#[cfg_attr(tarpaulin, skip)]
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

#[derive(Clone)]
pub struct Secp256k1PublicKey(pub PublicKey);

#[derive(Clone)]
pub struct Secp256k1PrivateKey(pub PrivateKey);

impl From<PublicKey> for Secp256k1PublicKey {
    fn from(pk: PublicKey) -> Self {
        Secp256k1PublicKey(pk)
    }
}

impl From<PrivateKey> for Secp256k1PrivateKey {
    fn from(sk: PrivateKey) -> Self {
        Secp256k1PrivateKey(sk)
    }
}

impl Secp256k1PublicKey {
    pub fn to_compressed(&self) -> Vec<u8> {
        self.0.key.serialize().to_vec()
    }

    pub fn to_uncompressed(&self) -> Vec<u8> {
        self.0.key.serialize_uncompressed().to_vec()
    }
}

impl Secp256k1PrivateKey {
    pub fn from_wif(wif: &str) -> Result<Self> {
        Secp256k1PrivateKey::from_ss58check(wif)
    }
}

impl TraitPrivateKey for Secp256k1PrivateKey {
    type PublicKey = Secp256k1PublicKey;

    fn from_slice(data: &[u8]) -> Result<Self> {
        let key = secp256k1::SecretKey::from_slice(data).map_err(transform_secp256k1_error)?;
        Ok(Secp256k1PrivateKey(PrivateKey {
            key,
            compressed: true,
            network: Network::Bitcoin,
        }))
    }

    fn public_key(&self) -> Self::PublicKey {
        Secp256k1PublicKey(self.0.public_key(&SECP256K1_ENGINE))
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign(&msg, &self.0.key);
        Ok(signature.serialize_der().to_vec())
    }

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        let msg = Message::from_slice(data).map_err(transform_secp256k1_error)?;
        let signature = SECP256K1_ENGINE.sign_recoverable(&msg, &self.0.key);
        let (recover_id, sign) = signature.serialize_compact();
        let signed_bytes = [sign[..].to_vec(), vec![(recover_id.to_i32()) as u8]].concat();
        Ok(signed_bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl std::fmt::Display for Secp256k1PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.key.fmt(f)
    }
}

impl TraitPublicKey for Secp256k1PublicKey {
    fn from_slice(data: &[u8]) -> Result<Self> {
        let key = PublicKey::from_slice(data)?;
        Ok(Secp256k1PublicKey(key))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl Ss58Codec for Secp256k1PrivateKey {
    fn from_ss58check_with_version(wif: &str) -> Result<(Self, Vec<u8>)> {
        let data = base58::from_check(wif)?;

        let compressed = match data.len() {
            33 => false,
            34 => true,
            _ => {
                return Err(KeyError::InvalidPrivateKey.into());
            }
        };

        let pk = Secp256k1PrivateKey(PrivateKey {
            key: secp256k1::SecretKey::from_slice(&data[1..33])?,
            compressed,
            network: Network::Bitcoin,
        });

        Ok((pk, vec![data[0]]))
    }

    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
        let mut ret = [0; 34];
        ret[0..1].copy_from_slice(&version[0..]);
        ret[1..33].copy_from_slice(&self.0.key[..]);
        if self.0.compressed {
            ret[33] = 1;
            base58::check_encode_slice(&ret[..]).to_string()
        } else {
            base58::check_encode_slice(&ret[..33]).to_string()
        }
    }
}

pub fn verify_private_key(private_key: &str, coin: &CoinInfo) -> Result<String> {
    if let Some(network) = network_from_coin(coin) {
        let (pk, version) = Secp256k1PrivateKey::from_ss58check_with_version(private_key)?;
        if version[0] != network.private_prefix {
            return Err(KeyError::InvalidPrivateKey.into());
        } else {
            return Ok(hex::encode(pk.to_bytes()));
        }
    }
    Ok(private_key.to_string())
}

pub fn private_key_without_version(private_key: &str) -> Result<Vec<u8>> {
    let (pk, _version) = Secp256k1PrivateKey::from_ss58check_with_version(private_key)?;
    Ok(pk.to_bytes())
}

#[cfg(test)]
mod tests {

    use super::{verify_private_key, Secp256k1PrivateKey, Ss58Codec};

    use crate::PrivateKey;

    use bitcoin_hashes::hex::ToHex;
    use bitcoin_hashes::Hash;

    use tcx_constants::coin_info::coin_info_from_param;

    #[test]
    fn secp256k1_prv_key() {
        let private_key =
            Secp256k1PrivateKey::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
                .unwrap();
        let pub_key = private_key.public_key();
        let compressed_pub_key = pub_key.to_compressed().to_hex();
        assert_eq!(
            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba",
            compressed_pub_key
        );

        let uncompressed_pub_key = pub_key.to_uncompressed().to_hex();
        assert_eq!(
            "04506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76",
            uncompressed_pub_key
        );

        assert_eq!(
            format!("{}", pub_key),
            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba"
        );

        let ret =
            Secp256k1PrivateKey::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZ");
        assert!(ret.is_err());
    }

    #[test]
    fn secp256k1_sign() {
        let prv_key =
            Secp256k1PrivateKey::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
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

    #[test]
    fn private_key() {
        let private_key =
            Secp256k1PrivateKey::from_wif("L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD")
                .unwrap();
        assert_eq!(
            "L2saPfZaQWXY6AMxBdLy4UdR8M3xz698fVo3HY5rmRPZDgHe2nAD",
            private_key.to_ss58check_with_version(&[0x80])
        );
    }

    #[test]
    fn wif_with_version() {
        let (pk, version) = Secp256k1PrivateKey::from_ss58check_with_version(
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
    fn verify_wif_test() {
        let coin_info = coin_info_from_param("LITECOIN", "MAINNET", "NONE", "").unwrap();
        let ret = verify_private_key(
            "6v3S2CrndTdGH8QS1Fw9cWZKJWfee52KytmiB687HPbPBdobUX9",
            &coin_info,
        );

        assert!(ret.is_ok());
        let ret = verify_private_key(
            "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
            &coin_info,
        );
        assert!(ret.is_err());
        assert_eq!("invalid_private_key", format!("{}", ret.err().unwrap()))
    }
}
