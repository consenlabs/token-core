use super::Result;
use crate::derive::Derive;
use crate::ecc::KeyError;
use crate::ed25519::{Ed25519PrivateKey, Ed25519PublicKey};
use crate::{
    DeterministicPrivateKey, DeterministicPublicKey, FromHex, PrivateKey, PublicKey, ToHex,
};
use bip39::{Language, Mnemonic};
use bitcoin::hashes::{sha512, Hash, HashEngine, Hmac, HmacEngine};
use bitcoin::util::bip32::{ChildNumber, Error as Bip32Error};
use ed25519_bip32::{DerivationIndex, DerivationScheme, XPrv, XPub};
use sp_core::ed25519::Pair;

pub struct Ed25519DeterministicPrivateKey(XPrv);

pub struct Ed25519DeterministicPublicKey(XPub);

#[cfg_attr(tarpaulin, skip)]
fn transform_ed25519_bip32_error(err: Bip32Error) -> KeyError {
    match err {
        Bip32Error::Ecdsa(_) => KeyError::InvalidEcdsa,
        Bip32Error::RngError(_) => KeyError::OverflowChildNumber,
        Bip32Error::CannotDeriveFromHardenedKey => KeyError::CannotDeriveFromHardenedKey,
        Bip32Error::InvalidChildNumber(_) => KeyError::InvalidChildNumber,
        Bip32Error::InvalidChildNumberFormat => KeyError::InvalidChildNumber,
        Bip32Error::InvalidDerivationPathFormat => KeyError::InvalidDerivationPathFormat,
    }
}

impl Ed25519DeterministicPrivateKey {
    pub fn from_seed(seed: &[u8]) -> Result<Self> {
        let mut hmac_engine: HmacEngine<sha512::Hash> = HmacEngine::new(b"Bitcoin seed");
        hmac_engine.input(seed);
        let hmac_result: Hmac<sha512::Hash> = Hmac::from_engine(hmac_engine);
        let mut temp_byte: [u8; 32] = [0; 32];
        temp_byte.copy_from_slice(&hmac_result[..32]);
        let mut chain_code: [u8; 32] = [0; 32];
        chain_code.copy_from_slice(&hmac_result[32..]);
        let master_key = XPrv::from_nonextended_force(&temp_byte, &chain_code);
        Ok(Ed25519DeterministicPrivateKey(master_key))
    }

    pub fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        let mn = Mnemonic::from_phrase(mnemonic, Language::English)?;
        let seed = bip39::Seed::new(&mn, "");
        Ok(Self::from_seed(seed.as_ref())?)
    }
}

impl Derive for Ed25519DeterministicPrivateKey {
    fn derive(&self, path: &str) -> Result<Self> {
        let mut extended_key = self.0.clone();

        let mut parts = path.split('/').peekable();
        if *parts.peek().unwrap() == "m" {
            parts.next();
        }
        let ret: std::result::Result<Vec<ChildNumber>, bitcoin::util::bip32::Error> =
            parts.map(str::parse).collect();
        let children_nums = ret.map_err(transform_ed25519_bip32_error)?;

        for child_number in children_nums {
            let chain_index = match child_number {
                ChildNumber::Normal { index } => index,
                ChildNumber::Hardened { index } => index | (1 << 31),
            };
            extended_key = extended_key.derive(DerivationScheme::V2, chain_index);
        }

        Ok(Ed25519DeterministicPrivateKey(extended_key))
    }
}

impl Derive for Ed25519DeterministicPublicKey {
    fn derive(&self, path: &str) -> Result<Self> {
        let mut extended_key = self.0.clone();

        let mut parts = path.split('/').peekable();
        if *parts.peek().unwrap() == "m" {
            parts.next();
        }
        let ret: std::result::Result<Vec<ChildNumber>, bitcoin::util::bip32::Error> =
            parts.map(str::parse).collect();
        let children_nums = ret.map_err(transform_ed25519_bip32_error)?;

        for child_number in children_nums {
            let chain_index = match child_number {
                ChildNumber::Normal { index } => index,
                ChildNumber::Hardened { index } => index | (1 << 31),
            };
            extended_key = extended_key.derive(DerivationScheme::V2, chain_index)?;
        }

        Ok(Ed25519DeterministicPublicKey(extended_key))
    }
}

impl DeterministicPrivateKey for Ed25519DeterministicPrivateKey {
    type DeterministicPublicKey = Ed25519DeterministicPublicKey;
    type PrivateKey = Ed25519PrivateKey;

