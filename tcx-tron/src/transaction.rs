use tcx_chain::{TransactionSinger as TransactionSignerTrait, TransactionSinger};

use failure::Error;
use protos::Transaction;

pub struct TransactionBuilder {}

impl TransactionBuilder {
    fn build(&self) -> Transaction {
        Transaction {}
    }

    fn json(value: &str) -> &Self {}
}

pub struct TransactionSigner {}

impl TransactionSignerTrait for TransactionSinger {
    fn sign_transaction(json_str: &str) -> Result<String, Error> {
        unimplemented!()
    }
}
