use rand::{thread_rng, Rng, RngCore};

pub fn random_iv() -> [u8; 16]  {
    let mut v = [0u8; 16];
    thread_rng().fill_bytes(&mut v);
    return v
}