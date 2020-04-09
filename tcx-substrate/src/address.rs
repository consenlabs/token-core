use sp_core::crypto::Ss58AddressFormat;
use sp_core::crypto::Ss58Codec;
use sp_core::sr25519::Public;
use tcx_chain::Address;
use tcx_constants::{CoinInfo, Result};
use tcx_primitive::{PublicKey, Sr25519PublicKey, TypedPublicKey};

pub struct SubstrateAddress();

impl Address for SubstrateAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        // todo: TypedPublicKey to public key
        let sr_pk = Sr25519PublicKey::from_slice(&public_key.to_bytes())?;
        let address = match coin.coin.as_str() {
            "KUSAMA" => sr_pk
                .0
                .to_ss58check_with_version(Ss58AddressFormat::KusamaAccount),
            "POLKADOT" => sr_pk
                .0
                .to_ss58check_with_version(Ss58AddressFormat::PolkadotAccount),
            _ => "".to_owned(),
        };

        Ok(address)
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        match Public::from_ss58check_with_version(address) {
            Ok((_addr, version)) => match coin.network.as_str() {
                "KUSAMA" => version == Ss58AddressFormat::KusamaAccount,
                "POLKADOT" => version == Ss58AddressFormat::PolkadotAccount,
                _ => false,
            },
            Err(_) => false,
        }
    }
}
