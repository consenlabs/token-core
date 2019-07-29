use primitive_types::H256;


pub trait Hash256 {
    fn hash(data: &[u8]) -> H256;
}

pub struct Keccak256();

impl Hash256 for Keccak256 {
    fn hash(data: &[u8]) -> H256 {
        H256(keccak_hash::keccak(data).0)
    }
}





