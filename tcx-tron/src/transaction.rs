use tcx_chain::{
    HdKeystore, Message as TraitMessage, MessageSigner as TraitMessageSigner, Result,
    SignedMessage as TraitSignedMessage, SignedTransaction as TraitSignedTransaction,
    Transaction as TraitTransaction, TransactionSigner as TraitTransactionSigner,
};

use bitcoin_hashes::sha256::Hash;
use bitcoin_hashes::Hash as TraitHash;

use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use tcx_primitive::Pair;
use tcx_primitive::Secp256k1Pair;

use failure::format_err;

use crate::keccak;
use serde::{Deserialize, Serialize};
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
        tcx_ensure!(password.is_some(), tcx_crypto::Error::PasswordIncorrect);
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    value: String,
    is_hex: bool,
    is_tron_header: bool,
}

impl TraitMessage for Message {}

pub struct SignedMessage {
    pub signature: String,
}

impl TraitSignedMessage for SignedMessage {}

impl TraitMessageSigner<Message, SignedMessage> for HdKeystore {
    fn sign_message(&self, message: &Message, password: Option<&str>) -> Result<SignedMessage> {
        tcx_ensure!(password.is_some(), tcx_crypto::Error::PasswordIncorrect);
        let data = match message.is_hex {
            true => hex::decode(&message.value)?,
            false => message.value.as_bytes().to_vec(),
        };
        let header = match message.is_tron_header {
            true => "\x19TRON Signed Message:\n32".as_bytes(),
            false => "\x19Ethereum Signed Message:\n32".as_bytes(),
        };
        let to_hash = [header, &data].concat();

        let hash = keccak(&to_hash);
        let account = self
            .account(&"TRON")
            .ok_or_else(|| format_err!("account_not_found"))?;
        let path = &account.derivation_path;
        let pair = &self.get_pair::<Secp256k1Pair>(path, password.unwrap())?;
        let mut sign_result = pair.sign_recoverable(&hash[..])?;
        sign_result[64] = sign_result[64] + 27;
        Ok(SignedMessage {
            signature: hex::encode(sign_result),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;
    use bitcoin::util::contracthash::Error::Secp;
    use bitcoin::util::misc::hex_bytes;
    use digest::Digest;
    use serde_json::Value;
    use std::convert::TryFrom;
    use tcx_chain::keystore::EmptyExtra;
    use tcx_chain::{Metadata, TransactionSigner};
    use tcx_constants::CoinInfo;
    use tcx_constants::CurveType;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    fn sign_transaction() -> core::result::Result<(), failure::Error> {
        let json: Value = serde_json::from_str(
            r#" {
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

    #[test]
    fn sign_message() {
        let pair = Secp256k1Pair::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
            .unwrap();
        let message =
            hex_bytes("645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76").unwrap();
        let header = "\x19TRON Signed Message:\n32".as_bytes();
        let to_signed = [header.to_vec(), message].concat();

        let hash = keccak(&to_signed);
        let mut signed = pair.sign_recoverable(&hash).unwrap();
        signed[64] = signed[64] + 27;
        assert_eq!("7209610445e867cf2a36ea301bb5d1fbc3da597fd2ce4bb7fa64796fbf0620a4175e9f841cbf60d12c26737797217c0082fdb3caa8e44079e04ec3f93e86bbea1c", hex::encode(&signed))
    }
}
