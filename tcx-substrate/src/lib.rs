mod address;
mod era;
mod signer;
mod transaction;
mod tx_serializer;

pub use address::SubstrateAddress;
pub use transaction::{ExtrinsicEra, SubstrateRawTxIn, SubstrateTxIn, SubstrateTxOut};

pub(crate) const ACCOUNT_INDEX_FLAG: u8 = 0xff;
pub(crate) const SIGNATURE_TYPE_ED25519: u8 = 0x00;
pub(crate) const SIGNATURE_TYPE_SR25519: u8 = 0x01;
pub(crate) const SIGNED_EXTRINSIC_V4: u8 = 0x84;
pub(crate) const PAYLOAD_HASH_THRESHOLD: usize = 256;

#[cfg(test)]
mod tests {

    use codec::HasCompact;
    use codec::{Decode, Encode};

    use sp_core::H256;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Test1CompactHasCompact<T: HasCompact> {
        #[codec(compact)]
        bar: T,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PolkadotTx {
        //        #[codec(compact)]
        method: Vec<u8>,
        era_period: u8,
        era_phase: u8,
        #[codec(compact)]
        nonce: u32,
        #[codec(compact)]
        tip: u128,
        spec_version: u32,
        genesis_hash: H256,
        block_hash: H256,
    }
    //
    //    #[test]
    //    fn test_generate_address() {
    //        let key = Sr25519Keyring::Alice;
    //        assert_eq!(
    //            format!("{}", hex::encode(key.to_raw_public_vec())),
    //            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    //        );
    //
    //        let acc_id = key.to_account_id();
    //        assert_eq!(
    //            format!("{}", acc_id.to_ss58check()),
    //            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    //        );
    //    }
    //
    //    #[test]
    //    fn test_compact() {
    //        let amount = Test1CompactHasCompact { bar: 2u128 }.encode();
    //        assert_eq!(format!("{}", hex::encode(amount)), "419c");
    //    }
    //
    //    fn hex_to_h256(hex: &str) -> H256 {
    //        let hash = hex::decode(hex).unwrap();
    //        let mut array = [0; 32];
    //        array.clone_from_slice(hash.as_slice());
    //        H256(array)
    //    }
    //
    //    #[test]
    //    fn test_serialize_tx() {
    //        let method = hex::decode(
    //            "0603ff96074594cccf1cd185fa8a72ceaeefd86648f8d45514f3ce33c31bdd07e4655d419c",
    //        )
    //        .unwrap();
    //        let era_period = 0xebu8;
    //        let era_phase = 0x58u8;
    //        let nonce = 0x2u32;
    //        let tip = 0x0u128;
    //        let spec_version = 0x000003fbu32;
    //        let genesis_hash =
    //            hex_to_h256("e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636");
    //        let block_hash =
    //            hex_to_h256("1fc7493f3c1e9ac758a183839906475f8363aafb1b1d3e910fe16fab4ae1b582");
    //        let tx = PolkadotTx {
    //            method,
    //            era_period,
    //            era_phase,
    //            nonce,
    //            tip,
    //            spec_version,
    //            genesis_hash,
    //            block_hash,
    //        };
    //        assert_eq!("940603ff96074594cccf1cd185fa8a72ceaeefd86648f8d45514f3ce33c31bdd07e4655d419ceb580800fb030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f6361fc7493f3c1e9ac758a183839906475f8363aafb1b1d3e910fe16fab4ae1b582", hex::encode(tx.encode()));
    //    }
    //
    //    #[test]
    //    fn test_sign_with_alice() {
    //        let msg = hex::decode("0603ff96074594cccf1cd185fa8a72ceaeefd86648f8d45514f3ce33c31bdd07e4655d419ceb580800fb030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f6361fc7493f3c1e9ac758a183839906475f8363aafb1b1d3e910fe16fab4ae1b582").unwrap();
    //        let hash = if msg.len() > 256 {
    //            blake2_256(&msg).to_vec()
    //        } else {
    //            msg
    //        };
    //
    //        assert_eq!("0603ff96074594cccf1cd185fa8a72ceaeefd86648f8d45514f3ce33c31bdd07e4655d419ceb580800fb030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f6361fc7493f3c1e9ac758a183839906475f8363aafb1b1d3e910fe16fab4ae1b582", hex::encode(hash.clone()));
    //        let signed = Keyring::Alice.sign(&hash);
    //        assert_eq!("", hex::encode(signed));
    //    }
}
