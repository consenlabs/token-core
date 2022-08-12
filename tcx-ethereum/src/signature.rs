use ethereum_types::H256;
use secp256k1::{Message, Secp256k1, SecretKey};

pub struct Signature {
    pub(crate) v: u64,
    pub(crate) r: H256,
    pub(crate) s: H256,
}

pub(crate) fn ecdsa_sign(hash: &[u8], private_key: &[u8]) -> Signature {
    let s = Secp256k1::signing_only();
    let msg = Message::from_slice(hash).unwrap();
    let key = SecretKey::from_slice(private_key).unwrap();
    let (recovery_id, sig_bytes) = s.sign_recoverable(&msg, &key).serialize_compact();
    let r = H256::from_slice(&sig_bytes[..32]);
    let s = H256::from_slice(&sig_bytes[32..]);
    Signature {
        v: recovery_id.to_i32() as u64,
        r,
        s,
    }
}
