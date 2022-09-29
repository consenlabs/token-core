use crate::construct_transaction::{
    message_from_instructions, transfer_instruction, Pubkey, Signature, SolanaTransaction,
};
use crate::transaction::{SolanaTxIn, SolanaTxOut};
use bincode::serialize;
use sp_core::bytes::from_hex;
use std::convert::TryFrom;
use tcx_chain::Result;
use tcx_chain::{Keystore, TransactionSigner};
impl TransactionSigner<SolanaTxIn, SolanaTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SolanaTxIn,
    ) -> Result<SolanaTxOut> {
        let from_pubkey = Pubkey(<[u8; 32]>::try_from(from_hex(address).unwrap().as_slice())?);
        let to_pubkey = Pubkey(<[u8; 32]>::try_from(tx.to.as_slice())?);
        let instruction = transfer_instruction(&from_pubkey, &to_pubkey, tx.lamports);
        let message = message_from_instructions(
            &[instruction],
            &from_pubkey,
            <[u8; 32]>::try_from(tx.recent_blockhash.as_slice())?,
        );
        let serialized_message = bincode::serialize(&message).expect("Serialization failed.");
        let sk = self.find_private_key(symbol, address)?;
        let sig = sk.sign(&*serialized_message)?;
        let tx = SolanaTransaction {
            signatures: vec![Signature::new(sig.as_slice())],
            message,
        };
        let serialized_tx = bs58::encode(serialize(&tx).unwrap()).into_string();
        Ok(SolanaTxOut { tx: serialized_tx })
    }
}
