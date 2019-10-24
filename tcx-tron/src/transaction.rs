use tcx_chain::{
    HdKeystore, Result, SignedTransaction as TraitSignedTransaction,
    Transaction as TraitTransaction, TransactionSigner as TraitTransactionSigner,
};

use bitcoin_hashes::sha256::Hash;
use bitcoin_hashes::Hash as TraitHash;

use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use tcx_primitive::Pair;
use tcx_primitive::Secp256k1Pair;

use failure::format_err;

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
    fn sign_transaction(
        &self,
        tx: &Transaction,
        password: Option<&str>,
    ) -> Result<SignedTransaction> {
        let mut raw = tx.raw.clone();
        tcx_ensure!(password.is_some(), tcx_crypto::Error::InvalidPassword);
        let hash = Hash::hash(&hex::decode(raw["raw_data_hex"].as_str().unwrap())?);
        let account = self
            .account(&"TRON")
            .ok_or_else(|| format_err!("account_not_found"))?;
        let path = &account.derivation_path;
        let pair = &self.get_pair::<Secp256k1Pair>(path, password.unwrap())?;
        let sign_result = pair.sign_recoverable(&hash[..]);

        match sign_result {
            Ok(r) => {
                raw.as_object_mut()
                    .unwrap()
                    .insert("signature".to_owned(), json!([hex::encode(&r)]));

                Ok(SignedTransaction { raw: raw.clone() })
            }
            Err(_e) => Err(format_err!("{}", "can not format error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;
    use serde_json::Value;
    use std::convert::TryFrom;
    use tcx_chain::keystore::EmptyExtra;
    use tcx_chain::{CoinInfo, Metadata, TransactionSigner};
    use tcx_primitive::CurveType;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    fn sign_transaction() -> core::result::Result<(), failure::Error> {
        let json: Value = serde_json::from_str(
            r#" {
    "visible": false,
    "txID": "2a0f45b6aa90dfa563698efb6d30ce5b8415f93ba15381e9a9314afdebcc7496",
    "raw_data": {
      "contract": [
        {
          "parameter": {
            "value": {
              "amount": 1100000,
              "owner_address": "415c68cc82c87446f602f019e5fd797437f5b79cc2",
              "to_address": "4156a6076cd1537fa317c2606e4edfa4acd3e8e92e"
            },
            "type_url": "type.googleapis.com/protocol.TransferContract"
          },
          "type": "TransferContract"
        }
      ],
      "ref_block_bytes": "f64e",
      "ref_block_hash": "ba4593ed82d50347",
      "expiration": 1571889387000,
      "timestamp": 1571889327301
    },
    "raw_data_hex": "0a02f64e2208ba4593ed82d5034740f8aba2dfdf2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18e0914370c5d99edfdf2d"
  } "#,
        )?;

        let tx = Transaction::try_from(json)?;

        let meta = Metadata::default();
        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let coin_info = CoinInfo {
            symbol: "TRON".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
        };
        let _ = keystore.derive_coin::<Address, EmptyExtra>(&coin_info, &PASSWORD);

        let signed_tx = keystore.sign_transaction(&tx, Some(&PASSWORD))?;

        assert_eq!(signed_tx.raw["signature"][0].as_str().unwrap(), "beac4045c3ea5136b541a3d5ec2a3e5836d94f28a1371440a01258808612bc161b5417e6f5a342451303cda840f7e21bfaba1011fad5f63538cb8cc132a9768800", "signature must be correct");

        Ok(())
    }
}
