use crate::keystore::{HdKeystore, Account, Address};
use crate::Result;

pub trait Coin {
    fn append_account(&self, keystore: HdKeystore, password: &str) -> Result<Account>;
    fn key(&self) -> String;
    fn derivate_address(&self) -> String;
    fn extended_private_key(&self) -> String;
    fn extended_public_key(&self) -> String;
    fn sign_transaction(json: &str) -> String;
}