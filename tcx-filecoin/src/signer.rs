use crate::transaction::{SignedMessage, UnsignedMessage, Signature};
use crate::utils::message_digest;
use crate::Error;
use forest_address::Address;
use forest_message::UnsignedMessage as ForestUnsignedMessage;
use forest_vm::Serialized;
use num_bigint_chainsafe::BigInt;
use serde_cbor::to_vec;
use std::convert::TryFrom;
use std::str::FromStr;
use tcx_chain::{ChainSigner, Keystore, Result, TransactionSigner};
use tcx_constants::CurveType;

impl TryFrom<&UnsignedMessage> for ForestUnsignedMessage {
    type Error = crate::Error;

    fn try_from(
        message: &UnsignedMessage,
    ) -> core::result::Result<ForestUnsignedMessage, Self::Error> {
        let to = Address::from_str(&message.to).map_err(|_| Error::InvalidAddress)?;
        let from = Address::from_str(&message.from).map_err(|_| Error::InvalidAddress)?;
        let value = BigInt::from_str(&message.value).map_err(|_| Error::InvalidNumber)?;
        let gas_limit = message.gas_limit;
        let gas_fee_cap =
            BigInt::from_str(&message.gas_fee_cap).map_err(|_| Error::InvalidNumber)?;
        let gas_premium =
            BigInt::from_str(&message.gas_premium).map_err(|_| Error::InvalidNumber)?;

        let message_params_bytes =
            base64::decode(&message.params).map_err(|_| Error::InvalidParam)?;
        let params = Serialized::new(message_params_bytes);

        let tmp = ForestUnsignedMessage::builder()
            .to(to)
            .from(from)
            .sequence(message.nonce)
            .value(value)
            .method_num(message.method)
            .params(params)
            .gas_limit(gas_limit)
            .gas_premium(gas_premium)
            .gas_fee_cap(gas_fee_cap)
            .build()
            .map_err(|_| Error::InvalidFormat)?;

        Ok(tmp)
    }
}

impl TransactionSigner<UnsignedMessage, SignedMessage> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &UnsignedMessage,
    ) -> Result<SignedMessage> {
        let unsigned_message = forest_message::UnsignedMessage::try_from(tx)?;

        let cbor_buffer = to_vec(&unsigned_message)?;
        let cid = message_digest(&cbor_buffer);
        let account = self.account(symbol, address);
        let signature_type ;

        if account.is_none() {
            return Err(Error::CannotFoundAccount.into());
        }

        let signature;
        match account.unwrap().curve {
            CurveType::SECP256k1 => {
                signature_type = 1;
                signature = self.sign_recoverable_hash(&cid, symbol, address, None)?;
            }
            CurveType::BLS => {
                signature_type = 2;
                signature = self.sign_hash(&cbor_buffer, symbol, address, None)?;
            }
            _ => return Err(Error::InvalidCurveType.into()),
        }

        Ok(SignedMessage {
            cid: base64::encode(&cid),
            message: Some(tx.clone()),
            signature: Some(Signature{ r#type: signature_type, data: base64::encode(&signature) }),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilecoinAddress, UnsignedMessage};
    use tcx_chain::{Keystore, Metadata, TransactionSigner};
    use tcx_constants::{CoinInfo, CurveType};

    #[test]
    fn test_sign_secp256k1() {
        let unsigned_message = UnsignedMessage {
            to: "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy".to_string(),
            from: "t1d2xrzcslx7xlbbylc5c3d5lvandqw4iwl6epxba".to_string(),
            nonce: 1,
            value: "100000".to_string(),
            gas_limit: 1,
            gas_fee_cap: "1".to_string(),
            gas_premium: "1".to_string(),
            method: 0,
            params: "".to_string(),
        };

        let mut ks = Keystore::from_private_key(
            "f15716d3b003b304b8055d9cc62e6b9c869d56cc930c3858d4d7c31f5f53f14a",
            "Password",
            Metadata::default(),
        );
        ks.unlock_by_password("Password").unwrap();

        let coin_info = CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        let account = ks
            .derive_coin::<FilecoinAddress>(&coin_info)
            .unwrap()
            .clone();

        let signed_message = ks.sign_transaction("FILECOIN", &account.address, &unsigned_message);
        let signature = signed_message.unwrap().signature.unwrap();

        assert_eq!(signature.r#type,1);
        assert_eq!(signature.data, "aTqtx4X5mjv53sgVCG8FRtt6jNcbnmCNQL28mHFgRooa024ARS6LmXXp9PJk0qIwEt52Xz1GV9yTN+SVSaqF2QE=");
    }

    #[test]
    fn test_sign_bls() {
        let unsigned_message = UnsignedMessage {
            to: "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy".to_string(),
            from: "t3vxrizeiel2e2bxg3jhk62dlcutyc26fjnw6ua2sptu32dtjpwxbjawg666nqdngrkvvn45h7yb4qiya6ls7q".to_string(),
            nonce: 1,
            value: "100000".to_string(),
            gas_limit: 25000,
            gas_fee_cap: "2500".to_string(),
            gas_premium: "2500".to_string(),
            method: 0,
            params: "".to_string(),
        };

        let mut ks = Keystore::from_private_key(
            "d31ed8d06197f7631e58117d99c5ae4791183f17b6772eb4afc5c840e0f7d412",
            "Password",
            Metadata::default(),
        );
        ks.unlock_by_password("Password").unwrap();

        let coin_info = CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::BLS,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        let account = ks
            .derive_coin::<FilecoinAddress>(&coin_info)
            .unwrap()
            .clone();

        let signed_message = ks.sign_transaction("FILECOIN", &account.address, &unsigned_message);
        let signature = signed_message.unwrap().signature.unwrap();

        assert_eq!(signature.r#type,2);
        assert_eq!(signature.data, "oOg4CXfSzMXdTV69gjQG7SL96ICjvQ+oQmwWs0ATxIfFEQf14oCAMbaA/yAKoW93ChKoICLMD9KnsDArqs7oeGL+0Rvgh2CdOw2vkIaVWFdOFblLN1oPNLzpR46XW7As");
    }
}
