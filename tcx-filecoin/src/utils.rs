use blake2b_rs::Blake2bBuilder;

pub enum HashSize {
    Checksum = 4,
    Payload = 20,
    Default = 32,
}

pub fn digest(ingest: &[u8], hash_size: HashSize) -> Vec<u8> {
    //allocate max length byte
    let mut result = [0u8; 32];

    let size = hash_size as usize;
    let mut hasher = Blake2bBuilder::new(size).build();
    hasher.update(ingest);
    hasher.finalize(&mut result);
    result[0..size].to_vec()
}

#[cfg(test)]
mod tests {
    use crate::utils::{digest, HashSize};

    #[test]
    fn test_digest() {
        let payload = [1u8, 2];

        assert_eq!(
            digest(&payload, HashSize::Checksum),
            vec![219, 55, 214, 157]
        );
    }
}
