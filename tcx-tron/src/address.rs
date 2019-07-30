use tcx_chain::{Address as AddressTrait, Result};
use bitcoin::util::base58;

pub struct Address();

impl AddressTrait for Address {
    fn from_public_key(public_key: &[u8]) -> Result<String> {
        let hash = keccak_hash::keccak(
           if public_key.len() == 65 {
               &public_key[1..]
           }  else {
               public_key
           }
        );

        Ok(base58::check_encode_slice(&[0x41, hash[12..32].to_vec()].concat()))
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use tcx_chain::Address as AddressTrait;

    #[test]
    fn tron_address() {
        let mut public_key = "04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8";
        let mut bytes = hex::decode(public_key).unwrap();

        assert_eq!(Address::from_public_key(&bytes).unwrap(), "THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq");

        public_key = "DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8";
        bytes = hex::decode(public_key).unwrap();

        assert_eq!(Address::from_public_key(&bytes).unwrap(), "THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq");
    }
}


