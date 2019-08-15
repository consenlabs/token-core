use tcx_chain::{
    HdKeystore, Result, SignedTransaction as TraitSignedTransaction,
    Transaction as TraitTransaction, TransactionSigner as TraitTransactionSigner,
};

use super::Address;
use bitcoin_hashes::sha256::Hash;
use bitcoin_hashes::Hash as TraitHash;
use failure::Error;
use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use tcx_primitive::key::secp256k1::Pair;
use tcx_primitive::key::{KeyError, Signer};

use failure::format_err;
use secp256k1::{RecoverableSignature, Signature};
use serde_json::json;

pub struct Transaction {
    raw: Value,
}

impl TryFrom<Value> for Transaction {
    type Error = failure::Error;

    fn try_from(tx: Value) -> Result<Self> {
        Ok(Transaction { raw: tx })
    }
}

impl TraitTransaction for Transaction {}

pub struct SignedTransaction {
    raw: Value,
}

impl TryInto<Value> for SignedTransaction {
    type Error = failure::Error;

    fn try_into(self) -> Result<Value> {
        Ok(self.raw)
    }
}

impl TraitSignedTransaction for SignedTransaction {}

impl TraitTransactionSigner<Transaction, SignedTransaction> for HdKeystore {
    fn sign_transaction(&self, tx: Transaction, password: &str) -> Result<SignedTransaction> {
        unimplemented!()
    }
}

impl TraitTransactionSigner<Transaction, SignedTransaction> for Pair {
    fn sign_transaction(&self, tx: Transaction, password: &str) -> Result<SignedTransaction> {
        let mut raw = tx.raw;

        let hash = Hash::hash(&hex::decode(raw["raw_data_hex"].as_str().unwrap())?);

        let sign_result: core::result::Result<RecoverableSignature, KeyError> =
            self.sign(&hash[..]);

        match sign_result {
            Ok(r) => {
                let (recover_id, sign) = r.serialize_compact();
                let mut bs = bytebuffer::ByteBuffer::new();
                bs.write_bytes(&sign);
                bs.write_u8(recover_id.to_i32() as u8);

                raw.as_object_mut()
                    .unwrap()
                    .insert("signature".to_owned(), json!([hex::encode(&bs.to_bytes())]));

                Ok(SignedTransaction { raw })
            }
            Err(e) => Err(format_err!("{}", "can not format error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::convert::TryFrom;
    use tcx_chain::TransactionSigner;
    use tcx_primitive::key::secp256k1::Pair;
    use tcx_primitive::key::Pair as TraitPair;

    #[test]
    fn sign_transaction() -> core::result::Result<(), failure::Error> {
        let json: Value = serde_json::from_str( r#" {
            "visible": false,
            "txID": "dc74fc99076e7638067753c5c9c3aa61f9ce208707ef6940e4ab8a4944b5d69f",
            "raw_data": {
            "contract": [
                {
                    "parameter": {
                    "value": {
                        "amount": 100,
                        "owner_address": "41a1e81654258bf14f63feb2e8d1380075d45b0dac",
                        "to_address": "410b3e84ec677b3e63c99affcadb91a6b4e086798f"
                    },
                    "type_url": "type.googleapis.com/protocol.TransferContract"
                },
                    "type": "TransferContract"
                }
            ],
            "ref_block_bytes": "0831",
            "ref_block_hash": "b02efdc02638b61e",
            "expiration": 1565866902000,
            "timestamp": 1565866844064
        },
            "raw_data_hex": "0a0208312208b02efdc02638b61e40f083c3a7c92d5a65080112610a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412300a1541a1e81654258bf14f63feb2e8d1380075d45b0dac1215410b3e84ec677b3e63c99affcadb91a6b4e086798f186470a0bfbfa7c92d"
        } "#)?;

        let tx = Transaction::try_from(json)?;

        let pair = Pair::from_slice(&hex::decode(
            "1111111111111311111111111111111111111111111111111111111111111111",
        )?)
        .map_err(|_| format_err!("{}", "can not sign"))?;
        let signed_tx = pair.sign_transaction(tx, "")?;

        assert_eq!("2c6656633c2ac6663339d33f0ca3af3849c718b79534428a30095b0ab4a4df5f11636c49e8f0969586014267270b7a347da18764da45b9754b2ca98e093f735e00", signed_tx.raw["signature"][0].as_str().unwrap(), "signature must be correct");

        Ok(())
    }
}
