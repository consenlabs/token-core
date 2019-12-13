use bitcoin::util::base58;

use crate::keccak;

use tcx_chain::Address as TraitAddress;
use tcx_chain::Result;
use tcx_constants::CoinInfo;
use tcx_primitive::TypedPublicKey;

pub struct Address(pub String);

impl TraitAddress for Address {
    fn from_public_key(public_key: &TypedPublicKey, _coin: &CoinInfo) -> Result<String> {
        let pk = public_key.as_secp256k1()?;
        let bytes = pk.to_uncompressed();

        let hash = keccak(&bytes[1..]);
        let hex: Vec<u8> = [vec![0x41], hash[12..32].to_vec()].concat();
        Ok(base58::check_encode_slice(&hex))
    }

    fn is_valid(address: &str) -> bool {
        let decode_ret = base58::from_check(address);
        if let Ok(data) = decode_ret {
            data.len() == 21 && data[0] == 0x41
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use tcx_chain::Address as TraitAddress;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::TypedPublicKey;

    #[test]
    fn tron_address() {
        let bytes = hex::decode("04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8").unwrap();
        let coin_info = CoinInfo {
            coin: "".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        };

        assert_eq!(
            Address::from_public_key(
                &TypedPublicKey::from_slice(CurveType::SECP256k1, &bytes).unwrap(),
                &coin_info
            )
            .unwrap(),
            "THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq"
        );
    }

    #[test]
    fn tron_address_validation() {
        assert!(Address::is_valid("THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq"));
        assert!(!Address::is_valid("THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acot"));
        assert!(!Address::is_valid(
            "qq9j7zsvxxl7qsrtpnxp8q0ahcc3j3k6mss7mnlrj8"
        ));
        assert!(!Address::is_valid("mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN"));
    }
}
