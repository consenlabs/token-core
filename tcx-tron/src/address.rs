use bitcoin::util::base58;

use crate::keccak;
use tcx_chain::Address as TraitAddress;
use tcx_chain::Result;
use tcx_constants::CoinInfo;
use tcx_primitive::{
    PrivateKey, PublicKey, Secp256k1PrivateKey, Secp256k1PublicKey, TypedPublicKey,
};

pub struct Address(pub String);

pub enum Error {
    InvalidBase58,
    InvalidEcc,
}

impl TraitAddress for Address {
    fn from_public_key(public_key: &TypedPublicKey, _coin: &CoinInfo) -> Result<String> {
        match public_key {
            TypedPublicKey::Secp256k1(k) => {
                let bytes = k.to_uncompressed();

                let hash = keccak(&bytes[1..]);
                let hex: Vec<u8> = [vec![0x41], hash[12..32].to_vec()].concat();
                Ok(base58::check_encode_slice(&hex))
            }

            _ => Err(Error::InvalidEcc.into()),
        }
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
    use tcx_primitive::{PublicKey, Secp256k1PublicKey};

    #[test]
    fn tron_address() {
        let bytes = hex::decode("04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8").unwrap();
        let _public_key = Secp256k1PublicKey::from_slice(&bytes).unwrap();
        assert_eq!(
            Address::from_public_key(&bytes, None).unwrap(),
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
