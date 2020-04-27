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
