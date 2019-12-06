use digest::Digest;
use sha2::Sha256;

pub fn str_sha256(hex: &str) -> String {
    let bytes: Vec<u8> = hex::decode(hex).expect("hex can't decode");
    sha256(&bytes)
}

pub fn sha256(bytes: &[u8]) -> String {
    hex::encode(&Sha256::digest(&bytes))
}
