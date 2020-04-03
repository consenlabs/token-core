use tcx_chain::{Keystore, TransactionSigner};

use crate::address::EthAddress;
use tcx_primitive::{
    Bip32DeterministicPublicKey, Derive, DerivePath, DeterministicPublicKey, FromHex, PrivateKey,
    PublicKey, TypedDeterministicPublicKey,
};

use crate::transaction::{EthTxInput, EthTxOutput};
use tcx_constants::Result;

struct SignatureData {
    v: int32,
    r: Vec<u8>,
    s: Vec<u8>,
}

impl EthTxInput {}

impl TransactionSigner<EthTxInput, EthTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &EthTxInput,
    ) -> Result<EthTxOutput> {
    }
}
