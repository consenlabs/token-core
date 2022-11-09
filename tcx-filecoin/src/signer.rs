use crate::transaction::{Signature, SignedMessage, UnsignedMessage};
use crate::utils::{digest, HashSize};
use crate::Error;
use forest_address::Address;
use forest_cid::Cid;
use forest_encoding::Cbor;
use forest_message::UnsignedMessage as ForestUnsignedMessage;
use forest_vm::Serialized;
use num_bigint_chainsafe::BigInt;
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

        let account = self.account(symbol, address);
        let signature_type;

        if account.is_none() {
            return Err(Error::CannotFoundAccount.into());
        }

        let signature;
        let mut cid: Cid = unsigned_message.cid()?;
        match account.unwrap().curve {
            CurveType::SECP256k1 => {
                signature_type = 1;
                signature = self.sign_recoverable_hash(
                    &digest(&cid.to_bytes(), HashSize::Default),
                    symbol,
                    address,
                    None,
                )?;

                let forest_sig = forest_crypto::Signature::new_secp256k1(signature.clone());
                let forest_signed_msg = forest_message::SignedMessage {
                    message: unsigned_message,
                    signature: forest_sig,
                };
                cid = forest_signed_msg
                    .cid()
                    .map_err(|_e| format_err!("{}", "forest_message cid error"))?;
            }
            CurveType::BLS => {
                signature_type = 2;
                signature = self.sign_hash(&cid.to_bytes(), symbol, address, None)?;
                cid = unsigned_message.cid()?;
            }
            _ => return Err(Error::InvalidCurveType.into()),
        }

        Ok(SignedMessage {
            cid: cid.to_string(),
            message: Some(tx.clone()),
            signature: Some(Signature {
                r#type: signature_type,
                data: base64::encode(&signature),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilecoinAddress, KeyInfo, UnsignedMessage};
    use tcx_chain::{Keystore, Metadata, TransactionSigner};
    use tcx_constants::{CoinInfo, CurveType};

    #[test]
    fn test_sign_spec256k1() {
        let unsigned_message = UnsignedMessage {
            to: "f1zlkjwo5pnm6petm4u4luj6gb6e64eecrw4t4stq".to_string(),
            from: "f12i3bop43tprlnymx2c75u6uvlq7iur2rcd7qsey".to_string(),
            nonce: 1,
            value: "10000000000000000".to_string(),
            gas_limit: 491585,
            gas_fee_cap: "151367".to_string(),
            gas_premium: "150313".to_string(),
            method: 0,
            params: "".to_string(),
        };

        let key_info =
            KeyInfo::from_lotus(
                &hex::decode("7b2254797065223a22736563703235366b31222c22507269766174654b6579223a222f5059574777574e577a58614d5675437a613958502b314b4a695a4474696f4c76777863754268783041553d227d").unwrap()).unwrap();
        let private_key = key_info.decode_private_key().unwrap();
        let mut ks = Keystore::from_private_key(
            &hex::encode(private_key),
            "Password",
            Metadata::default(),
            "",
        );
        ks.unlock_by_password("Password").unwrap();

        let coin_info = CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        };

        let account = ks
            .derive_coin::<FilecoinAddress>(&coin_info)
            .unwrap()
            .clone();

        let signed_message = ks
            .sign_transaction("FILECOIN", &account.address, &unsigned_message)
            .unwrap();
        let signature = signed_message.signature.unwrap();

        assert_eq!(
            "bafy2bzacec6nqhpi35nwfmdc2two6gs6khs3cgxe7ao2ks6xdwz53qvp2boyu",
            signed_message.cid
        );
        assert_eq!(signature.r#type, 1);
        assert_eq!(signature.data, "MCTI+WjYRozaU/7gYWAwSeOixkSmIHDWHwsU1NVPTrtH4IkXPUrgRcZh4DduJqvHLzoek31LYZxhWkGAzd0j9wA=");
    }

    #[test]
    fn test_sign_bls() {
        let unsigned_message = UnsignedMessage {
            to: "f1zlkjwo5pnm6petm4u4luj6gb6e64eecrw4t4stq".to_string(),
            from: "f3qdyntx5snnwgmjkp2ztd6tf6hhcmurxfj53zylrqyympwvzvbznx6vnvdqloate5eviphnzrkupno4wheesa".to_string(),
            nonce: 1,
            value: "10000000000000000".to_string(),
            gas_limit: 491585,
            gas_fee_cap: "151367".to_string(),
            gas_premium: "150313".to_string(),
            method: 0,
            params: "".to_string()
        };

        let key_info =
            KeyInfo::from_lotus(
                &hex::decode("7b2254797065223a22626c73222c22507269766174654b6579223a2269376b4f2b7a78633651532b7637597967636d555968374d55595352657336616e6967694c684b463830383d227d").unwrap()).unwrap();
        let private_key = key_info.decode_private_key().unwrap();
        let mut ks = Keystore::from_private_key(
            &hex::encode(private_key),
            "Password",
            Metadata::default(),
            "",
        );
        ks.unlock_by_password("Password").unwrap();

        let coin_info = CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::BLS,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        };

        let account = ks
            .derive_coin::<FilecoinAddress>(&coin_info)
            .unwrap()
            .clone();

        let signed_message = ks
            .sign_transaction("FILECOIN", &account.address, &unsigned_message)
            .unwrap();
        let signature = signed_message.signature.unwrap();

        assert_eq!(signature.r#type, 2);
        assert_eq!(
            signed_message.cid,
            "bafy2bzacedbxcjpwgqfkdub732bo5bmtlhudum4fgxdz5ku3e2rziybwm5x5a"
        );
        assert_eq!(signature.data, "tNRsgNdWO6UdY9IOh5tvzcL1Dwi7gljLt22aITKUgtF363lrP2gHxOX9oNGhnFD6BoM4/Y/HMzETlYF0r4+1aHZo1F8fV3XDwxwwz1HKxoDIreXBtPAjTiqBGlTiMwPX");
    }
}
