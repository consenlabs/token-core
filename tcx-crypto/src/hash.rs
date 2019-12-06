use digest::Digest;
use sha2::Sha256;

pub fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(&Sha256::digest(&bytes)))
}

pub fn hex_sha256(hex: &str) -> String {
    let key_data: Vec<u8> = hex::decode(hex).expect("hex can't decode");
    sha256(&key_data)
}

pub fn str_sha256(str: &str) -> String {
    let key_data = str.as_bytes();
    sha256(&key_data)
}
