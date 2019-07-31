use tcx_chain::{Address as AddressTrait, Result};
use bitcoin::util::base58;
use tcx_crypto::PublicKey;

pub struct Address(String);

impl AddressTrait for Address {
    fn from_public_key(public_key: &impl PublicKey) -> Result<Address> {
        let bytes = public_key.to_uncompressed();

        let hash = keccak_hash::keccak(
           if bytes.len() == 65 {
               &bytes[1..]
           }  else {
               &bytes
           }
        );

        let hex:Vec<u8> = [vec![0x41], hash[12..32].to_vec()].concat();

        Ok(Address(base58::check_encode_slice(&hex)))
    }

    fn as_string(&self) -> &str {
        unimplemented!()
    }
}

impl PartialEq for Address {

    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn ne(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use tcx_chain::Address as AddressTrait;
    use tcx_crypto::PublicKey;

    #[test]
    fn tron_address() {
        let bytes = hex::decode("04DAAC763B1B3492720E404C53D323BAF29391996F7DD5FA27EF0D12F7D50D694700684A32AD97FF4C09BF9CF0B9D0AC7F0091D9C6CB8BE9BB6A1106DA557285D8").unwrap();
        let public_key = <bitcoin::PublicKey as PublicKey>::from_slice(&bytes).unwrap();

        assert_eq!(Address::from_public_key(&public_key).unwrap().0, "THfuSDVRvSsjNDPFdGjMU19Ha4Kf7acotq");
    }
}


