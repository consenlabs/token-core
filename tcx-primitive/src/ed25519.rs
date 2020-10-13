use crate::ecc::{KeyError, PrivateKey as TraitPrivateKey, PublicKey as TraitPublicKey};
use crate::{FromHex, Result, ToHex};
use sp_core::ed25519::{Pair, Public};
use sp_core::{Pair as TraitPair, Public as TraitPublic};

#[derive(Clone)]
pub struct Ed25519PublicKey(pub Public);

#[derive(Clone)]
pub struct Ed25519PrivateKey(pub Pair);

impl From<Public> for Ed25519PublicKey {
    fn from(pk: Public) -> Self {
        Ed25519PublicKey(pk)
    }
}

impl From<Pair> for Ed25519PrivateKey {
    fn from(sk: Pair) -> Self {
        Ed25519PrivateKey(sk)
    }
}

impl TraitPrivateKey for Ed25519PrivateKey {
    type PublicKey = Ed25519PublicKey;

    fn from_slice(data: &[u8]) -> Result<Self> {
        if data.len() != 32 {
            return Err(KeyError::InvalidEd25519Key.into());
        }
        let pair = Pair::from_seed_slice(&data).map_err(|_| KeyError::InvalidEd25519Key)?;
        Ok(Ed25519PrivateKey(pair))
    }

    fn public_key(&self) -> Self::PublicKey {
        Ed25519PublicKey(self.0.public())
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(data).0.to_vec())
    }

    fn sign_recoverable(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.sign(data)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_raw_vec()
    }
}

impl std::fmt::Display for Ed25519PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TraitPublicKey for Ed25519PublicKey {
    fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(Ed25519PublicKey(Public::from_slice(data)))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl ToHex for Ed25519PublicKey {
    fn to_hex(&self) -> String {
        hex::encode(self.0.to_raw_vec())
    }
}

impl FromHex for Ed25519PublicKey {
    fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)?;
        let pk = Ed25519PublicKey::from_slice(bytes.as_slice())?;
        Ok(pk)
    }
}

#[cfg(test)]
mod test {
    use crate::ed25519::Ed25519PrivateKey;
    use crate::PrivateKey;
    use bitcoin_hashes::Hash;
    use blake2b_simd::{blake2b, Params};
    use hex;
    #[test]
    fn from_slice_test() {
        let pk_bytes: Vec<u8> =
            hex::decode("1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap();
        let sk = Ed25519PrivateKey::from_slice(&pk_bytes);
        assert!(sk.is_ok());
    }

    #[test]
    fn sign() {
        let pk_bytes: Vec<u8> =
            hex::decode("2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6f")
                .unwrap();
        let sk_result = Ed25519PrivateKey::from_slice(&pk_bytes);
        assert!(sk_result.is_ok());

        let sk = sk_result.ok().unwrap();
        let msg = hex::decode("03ffaa").unwrap();
        let mut params = Params::new();
        params.hash_length(32);
        let genericHash = params.hash(&msg[..]);
        let sign_result = sk.sign(&genericHash.as_bytes()).unwrap();
        //        println!("sign result ： {}", hex::encode(sign_result));
        let expected_val = "eaab7f4066217b072b79609a9f76cdfadd93f8dde41763887e131c02324f18c8e41b1009e334baf87f9d2e917bf4c0e73165622e5522409a0c5817234a48cc02";
        assert_eq!(hex::encode(sign_result), expected_val);
    }
}
