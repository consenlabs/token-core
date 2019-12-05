use crate::Result;
use serde::{Deserialize, Serialize};

pub trait TransactionSigner<Input, Output> {
    fn sign_transaction(&self, tx: &Input) -> Result<Output>;
}

//pub trait Message: Sized {}
//pub trait SignedMessage: Sized {}
pub trait MessageSigner<Input, Output> {
    fn sign_message(&self, message: &Input) -> Result<Output>;
}
