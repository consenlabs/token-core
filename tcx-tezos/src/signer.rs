use crate::transaction::{TezosRawTxIn, TezosTxOut};
use tcx_chain::{ChainSigner, Keystore, TransactionSigner as TraitTransactionSigner};
use tcx_constants::Result;

impl TraitTransactionSigner<TezosRawTxIn, TezosTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &TezosRawTxIn,
    ) -> Result<TezosTxOut> {
        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };
        let raw_data_bytes = hex::decode(&raw_data_bytes)?;
        let sign_result =
            self.sign_recoverable_hash(raw_data_bytes.as_slice(), symbol, address, None)?;

        let tx_out = TezosTxOut {
            signature: hex::encode(sign_result),
        };
        Ok(tx_out)
    }
}
