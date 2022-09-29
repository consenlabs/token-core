mod address;
mod construct_transaction;
mod signer;
mod transaction;

pub use crate::address::SolanaAddress;
pub use crate::transaction::{SolanaTxIn, SolanaTxOut};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
