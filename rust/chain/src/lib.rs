pub mod signer;
pub mod keystore;

pub use keystore::V3MnemonicKeystore;

pub use signer::{TxSignResult, TransactionSinger};
pub use keystore::{Metadata, Keystore, Source};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
