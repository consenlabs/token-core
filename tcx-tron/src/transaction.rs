use tcx_chain::{
    HdKeystore, SignedTransaction as TraitSignedTransaction, Transaction as TraitTransaction,
    TransactionSigner as TraitTransactionSigner,
};

use super::Address;
use failure::Error;

pub struct Transaction {}

impl TraitTransaction for Transaction {}

pub struct SignedTransaction {}

impl TraitSignedTransaction for SignedTransaction {}

impl TraitTransactionSigner<Transaction, SignedTransaction> for HdKeystore {
    fn sign(&self, tx: &Transaction) -> SignedTransaction {
        SignedTransaction {}
    }
}

/*
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
*/
