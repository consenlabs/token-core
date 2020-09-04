use tcx_chain::{Address, Result};
use tcx_constants::CoinInfo;
use tcx_primitive::{PublicKey, TypedPublicKey};

use super::Error;
use crate::utils::{digest, HashSize};
use base32::Alphabet;

const MAINNET_PREFIX: &'static str = "f";
const TESTNET_PREFIX: &'static str = "t";

#[derive(Clone, Copy)]
pub enum Protocol {
    Secp256k1 = 1,
    BLS = 3,
}

pub struct FilecoinAddress();

impl FilecoinAddress {
    fn checksum(ingest: &[u8]) -> Vec<u8> {
        digest(ingest, HashSize::Checksum)
    }

    fn address_hash(ingest: &[u8]) -> Vec<u8> {
        digest(ingest, HashSize::Payload)
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
            TypedPublicKey::BLS(pk) => {
                protocol = Protocol::BLS;
                payload = pk.to_bytes();

                cksm = Self::checksum(&[vec![protocol as u8], payload.clone().to_vec()].concat());
            }
            _ => {
                return Err(Error::InvalidCurveType.into());
            }
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
    use tcx_primitive::{Secp256k1PublicKey, TypedPublicKey};

    #[test]
    fn test_bls_address() {
        let test_cases = vec![
            (vec![173, 88, 223, 105, 110, 45, 78, 145, 234, 134, 200, 129, 233, 56,
                186, 78, 168, 27, 57, 94, 18, 121, 123, 132, 185, 207, 49, 75, 149, 70,
                112, 94, 131, 156, 122, 153, 214, 6, 178, 71, 221, 180, 249, 172, 122,
                52, 20, 221],
                "t3vvmn62lofvhjd2ugzca6sof2j2ubwok6cj4xxbfzz4yuxfkgobpihhd2thlanmsh3w2ptld2gqkn2jvlss4a"),
            (vec![179, 41, 79, 10, 46, 41, 224, 198, 110, 188, 35, 93, 47, 237,
                202, 86, 151, 191, 120, 74, 246, 5, 199, 90, 246, 8, 230, 166, 61, 92,
                211, 142, 168, 92, 168, 152, 158, 14, 253, 233, 24, 139, 56, 47,
                147, 114, 70, 13],
                "t3wmuu6crofhqmm3v4enos73okk2l366ck6yc4owxwbdtkmpk42ohkqxfitcpa57pjdcftql4tojda2poeruwa"),
            (vec![150, 161, 163, 228, 234, 122, 20, 212, 153, 133, 230, 97, 178,
                36, 1, 212, 79, 237, 64, 45, 29, 9, 37, 178, 67, 201, 35, 88, 156,
                15, 188, 126, 50, 205, 4, 226, 158, 215, 141, 21, 211, 125, 58, 170,
                63, 230, 218, 51],
                "t3s2q2hzhkpiknjgmf4zq3ejab2rh62qbndueslmsdzervrhapxr7dftie4kpnpdiv2n6tvkr743ndhrsw6d3a"),
            (vec![134, 180, 84, 37, 140, 88, 148, 117, 247, 209, 111, 90, 172, 1,
                138, 121, 246, 193, 22, 157, 32, 252, 51, 146, 29, 216, 181, 206, 28,
                172, 108, 52, 143, 144, 163, 96, 54, 36, 246, 174, 185, 27, 100, 81,
                140, 46, 128, 149],
                "t3q22fijmmlckhl56rn5nkyamkph3mcfu5ed6dheq53c244hfmnq2i7efdma3cj5voxenwiummf2ajlsbxc65a"),
            (vec![167, 114, 107, 3, 128, 34, 247, 90, 56, 70, 23, 88, 83, 96, 206,
                230, 41, 7, 10, 45, 157, 40, 113, 41, 101, 229, 242, 110, 204, 64,
                133, 131, 130, 128, 55, 36, 237, 52, 242, 114, 3, 54, 240, 157, 182,
                49, 240, 116],
                "t3u5zgwa4ael3vuocgc5mfgygo4yuqocrntuuhcklf4xzg5tcaqwbyfabxetwtj4tsam3pbhnwghyhijr5mixa"),
        ];

        let coin_info = CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::BLS,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        for (input, expected) in test_cases {
            let pk = TypedPublicKey::from_slice(CurveType::BLS, &input).unwrap();
            let address = FilecoinAddress::from_public_key(&pk, &coin_info).unwrap();
            assert_eq!(address, expected);
        }
    }

    #[test]
    fn test_secp256k1_address() {
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
