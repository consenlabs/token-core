use digest::Digest;
use sha2::Sha256;

pub fn dsha256(bytes: &[u8]) -> Vec<u8> {
    Sha256::digest(&Sha256::digest(&bytes)).to_vec()
}

pub fn hex_dsha256(hex: &str) -> String {
    let key_data: Vec<u8> = hex::decode(hex).expect("hex can't decode");
    hex::encode(dsha256(&key_data))
}

pub fn str_dsha256(str: &str) -> String {
    let key_data = str.as_bytes();
    hex::encode(dsha256(&key_data))
}

#[cfg(test)]
mod tests {
    use crate::hash::{dsha256, hex_dsha256, str_dsha256};

    #[test]
    fn sha256_test() {
        assert_eq!(
            hex::encode(dsha256(vec![0x1, 0x2, 0x3, 0x4].as_slice())),
            "8de472e2399610baaa7f84840547cd409434e31f5d3bd71e4d947f283874f9c0"
        );
        assert_eq!(
            hex_dsha256("01020304"),
            "8de472e2399610baaa7f84840547cd409434e31f5d3bd71e4d947f283874f9c0"
        );
        assert_eq!(
            str_dsha256("01020304"),
            "26a0f059b048e922a223ff432ce9c87b13df2f25adc8e876a79a15326519fd76"
        );
    }
}
