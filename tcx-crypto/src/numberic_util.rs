use rand::{thread_rng, RngCore};
use std::vec;

pub fn random_iv(len: usize) -> Vec<u8> {
    let mut v = vec![0u8; len];
    thread_rng().fill_bytes(&mut v);
    v
}

#[cfg(test)]
mod tests {
    use crate::numberic_util::random_iv;

    #[test]
    fn it_works() {
        let ret = random_iv(32);
        assert_eq!(32, ret.len());

        let ret = random_iv(64);
        assert_eq!(64, ret.len());
    }
}
