use blake2b_rs::Blake2bBuilder;

pub enum HashSize {
    Checksum = 4,
    Payload = 20,
}

static CID_PREFIX: &[u8] = &[0x01, 0x71, 0xa0, 0xe4, 0x02, 0x20];

pub fn digest(ingest: &[u8], hash_size: HashSize) -> Vec<u8> {
    //allocate max length byte
    let mut result = [0u8; 32];

    let size = hash_size as usize;
    let mut hasher = Blake2bBuilder::new(size).build();
    hasher.update(ingest);
    hasher.finalize(&mut result);
    result[0..size].to_vec()
}

pub fn message_digest(message: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut message_hasher = Blake2bBuilder::new(32).build();

    message_hasher.update(message);
    message_hasher.finalize(&mut result);

    let mut hasher = Blake2bBuilder::new(32).build();
    hasher.update(CID_PREFIX);
    hasher.update(&result);
    hasher.finalize(&mut result);

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::{digest, message_digest, HashSize};
    use hex::{decode, encode};

    #[test]
    fn test_digest_message() {
        const EXAMPLE_CBOR_DATA: &str =
            "885501fd1d0f4dfcd7e99afcb99a8326b7dc459d32c6285501b882619d46558f3d9e316d11b48dcf211327025a0144000186a0430009c4430061a80040";

        let result = message_digest(&decode(EXAMPLE_CBOR_DATA.as_bytes()).unwrap());

        assert_eq!(
            encode(result),
            "5a51287d2e5401b75014da0f050c8db96fe0bacdad75fce964520ca063b697e1"
        );
    }

    #[test]
    fn test_digest() {
        let payload = [1u8, 2];

        assert_eq!(
            digest(&payload, HashSize::Checksum),
            vec![219, 55, 214, 157]
        );
    }
}
