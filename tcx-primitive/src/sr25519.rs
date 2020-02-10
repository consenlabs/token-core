


#[derive(Clone)]
pub struct Sr25519PublicKey(pub PublicKey);

#[derive(Clone)]
pub struct Sr25519PrivateKey(pub PrivateKey);

impl From<PublicKey> for Sr25519PublicKey {
    fn from(pk: PublicKey) -> Self {
        Sr25519PublicKey(pk)
    }
}

impl From<PrivateKey> for Sr25519PrivateKey {
    fn from(sk: PrivateKey) -> Self {
        Sr25519PrivateKey(sk)
    }
}

impl Sr25519PublicKey {
    pub fn to_compressed(&self) -> Vec<u8> {
        self.0.key.serialize().to_vec()
    }

    pub fn to_uncompressed(&self) -> Vec<u8> {
        self.0.key.serialize_uncompressed().to_vec()
    }
}

impl Sr25519PrivateKey {
    pub fn from_wif(wif: &str) -> Result<Self> {
        Secp256k1PrivateKey::from_ss58check(wif)
    }
}

impl TraitPrivateKey for Sr25519PrivateKey {
    type PublicKey = Sr25519PublicKey;

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

impl std::fmt::Display for Sr25519PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.key.fmt(f)
    }
}

impl TraitPublicKey for Sr25519PublicKey {
    fn from_slice(data: &[u8]) -> Result<Self> {
        let key = PublicKey::from_slice(data)?;
        Ok(Secp256k1PublicKey(key))
    }

    fn write_into<W: io::Write>(&self, writer: W) {
        self.0.write_into(writer);
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl Ss58Codec for Sr25519PrivateKey {
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