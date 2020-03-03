use super::Result;

use crate::ecc::{DeterministicPrivateKey, DeterministicPublicKey};

use crate::sr25519::{Sr25519PrivateKey, Sr25519PublicKey};
use crate::Derive;
use regex::Regex;
use sp_core::crypto::Derive as SpDerive;
use sp_core::crypto::DeriveJunction;

use sp_core::sr25519::Pair;
use sp_core::Pair as TraitPair;

impl Derive for Sr25519PrivateKey {
    fn derive(&self, path: &str) -> Result<Self> {
        let re_junction = Regex::new(r"/(/?[^/]+)")?;
        let junctions = re_junction
            .captures_iter(path)
            .map(|f| DeriveJunction::from(&f[1]));
        Ok(Sr25519PrivateKey(self.0.derive(junctions, None).unwrap().0))
    }
}

impl Derive for Sr25519PublicKey {
    fn derive(&self, path: &str) -> Result<Self> {
        let re_junction = Regex::new(r"/(/?[^/]+)")?;
        let junctions = re_junction
            .captures_iter(path)
            .map(|f| DeriveJunction::from(&f[1]));
        Ok(Sr25519PublicKey(self.0.derive(junctions).unwrap()))
    }
}

impl DeterministicPrivateKey for Sr25519PrivateKey {
    type DeterministicPublicKey = Sr25519PublicKey;
    type PrivateKey = Sr25519PrivateKey;

    fn from_seed(seed: &[u8]) -> Result<Self> {
        let pair = Pair::from_seed_slice(seed).map_err(|_| format_err!("invalid_seed"))?;
        Ok(Sr25519PrivateKey(pair))
    }

    fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        let pair = Pair::from_phrase(mnemonic, None).map_err(|_| format_err!("mnemonic_error"))?;
        Ok(Sr25519PrivateKey(pair.0))
    }

    fn private_key(&self) -> Self::PrivateKey {
        self.clone()
    }

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey {
        Sr25519PublicKey(self.0.public())
    }
}

impl DeterministicPublicKey for Sr25519PublicKey {
    type PublicKey = Sr25519PublicKey;

    fn public_key(&self) -> Self::PublicKey {
        Sr25519PublicKey::from(self.0)
    }
}
