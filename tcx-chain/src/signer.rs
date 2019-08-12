//use crate::keystore::V3MnemonicKeystore;

//use crate::keystore::Keystore;
use crate::{HdKeystore, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxSignResult {
    pub signature: String,
    pub tx_hash: String,
    pub wtx_id: String,
}

pub trait TransactionSinger {
    fn sign_transaction(
        &self,
        json_str: &str,
        keystore: &HdKeystore,
        password: &str,
    ) -> Result<String>;
}
