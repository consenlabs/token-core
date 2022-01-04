// https://www.freecodecamp.org/news/how-to-create-an-ethereum-wallet-address-from-a-private-key-ae72b0eee27b/
use crypto::{digest::Digest as cDigest, sha3::Sha3};
use sha3::Digest;
use tcx_chain::Address;
use tcx_constants::{CoinInfo, Result};
use tcx_primitive::TypedPublicKey;
pub struct EthereumAddress();

impl Address for EthereumAddress {
    fn from_public_key(public_key: &TypedPublicKey, _coin: &CoinInfo) -> Result<String> {
        let pk = public_key.as_secp256k1()?;
        let bytes = pk.to_uncompressed();

        let hash = sha3::Keccak256::digest(&bytes[1..]).to_vec().split_off(12);

        let address_lower = hex::encode(hash.clone());
        let address = to_checksum(address_lower.as_str());
        Ok(address)
    }

    fn is_valid(address: &str, _coin: &CoinInfo) -> bool {
        if address.is_empty() {
            return false;
        };
        if address.starts_with("0x") {
            if address.len() != 22 {
                return false;
            };
        } else {
            if address.len() != 20 {
                return false;
            };
        };
        true
    }
}

fn to_checksum(address: &str) -> String {
    // https://github.com/miguelmota/rust-eth-checksum/blob/master/src/lib.rs
    let address = address.trim_start_matches("0x").to_lowercase();

    let address_hash = {
        let mut hasher = Sha3::keccak256();
        hasher.input(address.as_bytes());
        hasher.result_str()
    };
    address
        .char_indices()
        .fold(String::from("0x"), |mut acc, (index, address_char)| {
            let n = u16::from_str_radix(&address_hash[index..index + 1], 16).unwrap();

            if n > 7 {
                acc.push_str(&address_char.to_uppercase().to_string())
            } else {
                acc.push(address_char)
            }

            acc
        })
}

#[test]
fn test_change() {
    use tcx_constants::CurveType;
    let bytes = hex::decode("04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8").unwrap();
    let coin_info = CoinInfo {
        coin: "".to_string(),
        derivation_path: "".to_string(),
        curve: CurveType::SECP256k1,
        network: "".to_string(),
        seg_wit: "".to_string(),
    };
    let res = EthereumAddress::from_public_key(
        &TypedPublicKey::from_slice(CurveType::SECP256k1, &bytes).unwrap(),
        &coin_info,
    );
    assert_eq!(
        res.unwrap(),
        String::from("0x547b45770EE4401494c9157e8263E7a133cbD88d")
    );
}
