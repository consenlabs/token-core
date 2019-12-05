use crate::transaction::{TronMessageInput, TronMessageOutput, TronTxInput, TronTxOutput};
use tcx_chain::{
    HdKeystore, MessageSigner as TraitMessageSigner, Result,
    TransactionSigner as TraitTransactionSigner,
};

use bitcoin_hashes::sha256::Hash;
use bitcoin_hashes::Hash as TraitHash;

use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use tcx_primitive::PrivateKey;

use failure::format_err;

use crate::keccak;
use tcx_constants::coin_info::coin_info_from_param;

// http://jsoneditoronline.org/index.html?id=2b86a8503ba641bebed73f32b4ac9c42
//{
//"visible": false,
//"txID": "88817b9c6276e3c535e4f8f15baf546292ca6ad9d44a7d97857bd6f8909d63d4",
//"raw_data": {
//"contract": [
//{
//"parameter": {
//"value": {
//"amount": 100000,
//"owner_address": "415c68cc82c87446f602f019e5fd797437f5b79cc2",
//"to_address": "4156a6076cd1537fa317c2606e4edfa4acd3e8e92e"
//},
//"type_url": "type.googleapis.com/protocol.TransferContract"
//},
//"type": "TransferContract"
//}
//],
//"ref_block_bytes": "02a2",
//"ref_block_hash": "e216e254e43ee108",
//"expiration": 1571898861000,
//"timestamp": 1571898802704
//},
//"raw_data_hex": "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d",
//"chainId": "1",
//"id": "d5ca6979-2586-4b6f-88f2-09a3d8b833b0",
//"password": "123123123",
//"chainType": "TRON"
//}

impl TraitTransactionSigner<TronTxInput, TronTxOutput> for HdKeystore {
    fn sign_transaction(&self, tx: &TronTxInput) -> Result<TronTxOutput> {
        //        let mut raw = tx.raw.clone();
        let coin_info = coin_info_from_param(&"TRON", "", "")?;
        let hash = Hash::hash(&tx.raw_data);
        let account = self
            .account(&coin_info)
            .ok_or_else(|| format_err!("account_not_found"))?;
        let path = &account.derivation_path;
        let sk = &self.get_private_key(path)?;
        let sign_result = sk.sign_recoverable(&hash[..]);

        match sign_result {
            Ok(r) => Ok(TronTxOutput { signature: r }),
            Err(_e) => Err(format_err!("{}", "can not format error")),
        }
    }
}

impl TraitMessageSigner<TronMessageInput, TronMessageOutput> for HdKeystore {
    fn sign_message(&self, message: &TronMessageInput) -> Result<TronMessageOutput> {
        let coin_info = coin_info_from_param(&"TRON", "", "")?;
        let data = match message.is_hex {
            true => {
                let mut raw_hex: String = message.value.to_owned();
                if raw_hex.to_uppercase().starts_with("0X") {
                    raw_hex.replace_range(..2, "")
                }
                hex::decode(&raw_hex)?
            }
            false => message.value.as_bytes().to_vec(),
        };
        let header = match message.is_tron_header {
            true => "\x19TRON Signed Message:\n32".as_bytes(),
            false => "\x19Ethereum Signed Message:\n32".as_bytes(),
        };
        let to_hash = [header, &data].concat();

        let hash = keccak(&to_hash);
        let account = self
            .account(&coin_info)
            .ok_or_else(|| format_err!("account_not_found"))?;
        let path = &account.derivation_path;
        let sk = &self.get_private_key(path)?;
        let mut sign_result = sk.sign_recoverable(&hash[..])?;
        sign_result[64] = sign_result[64] + 27;
        Ok(TronMessageOutput {
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
    use tcx_chain::keystore_guard::KeystoreGuard;
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
            coin: "TRON".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        };
        let mut guard = KeystoreGuard::unlock_by_password(&mut keystore, PASSWORD).unwrap();

        let _ = guard.keystore_mut().derive_coin::<Address>(&coin_info);

        let signed_tx = guard.keystore_mut().sign_transaction(&tx)?;

        assert_eq!(signed_tx.raw["signature"][0].as_str().unwrap(), "beac4045c3ea5136b541a3d5ec2a3e5836d94f28a1371440a01258808612bc161b5417e6f5a342451303cda840f7e21bfaba1011fad5f63538cb8cc132a9768800", "signature must be correct");

        Ok(())
    }

    #[test]
    fn sign_message() {
        let sk =
            Secp256k1PrivateKey::from_wif("L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB")
                .unwrap();
        let message =
            hex_bytes("645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76").unwrap();
        let header = "\x19TRON Signed Message:\n32".as_bytes();
        let to_signed = [header.to_vec(), message].concat();

        let hash = keccak(&to_signed);
        let mut signed = sk.sign_recoverable(&hash).unwrap();
        signed[64] = signed[64] + 27;
        assert_eq!("7209610445e867cf2a36ea301bb5d1fbc3da597fd2ce4bb7fa64796fbf0620a4175e9f841cbf60d12c26737797217c0082fdb3caa8e44079e04ec3f93e86bbea1c", hex::encode(&signed))
    }
}