    fn from_seed(seed: &[u8]) -> Result<Self> {
        let mut hmac_engine: HmacEngine<sha512::Hash> = HmacEngine::new(b"Bitcoin seed");
        hmac_engine.input(seed);
        let hmac_result: Hmac<sha512::Hash> = Hmac::from_engine(hmac_engine);
        let mut hash_left = [0; 32];
        hash_left.copy_from_slice(&hmac_result[..32]);
        let mut hash_rigth = [0; 32];
        hash_rigth.copy_from_slice(&hmac_result[32..]);
        let master_key = XPrv::from_nonextended_force(&hash_left, &&hash_rigth);
        Ok(Ed25519DeterministicPrivateKey(master_key))
    }

    fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        let mn = Mnemonic::from_phrase(mnemonic, Language::English)?;
        let seed = bip39::Seed::new(&mn, "");
        Ok(Self::from_seed(seed.as_ref())?)
    }

    fn private_key(&self) -> Self::PrivateKey {
        Ed25519PrivateKey::from_slice(self.0.as_ref()).unwrap()
    }

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey {
        Ed25519DeterministicPublicKey(self.0.public())
    }
}

impl DeterministicPublicKey for Ed25519DeterministicPublicKey {
    type PublicKey = Ed25519PublicKey;

    fn public_key(&self) -> Self::PublicKey {
        Ed25519PublicKey::from_slice(self.0.as_ref()).unwrap()
    }
}

impl ToString for Ed25519DeterministicPrivateKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl ToString for Ed25519DeterministicPublicKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl ToHex for Ed25519DeterministicPublicKey {
    fn to_hex(&self) -> String {
        self.to_hex()
    }
}

impl FromHex for Ed25519DeterministicPublicKey {
    fn from_hex(hex: &str) -> Result<Self> {
        let data = hex::decode(hex)?;

        if data.len() != 74 {
            return Err(KeyError::InvalidBase58.into());
        }
        let mut pk: [u8; 32] = [0; 32];
        pk.copy_from_slice(&data[41..74]);
        let mut chain_code: [u8; 32] = [0; 32];
        chain_code.copy_from_slice(&data[9..41]);
        Ok(Ed25519DeterministicPublicKey(XPub::from_pk_and_chaincode(
            &pk,
            &chain_code,
        )))
    }
}
#[cfg(test)]
mod test {
    use crate::ed25519_bip32::{Ed25519DeterministicPrivateKey, Ed25519DeterministicPublicKey};
    use crate::{Derive, DeterministicPrivateKey, PrivateKey, PublicKey};
    use bip39::{Language, Mnemonic, Seed};

    fn default_seed() -> Seed {
        let mn = Mnemonic::from_phrase(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            Language::English,
        )
        .unwrap();
        Seed::new(&mn, "")
    }

    #[test]
    fn derive_public_keys() {
        let seed = default_seed();
        let paths = vec![
            "m/44'/0'/0'/0/0",
            "m/44'/0'/0'/0/1",
            "m/44'/0'/0'/1/0",
            "m/44'/0'/0'/1/1",
        ];

        let esk = Ed25519DeterministicPrivateKey::from_seed(seed.as_bytes()).unwrap();

        let pub_keys = paths
            .iter()
            .map(|path| hex::encode(esk.derive(path).unwrap().0.public().as_ref()))
            .collect::<Vec<String>>();
        let expected_pub_keys = vec![
            "d0c5ee97112a8a6f192ec44ab10f6a51bbfa327f7736e8e8b30b9ec636bc533b4a5a79f69bacc08e3c546cf8cc912ab820eb933f18af0630eb9c5978c41c37e2",
            "72cc175bb3cec0b14c145c60dfc8044857af20408d13bd27d37bb22131802650ca56b9788a0c4bfc220326eb1f03911fd504c180f988e69370dd1f3140193671",
            "33b203d6231b7bd8a465799e210345c12446b1be99918e32392ce89c77c1d9fa371db326eba11ac066afa58f895d459272862957a5df8b48b6d81cadd5d36196",
            "8cc4569d3ab5a83638b65072349075ad58126204c1085e64492fac932e8077c942c1fc372392916a4c28ffd37ae81653ac7ed82311ff4ac1a6d72a763da1ae81",
        ];
        assert_eq!(pub_keys, expected_pub_keys);
    }

    #[test]
    fn derive_key() {
        let seed = default_seed();
        let root = Ed25519DeterministicPrivateKey::from_seed(seed.as_bytes()).unwrap();
        let dpk = root
            .derive("m/44'/0'/0'")
            .unwrap()
            .deterministic_public_key();
        assert_eq!(dpk.to_string(), "37491e84ade93e4456ee6cf923f0d148d8f32ffd0a988159b42e6f5a1b7c2d06db982cd0653d1173d6d8836a13fa77ad0050200911204815973d33ae1b4d6cf1")
    }
}
