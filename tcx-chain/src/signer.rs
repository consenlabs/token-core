//use crate::keystore::V3MnemonicKeystore;

//use crate::keystore::Keystore;
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxSignResult {
    pub signature: String,
    pub tx_hash: String,
    pub wtx_id: String,
}

impl SignedTransaction for TxSignResult {}
/*
pub trait TransactionSinger {
    fn sign_transaction(
        &self,
        json_str: &str,
        keystore: &HdKeystore,
        password: &str,
    ) -> Result<String>;
}
*/

pub trait Transaction: Sized {}

pub trait SignedTransaction: Sized {}

pub trait TransactionSigner<Input: Transaction, Output: SignedTransaction> {
    fn sign_transaction(&self, tx: &Input, password: Option<&str>) -> Result<Output>;
}
