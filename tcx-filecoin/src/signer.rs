use tcx_chain::{Keystore, Result, TransactionSigner};

use crate::hash::new_blake2b;
use crate::serializer::Serializer;
use crate::{hex_to_bytes, Error};
use std::collections::HashMap;
use tcx_chain::ChainSigner;
use crate::transaction::{SignedMessageOutput, MessageInput};


pub struct FilecoinMessageSigner<'a> {
    ks: &'a mut dyn ChainSigner,
    symbol: &'a str,
    address: &'a str,
}

impl<'a> FilecoinMessageSinger<'a> {
    fn sign_message(&mut self, symbol: &str, address: &str, input: &MessageInput) {
        self.ks.sign_recoverable_hash();
    }
}

impl TransactionSigner<MssageInput, SignedMessageOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &MessageInput,
    ) {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_message_id() {

    }


}


