use crate::transaction::{TezosRawTxIn, TezosTxOut};
use bitcoin::util::base58;
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
        //tezos ed25519 signature prefix
        let edsig_prefix: [u8; 5] = [9, 245, 205, 134, 18];

        let mut edsig_source_data = vec![];
        edsig_source_data.extend(&edsig_prefix);
        edsig_source_data.extend(sign_result.as_slice());

        let sign_result_hex = hex::encode(sign_result);
        let tx_out = TezosTxOut {
            signature: sign_result_hex.clone(),
            edsig: base58::check_encode_slice(edsig_source_data.as_slice()),
            sbytes: format!("{}{}", tx.raw_data, sign_result_hex),
        };
        Ok(tx_out)
    }
}
