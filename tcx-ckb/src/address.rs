use crate::hash::blake2b_160;
use bech32::ToBase32;
use tcx_chain::{Address, Result};
use tcx_constants::CoinInfo;
use tcx_primitive::TypedPublicKey;

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

        Ok(bech32::encode(prefix, buf.to_base32())?)
    }

    fn is_valid(address: &str, _coin: &CoinInfo) -> bool {
        let ret = bech32::decode(address);
        if ret.is_err() {
            return false;
        }

        address.starts_with("ckb") || address.starts_with("ckt")
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
        let coin_info = CoinInfo {
            coin: "CKB".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        let pub_key = TypedPublicKey::from_slice(
            CurveType::SECP256k1,
            &hex::decode("024a501efd328e062c8675f2365970728c859c592beeefd6be8ead3d901330bc01")
                .unwrap(),
        )
        .unwrap();
        let address = CkbAddress::from_public_key(&pub_key, &coin_info).unwrap();
        assert_eq!(address, "ckt1qyqrdsefa43s6m882pcj53m4gdnj4k440axqswmu83");
    }
}
