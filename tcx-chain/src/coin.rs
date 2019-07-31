use crate::keystore::{HdKeystore, Account, Address};
use crate::Result;

pub trait Coin<'z> where Self: std::marker::Sized {

    // used for find_wallet_by_mnemonic in TokenV2
    fn mnemonic_to_account(mnemonic: &str, path: &str) -> Result<Account>;

    fn account(&self) -> &Account;
    fn derive_address(prv_key: &[u8]) -> Result<String>;
    fn load(keystore: &'z HdKeystore) -> Result<Self>;
//    fn load<'a>(keystore: &'a HdKeystore) -> Result<&'a Self>;
    fn append_account(keystore: &'z mut HdKeystore, password: &str, path: &str) -> Result<Self>;
    fn key(&self, password: &str) -> Result<Vec<u8>>;

    fn extended_private_key(&self, password: &str) -> Result<String>;
    fn extended_public_key(&self) -> String;
    fn sign_transaction(&self, json: &str) -> Result<String>;

}