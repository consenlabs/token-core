use crate::era::Era;
use crate::transaction::ExtrinsicEra;
use crate::{SubstrateAddress, SubstrateTxIn, SubstrateTxOut, ACCOUNT_INDEX_FLAG};
use base58::FromBase58;
use byteorder::{LittleEndian, WriteBytesExt};
use codec::{Compact, Decode, Encode, HasCompact};
use failure::format_err;
use prost::Message;
use sp_core::crypto::Ss58Codec as SpSs58Codec;
use sp_core::sr25519::Public;
use sp_core::{blake2_256, H256};
use std::mem;
use tcx_constants::Result;
use tcx_primitive::{Sr25519PublicKey, Ss58Codec};

#[derive(Debug, PartialEq, Encode, Decode)]
struct SubstrateInnerTx {
    #[codec(compact)]
    nonce: u32,
    #[codec(compact)]
    tip: u128,
    spec_version: u32,
    genesis_hash: H256,
    block_hash: H256,
}

fn hex_to_h256(hex: &str) -> H256 {
    let hash = hex::decode(hex).unwrap();
    let mut array = [0; 32];
    array.clone_from_slice(hash.as_slice());
    H256(array)
}

impl SubstrateTxIn {
    pub fn unsigned_payload(&self) -> Result<Vec<u8>> {
        let method_raw = self.method_raw()?;
        let era_raw = self.era_raw();
        let inner_tx = SubstrateInnerTx {
            nonce: self.nonce,
            tip: self.tip as u128,
            spec_version: self.sepc_version,
            genesis_hash: hex_to_h256(&self.genesis_hash),
            block_hash: hex_to_h256(&self.block_hash),
        };

        Ok([method_raw, era_raw, inner_tx.encode()].concat())
    }

    pub fn method_raw(&self) -> Result<Vec<u8>> {
        let method = match self.method.as_str() {
            "transfer" => hex::decode("0400").map_err(|_| format_err!("expected no error")),
            "transfer_keep_alive" => {
                hex::decode("0603").map_err(|_| format_err!("expected no error"))
            }
            // todo: stack method
            _ => Err(format_err!("unsupported_method")),
        }?;

        let pub_key = Public::from_ss58check(&self.address)
            .map_err(|_| format_err!("invalid address format"))?;

        let account_index_flag = vec![ACCOUNT_INDEX_FLAG];

        let concat_bytes: Vec<u8> = [
            method.clone(),
            account_index_flag,
            pub_key.to_vec(),
            Compact::<u128>(self.amount as u128).encode(),
        ]
        .concat();

        Ok(concat_bytes)
    }

    pub fn era_raw(&self) -> Vec<u8> {
        let extrinsic_era = self.era.as_ref().unwrap();
        let era = Era::mortal(extrinsic_era.period.clone(), extrinsic_era.current.clone());
        era.encode()
        // crate::era::Era::Immortal.encode()
    }
}

impl ExtrinsicEra {
    pub(crate) fn default() -> Self {
        ExtrinsicEra {
            current: 4302222,
            period: 2400,
        }
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ExtrinsicSignature {
    pub signer: Vec<u8>,
    pub signature: Vec<u8>,
    pub era: Vec<u8>,
    #[codec(compact)]
    pub nonce: u32,
    #[codec(compact)]
    pub tip: u128,
}

#[cfg(test)]
mod tests {
    use crate::transaction::ExtrinsicEra;
    use crate::SubstrateTxIn;
    use base58::ToBase58;
    use codec::{Decode, Encode};
    use sp_core::blake2_256;
    use sp_core::crypto::Ss58Codec;
    use sp_keyring::sr25519::Keyring;

    #[test]
    fn serialize_tx() {
        let expected = "0603ff96074594cccf1cd185fa8a72ceaeefd86648f8d45514f3ce33c31bdd07e4655d419ceb580800fb030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f6361fc7493f3c1e9ac758a183839906475f8363aafb1b1d3e910fe16fab4ae1b582";

        let tx_in = SubstrateTxIn {
            method: "transfer_keep_alive".to_string(),
            address: "Fy2rsYCoowQBtuFXqLE65ehAY9T6KWcGiNCQAyPDCkfpm4s".to_owned(),
            amount: 10000,
            era: Some(ExtrinsicEra::default()),
            nonce: 2,
            tip: 0,
            sepc_version: 1019,
            genesis_hash: "e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636"
                .to_string(),
            block_hash: "1fc7493f3c1e9ac758a183839906475f8363aafb1b1d3e910fe16fab4ae1b582"
                .to_string(),
        };
        let payload = tx_in.unsigned_payload().unwrap();
        assert_eq!(expected, hex::encode(payload.clone()));

        let hash = if payload.len() > 256 {
            blake2_256(&payload).to_vec()
        } else {
            payload
        };
        let sig = hex::encode(Keyring::Alice.sign(&hash).0.to_vec());
        assert_eq!(sig, "019880e3de5c0f02dfa0ef76ad725b79c94d26586c9a3d44ec6e10af6a803b4e378fea7eea77242df5e26f3f2ecbea8f36fb11dc84aea7dea934a190ccf6ac3b84")
    }
}
