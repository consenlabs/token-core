use crate::ecc::{KeyError, PrivateKey as TraitPrivateKey, PublicKey as TraitPublicKey};
use crate::Result;
use bls_signatures::{PrivateKey, PublicKey, Serialize};

#[derive(Clone)]
pub struct BLSPublicKey(PublicKey);

#[derive(Clone)]
pub struct BLSPrivateKey(PrivateKey);

impl From<PublicKey> for BLSPublicKey {
    fn from(pk: PublicKey) -> Self {
        BLSPublicKey(pk)
    }
}

impl From<PrivateKey> for BLSPrivateKey {
    fn from(sk: PrivateKey) -> Self {
        BLSPrivateKey(sk)
    }
}

impl TraitPrivateKey for BLSPrivateKey {
    type PublicKey = BLSPublicKey;

    fn from_slice(data: &[u8]) -> Result<Self> {
        let mut temp_data = data.to_vec();
        temp_data.resize(32, 0u8);
        Ok(BLSPrivateKey(PrivateKey::from_bytes(temp_data.as_ref())?))
    }

    fn public_key(&self) -> Self::PublicKey {
        BLSPublicKey(self.0.public_key())
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(message).as_bytes())
    }

    fn sign_recoverable(&self, _: &[u8]) -> Result<Vec<u8>> {
        Err(KeyError::NotImplement.into())
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }
}

impl TraitPublicKey for BLSPublicKey {
    fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(BLSPublicKey(PublicKey::from_bytes(data)?))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::bls::BLSPrivateKey;
    use crate::{PrivateKey, PublicKey};

    #[test]
    fn test_bls_private_key() {
        let private_key = BLSPrivateKey::from_slice(
            &hex::decode("0ef71710671a9f1cfc4bd441c017c9b6db68491929facc68ab072a9676e9e23c")
                .unwrap(),
        )
        .unwrap();

        assert_eq!(hex::encode(private_key.public_key().to_bytes()),
                   "b2be11dc8e54ee74dbc07569fd74fe03b5f52ad71cd49a8579b6c6387891f5a20ad980ec2747618c1b9ad35846a68a3e");
    }
}
