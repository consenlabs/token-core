use regex::Regex;
use tcx_chain::Address;
use tcx_constants::CoinInfo;
use tcx_constants::Result;
use tcx_primitive::TypedPublicKey;

pub struct EthAddress(Vec<u8>);

fn remove_0x_prefix(hex: &str) -> String {
    let re = Regex::new(r"^0x").unwrap();
    re.replace_all(&hex, "").to_string()
}

impl EthAddress {
    pub fn checksum(address: &str) -> String {
        let address = remove_0x_prefix(address);
        let mut checksum_address = "".to_string();
        let mut address_hash = [0u8; 32];

        let lower_address = address.to_lowercase();
        let without_prefix = lower_address.as_bytes();
        keccak_hash::keccak_256(&without_prefix, &mut address_hash);
        let address_hex = hex::encode(address_hash);

        for i in 0..address.len() {
            let n =
                i64::from_str_radix(&address_hex.chars().nth(i).unwrap().to_string(), 16).unwrap();
            let ch = address.chars().nth(i).unwrap();
            // make char uppercase if ith character is 9..f
            if n > 7 {
                checksum_address = format!("{}{}", checksum_address, ch.to_uppercase().to_string());
            } else {
                checksum_address = format!("{}{}", checksum_address, ch.to_string());
            }
        }

        return checksum_address;
    }
}

impl Address for EthAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        let bytes = public_key.as_secp256k1().unwrap().to_uncompressed();
        let without_prefix = &bytes[1..];
        let mut hashed_bytes = [0u8; 32];
        keccak_hash::keccak_256(&without_prefix, &mut hashed_bytes);
        let add_bytes = &hashed_bytes[(hashed_bytes.len() - 20)..];
        Ok(hex::encode(add_bytes))
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        let address = remove_0x_prefix(address);
        if address.len() != 40 {
            return false;
        }

        let regex = Regex::new(r"^(?-i)[0-9a-f]{40}$").unwrap();
        if regex.is_match(&address) {
            return true;
        } else {
            let regex_with_checksum = Regex::new(r"^(?-i)[0-9a-fA-F]{40}$").unwrap();
            if regex_with_checksum.is_match(&address) {
                return address == EthAddress::checksum(&address.to_lowercase());
            } else {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::address::EthAddress;
    use tcx_chain::Address;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::{
        PrivateKey, PublicKey, Secp256k1PrivateKey, Secp256k1PublicKey, TypedPublicKey,
    };

    fn eth_coin() -> CoinInfo {
        CoinInfo {
            coin: "ETH".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        }
    }

    #[test]
    fn from_public() {
        let eth_coin: CoinInfo = CoinInfo {
            coin: "ETH".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        };

        let pub_key = "506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76";
        let pub_key_bytes = hex::decode(pub_key).unwrap();
        let pk = Secp256k1PrivateKey::from_slice(
            &hex::decode("a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6")
                .unwrap(),
        )
        .unwrap();
        let pub_key = pk.public_key();
        // let pub_key = bitcoin::PublicKey {
        //     compressed: false,
        //     key: secp256k1::PublicKey::from_slice(&pub_key_bytes).unwrap(),
        // };

        // let k1_pub_key = Secp256k1PublicKey(pub_key);
        let typed_pub_key = TypedPublicKey::Secp256k1(pub_key);
        let addr = EthAddress::from_public_key(&typed_pub_key, &eth_coin).unwrap();
        assert_eq!("ef678007d18427e6022059dbc264f27507cd1ffc", addr);
    }

    #[test]
    fn address_valid() {
        let valid_addresses = vec![
            "0xe0FC04FA2d34a66B779fd5CEe748268032a146c0",
            "0xbda6b3d8af264a595ff3582285d0792a0a83ad28",
            "0x818dc3b0346fa4f7ebff232580dc5c8fb174b0da",
            "ef678007d18427e6022059dbc264f27507cd1ffc",
        ];
        for valid_address in valid_addresses {
            assert!(EthAddress::is_valid(valid_address, &eth_coin()));
        }

        let invalid_addresses = vec![
            "0xe0FC04FA2d34a66B779fd5CEe748268032a146C0",
            "0xbda6b3d8af264a595ff3582285d0792a0a83ad2",
        ];
        for invalid_address in invalid_addresses {
            assert!(!EthAddress::is_valid(invalid_address, &eth_coin()));
        }
    }
}
