use tcx_chain::{Address as AddressTrait, Result};
use tcx_common::{Hash256, Keccak256};

pub struct Address();

impl AddressTrait for Address {
    fn is_valid(address: &str) -> bool {
        unimplemented!()
    }

    fn from_public_key(public_key: &[u8]) -> Result<String> {
        let hash = Keccak256::hash(public_key);

        Ok("2222".to_owned())
    }
}

