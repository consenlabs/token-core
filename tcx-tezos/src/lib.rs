pub mod address;
pub mod signer;
pub mod transaction;
use bitcoin::util::base58;
use tcx_chain::Result;
use tcx_primitive::{Ed25519PrivateKey, PrivateKey, PublicKey};

pub fn build_tezos_base58_private_key(sk: &str) -> Result<String> {
    let edsk_prefix = [43 as u8, 246 as u8, 78 as u8, 7 as u8];
    let mut prefixed_sec_key_vec = vec![];
    prefixed_sec_key_vec.extend(&edsk_prefix);
    let ed25519_private_key =
        Ed25519PrivateKey::from_slice(hex::decode(sk).unwrap().as_slice()).unwrap();
    prefixed_sec_key_vec.extend(&ed25519_private_key.to_bytes());
    prefixed_sec_key_vec.extend(&ed25519_private_key.public_key().to_bytes());
    Ok(base58::check_encode_slice(prefixed_sec_key_vec.as_slice()))
}

pub fn pars_tezos_private_key(private_key: &str) -> Result<Vec<u8>> {
    let data = base58::from_check(private_key)?;
    let pk = Ed25519PrivateKey::from_slice(&data[4..36])?;
    Ok(pk.to_bytes())
}

mod tests {
    use crate::{build_tezos_base58_private_key, pars_tezos_private_key};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_build_tezos_private_key() {
        let base58_prikey = build_tezos_base58_private_key(
            "5740dedadb610333de66ef2db2d91fd648fcbe419dff766f921ae97d536f94ce",
        )
        .unwrap();
        assert_eq!(base58_prikey, "edskRoRrqsGXLTjMwAtzLSx8G7s9ipibZQh6ponFhZYSReSwxwPo7qJCkPJoRjdUhz8Hj7uZhZaFp7F5yftHUYBpJwF2ZY6vAc");
    }

    #[test]
    fn test_pars_tezos_private_key() {
        let tezos_base58_sk = "edskRoRrqsGXLTjMwAtzLSx8G7s9ipibZQh6ponFhZYSReSwxwPo7qJCkPJoRjdUhz8Hj7uZhZaFp7F5yftHUYBpJwF2ZY6vAc";
        let parsing_result = pars_tezos_private_key(tezos_base58_sk).unwrap();
        assert_eq!(
            hex::encode(parsing_result),
            "5740dedadb610333de66ef2db2d91fd648fcbe419dff766f921ae97d536f94ce".to_string()
        );
    }
}
