use crate::transaction::{SubstrateRawTxIn, SubstrateTxOut};
use crate::{PAYLOAD_HASH_THRESHOLD, SIGNATURE_TYPE_SR25519};
use sp_core::blake2_256;

use tcx_chain::{ChainSigner, Keystore, TransactionSigner as TraitTransactionSigner};
use tcx_constants::Result;

pub(crate) fn hash_unsigned_payload(payload: &[u8]) -> Result<Vec<u8>> {
    if payload.len() > PAYLOAD_HASH_THRESHOLD {
        Ok(blake2_256(&payload).to_vec())
    } else {
        Ok(payload.to_vec())
    }
}

impl TraitTransactionSigner<SubstrateRawTxIn, SubstrateTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SubstrateRawTxIn,
    ) -> Result<SubstrateTxOut> {
        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };
        let raw_data_bytes = hex::decode(&raw_data_bytes)?;
        let hash = hash_unsigned_payload(&raw_data_bytes)?;

        let sig = self.sign_recoverable_hash(&hash, symbol, address, None)?;

        let sig_with_type = [vec![SIGNATURE_TYPE_SR25519], sig].concat();

        let tx_out = SubstrateTxOut {
            signature: format!("0x{}", hex::encode(sig_with_type)),
        };
        Ok(tx_out)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_payload_hash() {
        let test_cases = vec![
            ("imToken", "696d546f6b656e"),
            ("super long sentence: 0x891D85380A227e5a8443bd0f39bDedBB6DA798830x891D85380A227e5a8443bd0f39bDedBB6DA798830x891D85380A227e5a8443bd0f39bDedBB6DA798830x891D85380A227e5a8443bd0f39bDedBB6DA798830x891D85380A227e5a8443bd0f39bDedBB6DA79883", "7375706572206c6f6e672073656e74656e63653a20307838393144383533383041323237653561383434336264306633396244656442423644413739383833307838393144383533383041323237653561383434336264306633396244656442423644413739383833307838393144383533383041323237653561383434336264306633396244656442423644413739383833307838393144383533383041323237653561383434336264306633396244656442423644413739383833307838393144383533383041323237653561383434336264306633396244656442423644413739383833")
         ];
        for case in test_cases {
            assert_eq!(
                case.1,
                hex::encode(hash_unsigned_payload(case.0.as_bytes()).unwrap())
            );
        }
    }

    #[test]
    fn test_sign() {
        // check the integration test
    }
}
