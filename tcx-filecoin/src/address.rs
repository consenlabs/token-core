use tcx_chain::{Address, Result};
use tcx_constants::CoinInfo;
use tcx_primitive::{PublicKey, TypedPublicKey};

use super::Error;
use base32::Alphabet;
use blake2b_rs::Blake2bBuilder;

const MAINNET_PREFIX: &'static str = "f";
const TESTNET_PREFIX: &'static str = "t";

#[derive(Clone, Copy)]
pub enum Protocol {
    Secp256k1 = 1,
}

pub enum HashSize {
    Checksum = 4,
    Payload = 20,
}

pub struct FilecoinAddress();

impl FilecoinAddress {
    fn hash(ingest: &[u8], hash_size: HashSize) -> Vec<u8> {
        //allocate max length byte
        let mut result = [0u8; 32];

        let size = hash_size as usize;
        let mut hasher = Blake2bBuilder::new(size).build();
        hasher.update(ingest);
        hasher.finalize(&mut result);
        result[0..size].to_vec()
    }

    fn checksum(ingest: &[u8]) -> Vec<u8> {
        Self::hash(ingest, HashSize::Checksum)
    }

    fn address_hash(ingest: &[u8]) -> Vec<u8> {
        Self::hash(ingest, HashSize::Payload)
    }
}

impl Address for FilecoinAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        let ntwk = match coin.network.as_str() {
            "TESTNET" => TESTNET_PREFIX,
            _ => MAINNET_PREFIX,
        };
        let protocol;
        let payload;
        let cksm;

        match public_key {
            TypedPublicKey::Secp256k1(pk) => {
                protocol = Protocol::Secp256k1;
                payload = Self::address_hash(&pk.to_bytes());

                cksm = Self::checksum(&[vec![protocol as u8], payload.clone().to_vec()].concat());
            }
            _ => return Err(Error::InvalidCurveType.into()),
        };

        Ok(format!(
            "{}{}{}",
            ntwk,
            protocol as i8,
            base32::encode(
                Alphabet::RFC4648 { padding: false },
                &[payload, cksm].concat()
            )
            .to_lowercase()
        ))
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        // TODO validate the address
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::address::FilecoinAddress;
    use crate::address::HashSize;
    use tcx_chain::Address;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::{PublicKey, Secp256k1PublicKey, TypedPublicKey};

    #[test]
    fn hash() {
        let payload = [1u8, 2];

        assert_eq!(
            FilecoinAddress::hash(&payload, HashSize::Checksum),
            vec![219, 55, 214, 157]
        );
    }

    #[test]
    fn secp256k1_address() {
        let test_cases = vec![
            (
                vec![
                    4, 148, 2, 250, 195, 126, 100, 50, 164, 22, 163, 160, 202, 84, 38, 181, 24, 90,
                    179, 178, 79, 97, 52, 239, 162, 92, 228, 135, 200, 45, 46, 78, 19, 191, 69, 37,
                    17, 224, 210, 36, 84, 33, 248, 97, 59, 193, 13, 114, 250, 33, 102, 102, 169,
                    108, 59, 193, 57, 32, 211, 255, 35, 63, 208, 188, 5,
                ],
                "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
            ),
            (
                vec![
                    4, 118, 135, 185, 16, 55, 155, 242, 140, 190, 58, 234, 103, 75, 18, 0, 12, 107,
                    125, 186, 70, 255, 192, 95, 108, 148, 254, 42, 34, 187, 204, 38, 2, 255, 127,
                    92, 118, 242, 28, 165, 93, 54, 149, 145, 82, 176, 225, 232, 135, 145, 124, 57,
                    53, 118, 238, 240, 147, 246, 30, 189, 58, 208, 111, 127, 218,
                ],
                "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
            ),
            (
                vec![
                    4, 222, 253, 208, 16, 1, 239, 184, 110, 1, 222, 213, 206, 52, 248, 71, 167, 58,
                    20, 129, 158, 230, 65, 188, 182, 11, 185, 41, 147, 89, 111, 5, 220, 45, 96, 95,
                    41, 133, 248, 209, 37, 129, 45, 172, 65, 99, 163, 150, 52, 155, 35, 193, 28,
                    194, 255, 53, 157, 229, 75, 226, 135, 234, 98, 49, 155,
                ],
                "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
            ),
            (
                vec![
                    4, 3, 237, 18, 200, 20, 182, 177, 13, 46, 224, 157, 149, 180, 104, 141, 178,
                    209, 128, 208, 169, 163, 122, 107, 106, 125, 182, 61, 41, 129, 30, 233, 115, 4,
                    121, 216, 239, 145, 57, 233, 18, 73, 202, 189, 57, 50, 145, 207, 229, 210, 119,
                    186, 118, 222, 69, 227, 224, 133, 163, 118, 129, 191, 54, 69, 210,
                ],
                "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
            ),
            (
                vec![
                    4, 247, 150, 129, 154, 142, 39, 22, 49, 175, 124, 24, 151, 151, 181, 69, 214,
                    2, 37, 147, 97, 71, 230, 1, 14, 101, 98, 179, 206, 158, 254, 139, 16, 20, 65,
                    97, 169, 30, 208, 180, 236, 137, 8, 0, 37, 63, 166, 252, 32, 172, 144, 251,
                    241, 251, 242, 113, 48, 164, 236, 195, 228, 3, 183, 5, 118,
                ],
                "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
            ),
            (
                vec![
                    4, 66, 131, 43, 248, 124, 206, 158, 163, 69, 185, 3, 80, 222, 125, 52, 149,
                    133, 156, 164, 73, 5, 156, 94, 136, 221, 231, 66, 133, 223, 251, 158, 192, 30,
                    186, 188, 95, 200, 98, 104, 207, 234, 235, 167, 174, 5, 191, 184, 214, 142,
                    183, 90, 82, 104, 120, 44, 248, 111, 200, 112, 43, 239, 138, 31, 224,
                ],
                "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
            ),
        ];

        let coin_info = CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        for (input, expected) in test_cases {
            let pk = TypedPublicKey::from_slice(CurveType::SECP256k1, &input).unwrap();
            let address = FilecoinAddress::from_public_key(&pk, &coin_info).unwrap();
            assert_eq!(address, expected);
        }
    }
}
