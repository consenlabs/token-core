//use crate::transaction::{TezosRawTxIn, TezosTxOut};
//use tcx_chain::{Keystore, ChainSigner};
//use tcx_constants::Result;
//
////pub(crate) fn hash_unsigned_payload(payload: &[u8]) -> Result<Vec<u8>> {
////    if payload.len() > PAYLOAD_HASH_THRESHOLD {
////        Ok(blake2_256(&payload).to_vec())
////    } else {
////        Ok(payload.to_vec())
////    }
////}
//
//
//impl TraitTransactionSigner<TezosRawTxIn, TezosTxOut> for Keystore {
//    fn sign_transaction(
//        &mut self,
//        symbol: &str,
//        address: &str,
//        tx: &TezosRawTxIn,
//    ) -> Result<TezosTxOut> {
//        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
//            tx.raw_data[2..].to_string()
//        } else {
//            tx.raw_data.clone()
//        };
//        let raw_data_bytes = hex::decode(&raw_data_bytes)?;
////        let hash = hash_unsigned_payload(&raw_data_bytes)?;//TODO
//        let sign_result = self.sign_recoverable_hash(raw_data_bytes.as_slice())?;
//
//        let tx_out = TezosTxOut {
//            signature: format!("0x{}", hex::encode(sign_result)),
//        };
//        Ok(tx_out)
//    }
//}
