use secp256k1::{Message, Secp256k1, SecretKey};

pub struct Signature {
    pub(crate) v: u64,
    pub(crate) r: Vec<u8>,
    pub(crate) s: Vec<u8>,
}

pub(crate) fn ecdsa_sign(hash: &[u8], private_key: &[u8], chain_id: Option<u64>) -> Signature {
    let s = Secp256k1::signing_only();
    let msg = Message::from_slice(hash).unwrap();
    let key = SecretKey::from_slice(private_key).unwrap();
    let (recovery_id, sig_bytes) = s.sign_recoverable(&msg, &key).serialize_compact();
    let standard_v = recovery_id.to_i32() as u64;
    let v = if let Some(chain_id) = chain_id {
        // When signing with a chain ID, add chain replay protection.
        standard_v + 35 + chain_id * 2
    } else {
        // Otherwise, convert to 'Electrum' notation.
        standard_v + 27
    };
    Signature {
        v,
        r: sig_bytes[0..32].to_vec(),
        s: sig_bytes[32..64].to_vec(),
    }
}
