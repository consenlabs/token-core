use bitcoin::util::base58;
use blake2b_simd::{blake2b, Params};
use tcx_chain::Address;
use tcx_chain::Result;
use tcx_constants::CoinInfo;
use tcx_crypto::hash::dsha256;
use tcx_primitive::TypedPublicKey;

pub struct TezosAddress();

impl Address for TezosAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        let tz1_prefix = hex::decode("06A19F")?;
        //get public key
        let pubkey = public_key.to_bytes();
        //Perform Blake2B hashing on the public key（no prefix）
        let mut params = Params::new();
        params.hash_length(20);
        let generic_hash = params.hash(&pubkey[..32]);
        //sha256Twice(prefix<3> + public key hash<20>)
        let mut prefixed_generic_hash = vec![];
        prefixed_generic_hash.extend_from_slice(tz1_prefix.as_ref());
        prefixed_generic_hash.extend_from_slice(generic_hash.as_bytes());
        let double_hash_result = sha256_hash(&sha256_hash(&prefixed_generic_hash));
        prefixed_generic_hash.extend_from_slice(&double_hash_result[..4]);
        //base58Encode(prefix<3> + public key hash<20> + checksum<4>)
        let address = base58::encode_slice(prefixed_generic_hash.as_slice());

        Ok(address)
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        let decode_result = base58::from(address);
        if decode_result.is_err() {
            return false;
        };

        let decode_data = decode_result.unwrap();
        let hash_res = sha256_hash(&sha256_hash(&decode_data[..decode_data.len() - 4]));
        for number in (0..4) {
            if hash_res[number] != decode_data[decode_data.len() - 4 + number] {
                return false;
            }
        }
        true
    }
}

use ring::digest;

pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let digest_obj = digest::digest(&digest::SHA256, data);
    digest_obj.as_ref().to_vec()
}

#[cfg(test)]
mod test {
    use crate::address::TezosAddress;
    use tcx_chain::Address;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::TypedPublicKey;

    #[test]
    fn from_public_key_test() {
        let coin_info = CoinInfo {
            coin: "TEZOS".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::ED25519,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        };

        let pub_key = TypedPublicKey::from_slice(
            CurveType::ED25519,
            &hex::decode("4a501efd328e062c8675f2365970728c859c592beeefd6be8ead3d901330bc01")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            TezosAddress::from_public_key(&pub_key, &coin_info).unwrap(),
            "tz1dLEU3WfzCrDq2bvoEz4cfLP5wg4S7xNo9"
        );

        let pub_key = TypedPublicKey::from_slice(
            CurveType::ED25519,
            &hex::decode("d0c5ee97112a8a6f192ec44ab10f6a51bbfa327f7736e8e8b30b9ec636bc533b")
                .unwrap(),
        )
        .unwrap();
        //tz1MSaHcwz8vqQKTq9YsxZWfM5PhqFLB2B17
        println!(
            "###->{}",
            TezosAddress::from_public_key(&pub_key, &coin_info).unwrap()
        );
    }

    #[test]
    fn is_valid_test() {
        let coin_info = CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        };
        let address = "tz1dLEU3WfzCrDq2bvoEz4cfLP5wg4S7xNo9";
        let valid_result = TezosAddress::is_valid(address, &coin_info);
        assert!(valid_result);

        let address = "tz1dLEU3WfzCrDq2bvoEz4cfLP5wg4S7xNoI";
        let valid_result = TezosAddress::is_valid(address, &coin_info);
        assert_eq!(false, valid_result);
    }
}
