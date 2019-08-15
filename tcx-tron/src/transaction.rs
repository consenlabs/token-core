use tcx_chain::{TransactionSigner as TraitTransactionSigner, SignedTransaction as TraitSignedTransaction, Transaction as TraitTransaction, HdKeystore};

use failure::Error;
use super::Address;

pub struct Transaction {

}

impl TraitTransaction for Transaction {

}

pub struct SignedTransaction {

}

impl TraitSignedTransaction for SignedTransaction {
}


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
