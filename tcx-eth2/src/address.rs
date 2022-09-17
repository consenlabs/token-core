use tcx_chain::Address;
use tcx_chain::Result;
use tcx_constants::CoinInfo;
use tcx_primitive::TypedPublicKey;

pub struct Eth2Address();

impl Address for Eth2Address {
    fn from_public_key(public_key: &TypedPublicKey, _coin: &CoinInfo) -> Result<String> {
        let public_key_str = hex::encode(public_key.to_bytes());
        Ok(public_key_str)
    }

    fn is_valid(address: &str, _coin: &CoinInfo) -> bool {
        if address.is_empty() {
            return false;
        };
        if address.starts_with("0x") {
            if address.len() != 98 {
                return false;
            };
        } else {
            if address.len() != 96 {
                return false;
            };
        };
        true
    }
}

#[cfg(test)]
mod test {
    use crate::address::Eth2Address;
    use tcx_chain::Address;
    use tcx_constants::{CoinInfo, CurveType};

    #[test]
    fn is_valid_test() {
        let coin_info = CoinInfo {
            coin: "ETHEREUM2".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::BLS,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        };
        let address = "0xb809eea8f4c1935fa6c0cc6d454a5b297c7856da4df4da96432ea2ef6fa86abf90f8553aaf8c6d605c94244d68d62eb8";
        let valid_result = Eth2Address::is_valid(address, &coin_info);
        assert!(valid_result);

        let address = "b809eea8f4c1935fa6c0cc6d454a5b297c7856da4df4da96432ea2ef6fa86abf90f8553aaf8c6d605c94244d68d62eb8";
        let valid_result = Eth2Address::is_valid(address, &coin_info);
        assert!(valid_result);

        let address = "";
        let valid_result = Eth2Address::is_valid(address, &coin_info);
        assert_eq!(false, valid_result);

        let address = "b809eea8f4c1935fa6c0cc6d454a5b297c7856da4df4da96432ea2ef6fa86abf";
        let valid_result = Eth2Address::is_valid(address, &coin_info);
        assert_eq!(false, valid_result);

        let address = "b809eea8f4c1935fa6c0cc6d454a5b297c7856da4df4da96432ea2ef6fa86abf90f8553aaf8c6d605c94244d68d62eb8b809eea8f4c1935fa6c0cc6d454a5b297c7856da4df4da96432ea2ef6fa86abf";
        let valid_result = Eth2Address::is_valid(address, &coin_info);
        assert_eq!(false, valid_result);
    }
}
