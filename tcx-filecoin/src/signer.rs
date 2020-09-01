use crate::transaction::{SignedMessage, UnsignedMessage};
use crate::utils::message_digest;
use tcx_chain::{ChainSigner, Keystore, Result, TransactionSigner};

impl TransactionSigner<UnsignedMessage, SignedMessage> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &UnsignedMessage,
    ) -> Result<SignedMessage> {
        let cobr_buffer = serde_cbor::to_vec(tx)?;
        let cid = message_digest(&cobr_buffer);

        let sign = self.sign_recoverable_hash(&cid, symbol, address, None)?;

        Ok(SignedMessage {
            signature: hex::encode(&sign),
            message: Some(tx.clone()),
        })
    }
}

#[cfg(test)]
mod tests {}
