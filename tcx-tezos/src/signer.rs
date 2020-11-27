use crate::transaction::{TezosRawTxIn, TezosTxOut};
use bitcoin::util::base58;
use blake2b_simd::Params;
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

        //Blake2b hash
        let mut params = Params::new();
        params.hash_length(32);
        //add watermark https://gitlab.com/tezos/tezos/-/issues/199
        let mut hash_message: Vec<u8> = vec![0x03];
        hash_message.extend(hex::decode(&raw_data_bytes)?.as_slice());
        let hash_result = params.hash(hash_message.as_slice());
        let sign_result =
            self.sign_recoverable_hash(hash_result.as_bytes(), symbol, address, None)?;

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
