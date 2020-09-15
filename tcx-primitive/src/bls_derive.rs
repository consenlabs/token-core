use super::Result;
use crate::ecc::KeyError;
use crate::{Derive, DeterministicPrivateKey, DeterministicPublicKey, FromHex, PrivateKey, ToHex};

use crate::bls::{BLSPrivateKey, BLSPublicKey};
use bip39::{Language, Mnemonic};
use bls_key_derivation::{derive_child, derive_master_sk};
use num_bigint::BigUint;

#[derive(Clone)]
pub struct BLSDeterministicPrivateKey(pub BigUint);

#[derive(Clone)]
pub struct BLSDeterministicPublicKey();

impl Derive for BLSDeterministicPrivateKey {
    fn derive(&self, path: &str) -> Result<Self> {
        let mut parts = path.split('/').peekable();
        if *parts.peek().unwrap() == "m" {
            parts.next();
        }

        let result = parts
            .map(str::parse)
            .collect::<std::result::Result<Vec<BigUint>, _>>();
        if result.is_err() {
            return Err(KeyError::InvalidDerivationPathFormat.into());
        }

        let children_nums = result.unwrap();

        let mut children_key = self.0.clone();
        for index in children_nums {
            children_key = derive_child(children_key, index);
        }

        Ok(BLSDeterministicPrivateKey(children_key))
    }
}

impl DeterministicPrivateKey for BLSDeterministicPrivateKey {
    type DeterministicPublicKey = BLSDeterministicPublicKey;
    type PrivateKey = BLSPrivateKey;

    fn from_seed(seed: &[u8]) -> Result<Self> {
        let master_sk = derive_master_sk(seed);
        if master_sk.is_err() {
            return Err(failure::err_msg("invalid seed"));
        }

        Ok(BLSDeterministicPrivateKey(master_sk.unwrap()))
    }

    fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        let mn = Mnemonic::from_phrase(mnemonic, Language::English)?;
        let seed = bip39::Seed::new(&mn, "");
        BLSDeterministicPrivateKey::from_seed(seed.as_bytes())
    }

    fn private_key(&self) -> Self::PrivateKey {
        BLSPrivateKey::from_slice(&self.0.to_bytes_le()).unwrap()
    }

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey {
        panic!("not supported")
    }
}

impl Derive for BLSDeterministicPublicKey {}

impl FromHex for BLSDeterministicPublicKey {
    fn from_hex(_: &str) -> Result<Self> {
        panic!("not supported")
    }
}

impl ToHex for BLSDeterministicPublicKey {
    fn to_hex(&self) -> String {
        panic!("not supported")
    }
}

impl DeterministicPublicKey for BLSDeterministicPublicKey {
    type PublicKey = BLSPublicKey;

    fn public_key(&self) -> Self::PublicKey {
        panic!("not supported")
    }
}

#[cfg(test)]
mod tests {
    use crate::bls_derive::BLSDeterministicPrivateKey;
    use crate::{Derive, DeterministicPrivateKey, PrivateKey};

    #[test]
    fn test_bls_derive() {
        let dsk = BLSDeterministicPrivateKey::from_seed(
            &hex::decode("c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04").unwrap()).unwrap();

        assert_eq!(
            hex::encode(dsk.private_key().to_bytes()),
            "41aaef9bc393b2aa2c9e082d1a75bef4c6428e7be7e35606485538aeba85aa1b"
        );

        assert_eq!(
            hex::encode(dsk.derive("m/0").unwrap().private_key().to_bytes()),
            "08c5678aa7be13f4a5693f3b230155307bf8cdf9c5c00d5654a21d3705506710"
        );
    }
}
