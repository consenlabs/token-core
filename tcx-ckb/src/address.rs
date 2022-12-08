use crate::hash::blake2b_160;
use bech32::{FromBase32, ToBase32, Variant};
use tcx_chain::{Address, Result};
use tcx_constants::CoinInfo;
use tcx_primitive::TypedPublicKey;

// TYPE should be u5
static TYPE_FULL_DATA: u8 = 2u8;
static TYPE_FULL_TYPE: u8 = 4u8;
static TYPE_SHORT: u8 = 1u8;

pub struct CkbAddress();

impl Address for CkbAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        let prefix = match coin.network.as_str() {
            "TESTNET" => "ckt",
            _ => "ckb",
        };

        let pub_key_hash = blake2b_160(public_key.as_secp256k1()?.to_compressed());

        let mut buf = vec![];
        buf.extend(vec![0x1, 0x00]); // append short version for locks with popular codehash and default code hash index
        buf.extend(pub_key_hash);

        Ok(bech32::encode(prefix, buf.to_base32(), Variant::Bech32)?)
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        let ret = bech32::decode(address);
        if ret.is_ok() {
            let (hrp, data, _) = ret.unwrap();
            let data = Vec::from_base32(&data).unwrap();
            let address_type = data[0];

            if !vec![TYPE_FULL_DATA, TYPE_FULL_TYPE, TYPE_SHORT].contains(&address_type) {
                return false;
            }

            if address_type == TYPE_SHORT {
                if data.len() != 22 {
                    // Short address data byte length must be equal to 20
                    return false;
                }

                let code_hash_index = data[1];
                // Short address code hash index must be 00 or 01
                if code_hash_index != 0 && code_hash_index != 1 {
                    return false;
                }
            }
            match hrp.as_str() {
                "ckb" => return coin.network == "MAINNET",
                "ckt" => return coin.network == "TESTNET",
                _ => false,
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::address::CkbAddress;
    use tcx_chain::Address;

    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::TypedPublicKey;

    #[test]
    fn pubkey_to_address() {
        let network_addresses = vec![
            ("TESTNET", "ckt1qyqrdsefa43s6m882pcj53m4gdnj4k440axqswmu83"),
            ("MAINNET", "ckb1qyqrdsefa43s6m882pcj53m4gdnj4k440axqdt9rtd"),
        ];
        for (network, address) in network_addresses {
            let coin_info = CoinInfo {
                coin: "NERVOS".to_string(),
                derivation_path: "".to_string(),
                curve: CurveType::SECP256k1,
                network: network.to_string(),
                seg_wit: "".to_string(),
            };

            let pub_key = TypedPublicKey::from_slice(
                CurveType::SECP256k1,
                &hex::decode("024a501efd328e062c8675f2365970728c859c592beeefd6be8ead3d901330bc01")
                    .unwrap(),
            )
            .unwrap();
            let addr = CkbAddress::from_public_key(&pub_key, &coin_info).unwrap();
            assert_eq!(addr, address);
        }
    }

    #[test]
    fn test_address_is_valid() {
        let valid_addresses = vec![
            ("TESTNET", "ckt1qyqd5eyygtdmwdr7ge736zw6z0ju6wsw7rssu8fcve"),
            ("MAINNET", "ckb1qyqdmeuqrsrnm7e5vnrmruzmsp4m9wacf6vsxasryq"),
        ];
        for (network, address) in valid_addresses {
            let coin_info = CoinInfo {
                coin: "NERVOS".to_string(),
                derivation_path: "".to_string(),
                curve: CurveType::SECP256k1,
                network: network.to_string(),
                seg_wit: "".to_string(),
            };
            assert!(CkbAddress::is_valid(address, &coin_info));
        }

        let invalid_addresses = vec![
            ("MAINNET", "ckb1qyqdmeuqrsrnm7e5vnrmruzmsp4m9wacf6vsxasryg"),
            ("TESTNET", "ckt1qyqrdsefa43s6m882pcj53m4gdnj4k440axqswmu85"),
        ];
        for (address, network) in invalid_addresses {
            let coin_info = CoinInfo {
                coin: "NERVOS".to_string(),
                derivation_path: "".to_string(),
                curve: CurveType::SECP256k1,
                network: network.to_string(),
                seg_wit: "".to_string(),
            };
            assert!(!CkbAddress::is_valid(address, &coin_info));
        }
    }

    #[test]
    fn test_valid_address_data() {
        let invalid_address = vec![
            "ckt1qyzndsefa43s6m882pcj53m4gdnj4k440axqcth0hp",
            "ckt1qyqrdsefa43s6m882pcj53m4gdnj4k440axqqm65l9j",
        ];
        for invalid_address in invalid_address {
            let coin_info = CoinInfo {
                coin: "NERVOS".to_string(),
                derivation_path: "".to_string(),
                curve: CurveType::SECP256k1,
                network: "TESTNET".to_string(),
                seg_wit: "".to_string(),
            };
            assert!(!CkbAddress::is_valid(invalid_address, &coin_info));
        }
    }
}
