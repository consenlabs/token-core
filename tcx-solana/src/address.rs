use sp_core::bytes::to_hex;
use tcx_chain::Address;
use tcx_constants::{CoinInfo, Result};
use tcx_primitive::{Ed25519PublicKey, PublicKey, TypedPublicKey};

pub struct SolanaAddress(String);

impl Address for SolanaAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        let address = to_hex(public_key.to_bytes().as_slice(), false);
        Ok(address)
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
       match Ed25519PublicKey::from_slice(address.as_bytes()){
            Ok(..)=>true,
            _ =>false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_valid() {

        assert_eq!(2 + 2, 4);
    }
}