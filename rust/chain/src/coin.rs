use crate::keystore::{HdKeystore, Account, Address};

trait Coin {
    fn append_account(keystore: HdKeystore) -> Account;
    fn key() -> String;
    fn key_at_path(path: &str, password: &str) -> String;
    fn derivate_address() -> String;
    fn extended_private_key() -> String;
    fn extended_public_key() -> String;
    fn sign_transaction(json: &str) -> String;
}