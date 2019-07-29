use crate::keystore::{HdKeystore, Account, Address};
use crate::Result;

pub trait Coin {
    fn append_account(&self, password: &str) -> Result<Account>;
    fn key(&self, password: &str) -> Result<Vec<u8>>;
    fn derive_address(&self, prv_key: &[u8]) -> Result<String>;
    fn extended_private_key(&self, password: &str) -> Result<String>;
    fn extended_public_key(&self) -> String;
    fn sign_transaction(&self, json: &str) -> Result<String>;
}