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

#[cfg(test)]
mod tests {

    use super::{Sr25519PrivateKey, Sr25519PublicKey};
    use crate::derive::Derive;
    use crate::ecc::DeterministicPrivateKey;
    use crate::ecc::DeterministicPublicKey;
    use crate::ecc::PrivateKey;
    use crate::ecc::PublicKey;
    use sp_core::crypto::Pair;
    use tcx_constants::TEST_MNEMONIC;

    #[test]
    fn test_from_seed() {
        let seed = hex::decode("1111111111111111111111111111111111111111111111111111111111111111")
            .unwrap();
        let hd_key = Sr25519PrivateKey::from_seed(&seed).unwrap();
        let pk = hd_key.private_key();
        assert_eq!(
            "50780547322a1ceba67ea8c552c9bc6c686f8698ac9a8cafab7cd15a1db19859",
            hex::encode(pk.0.public().to_vec())
        );
    }

    #[test]
    fn test_from_mnemonic() {
        let hd_key = Sr25519PrivateKey::from_mnemonic(TEST_MNEMONIC).unwrap();
        let pk = hd_key.private_key();
        assert_eq!(
            "fc581c897af481b10cf846d88754f1d115e486e5b7bcc39c0588c01b0a9b7a11",
            hex::encode(pk.0.public().to_vec())
        );
    }

    #[test]
    fn test_private_key_derive() {
        let hd_key: Sr25519PrivateKey = Sr25519PrivateKey::from_mnemonic(TEST_MNEMONIC).unwrap();
        let child_key: Sr25519PrivateKey = hd_key.derive("//imToken//Polakdot//0").unwrap();
        assert_eq!("5022ec28bad21ff2d22d05a9730d4342e0fac36c8a837ca8e1b31a8ab285120e22d0cb94e2bb0f5df0db08a4eaeb49124f5086f8512380206a3f7367e5693fc4", hex::encode(child_key.to_bytes()));
    }

    #[test]
    fn test_deterministic_public_key() {
        let hd_key = Sr25519PrivateKey::from_mnemonic(TEST_MNEMONIC).unwrap();
        let pub_key = hd_key.deterministic_public_key();
        assert_eq!(
            "5Hma6gDS9yY7gPTuAFvmMDNcxPf9JqMZdPsaihfXiyw5NRnQ",
            format!("{}", pub_key.public_key())
        );
    }

    #[test]
    fn test_public_key_derive() {
        let hd_key: Sr25519PrivateKey = Sr25519PrivateKey::from_mnemonic(TEST_MNEMONIC).unwrap();
        let hd_pub_key: Sr25519PublicKey = hd_key.deterministic_public_key();
        let child_key: Sr25519PublicKey = hd_pub_key.derive("/imToken/Polakdot/0").unwrap();
        assert_eq!(
            "8a8ae5479922fc2dac8a8fe867b20afada11edc63bca61793bedd6e5fc50c954",
            hex::encode(child_key.to_bytes())
        );
    }
}
