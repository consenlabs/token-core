use crate::transaction::{SignedMessage, UnsignedMessage};
use crate::utils::message_digest;
use crate::Error;
use tcx_chain::{ChainSigner, Keystore, Result, TransactionSigner};
use tcx_constants::CurveType;

impl TransactionSigner<UnsignedMessage, SignedMessage> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &UnsignedMessage,
    ) -> Result<SignedMessage> {
        let cobr_buffer = serde_cbor::to_vec(tx)?;
        let hash = message_digest(&cobr_buffer);

        let account = self.account(symbol, address);

        if account.is_none() {
            return Err(Error::CannotFoundAccount.into());
        }

        let signature;
        match account.unwrap().curve {
            CurveType::BLS => {
                signature = self.sign_hash(&hash, symbol, address, None)?;
            }
            CurveType::SECP256k1 => {
                signature = self.sign_recoverable_hash(&hash, symbol, address, None)?;
            }
            _ => return Err(Error::InvalidCurveType.into()),
        }

        Ok(SignedMessage {
            signature: hex::encode(&signature),
            message: Some(tx.clone()),
        })
    }
}

#[cfg(test)]
mod tests {}
