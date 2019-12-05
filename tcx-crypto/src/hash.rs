use digest::Digest;
use sha2::Sha256;

pub fn str_sha256(hex: &str) -> String {
    let key_data: Vec<u8> = hex::decode(hex).expect("hex can't decode");
    hex::encode(Sha256::digest(&Sha256::digest(&key_data)))
}
