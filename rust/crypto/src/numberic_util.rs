use rand::{thread_rng, RngCore};
use std::vec;

pub fn random_iv(len: usize) -> Vec<u8>  {
    let mut v = vec![0u8; len];
    thread_rng().fill_bytes(&mut v);
    return v
}
