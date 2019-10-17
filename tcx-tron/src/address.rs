use bitcoin::util::base58;

use tcx_chain::keystore::Address as TraitAddress;
use tcx_primitive::{Public, Secp256k1PublicKey};

pub struct Address(pub String);

pub enum Error {
    InvalidBase58,
}

impl TraitAddress for Address {
    fn is_valid(_address: &str) -> bool {
        unimplemented!()
    }

    fn from_public_key(public_key: &[u8], _coin: Option<&str>) -> Result<String, failure::Error> {
        let bytes = Secp256k1PublicKey::from_slice(public_key)?.to_uncompressed();
        let hash = keccak_hash::keccak(&bytes[1..]);
        let hex: Vec<u8> = [vec![0x41], hash[12..32].to_vec()].concat();
        Ok(base58::check_encode_slice(&hex))
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use tcx_chain::keystore::Address as TraitAddress;
    use tcx_primitive::Public;
    use tcx_primitive::Secp256k1PublicKey;

    #[test]
    fn tron_address() {
        let bytes = hex::decode("04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8").unwrap();
        let _public_key = Secp256k1PublicKey::from_slice(&bytes).unwrap();
        assert_eq!(
            Address::from_public_key(&bytes, None).unwrap(),
            "THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq"
        );
    }
}
