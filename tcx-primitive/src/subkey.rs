use super::Result;

use crate::ecc::{DeterministicPrivateKey, DeterministicPublicKey, KeyError};

use crate::sr25519::{Sr25519PrivateKey, Sr25519PublicKey};
use crate::{Derive, FromHex, PrivateKey, PublicKey, Ss58Codec, ToHex};
use regex::Regex;
use sp_core::crypto::Derive as SpDerive;
use sp_core::crypto::DeriveJunction;
use sp_core::crypto::Ss58Codec as SubSs58Codec;
use sp_core::sr25519::Pair;
use sp_core::{Pair as TraitPair, Public as TraitPublic};
use std::convert::TryInto;

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

    use super::{Sr25519PrivateKey, Ss58Codec};

    use crate::PrivateKey;

    use bitcoin_hashes::hex::ToHex;
    use bitcoin_hashes::Hash;

    use schnorrkel::{ExpansionMode, MiniSecretKey};
    use sp_core::sr25519::Pair;
    use tcx_constants::coin_info::coin_info_from_param;

    //    #[test]
    //    fn construct_test() {
    //        //       let entropy = "54101bfe06f6fc404289b973d6e4e7cf";
    //        let seed_hex = "9dd32e5182f147ffe08fee7c1b449647b5e17a89d35622c9d603c41b6a3937c717f8cf9db7d5293de58d14680ec3e7b897398026352b84e224017f5b82acc6fa";
    //        let seed = hex::decode(seed_hex).unwrap();
    //        let mini_key =
    //            MiniSecretKey::from_bytes(&seed[..32]).expect("Length is always correct; qed");
    //
    //        //
    //        let kp = mini_key.expand_to_keypair(ExpansionMode::Ed25519);
    //        let pair = Sr25519PrivateKey(Pair::from(kp));
    //        let sub_seed = mini_key.to_bytes();
    //
    //        //       let pair = Pair::from_entropy(&hex::decode("54101bfe06f6fc404289b973d6e4e7cf").unwrap(), None);
    //        assert_eq!(hex::encode(sub_seed), "");
    //    }
}
