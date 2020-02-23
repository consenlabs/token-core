use crate::transaction::{SubstrateTxIn, SubstrateTxOut};
use crate::tx_serializer::ExtrinsicSignature;
use base58::FromBase58;
use codec::Compact;
use codec::Encode;
use failure::format_err;
use sp_core::{blake2_256, Pair};
use sp_keyring::ed25519::Keyring;
use tcx_chain::{ChainSigner, Keystore, TransactionSigner as TraitTransactionSigner};
use tcx_constants::Result;
use tcx_primitive::{PublicKey, Sr25519PublicKey, Ss58Codec};

pub const SIGNATURE_TYPE_SR25519: u8 = 0x01;

impl TraitTransactionSigner<SubstrateTxIn, SubstrateTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SubstrateTxIn,
    ) -> Result<SubstrateTxOut> {
        //        let mut raw = tx.raw.clone();
        let payload = tx.unsigned_payload()?;
        println!("payload: {}", hex::encode(payload.clone().encode()));
        let hash = if payload.len() > 246 {
            blake2_256(&payload).to_vec()
        } else {
            payload
        };
        let signed = sp_keyring::ed25519::Keyring::Alice.sign(&hash.clone());
        println!(
            "ed25519 pubkey: {}",
            hex::encode(sp_keyring::ed25519::Keyring::Alice.public().to_vec())
        );
        println!("signed by ed25519 {}", hex::encode(signed.0.to_vec()));
        // let pk = sp_core::ed25519::Pair::from_seed_slice(&hex::decode("72b00236c3a74790cf21c06777278eecae5bd18a40573d6c956636df1c23950845ea00e414eeb6379d9fe0a2be7c7b0181aca43fb04034e5ec511764b5d3a597").unwrap()).unwrap();
        let sig = self.sign_recoverable_hash(&hash[..], symbol, address, None)?;
        // let pk = Keyring::Alice;
        // let sig_with_type = sig;
        // let sig = pk.sign(&hash).0.to_vec();
        let sig_with_type = [vec![SIGNATURE_TYPE_SR25519], sig].concat();

        let signer = [
            vec![0xff],
            FromBase58::from_base58(address).map_err(|_| format_err!("parse address error"))?
                [1..33]
                .to_vec(),
        ]
        .concat();

        // let signed = ExtrinsicSignature {
        //     signer,
        //     signature: sig_with_type,
        //     era: tx.era_raw(),
        //     nonce: tx.nonce,
        //     tip: tx.tip as u128
        // };
        // https://github.com/polkadot-js/api/blob/master/packages/types/src/primitive/Extrinsic/Extrinsic.spec.ts#L52
        println!("signer: {}", hex::encode(signer.clone()));
        println!("signature: {}", hex::encode(sig_with_type.clone()));
        println!("era: {}", hex::encode(tx.era_raw()));
        println!("nonce: {}", hex::encode(Compact::<u32>(tx.nonce).encode()));
        println!(
            "tip: {}",
            hex::encode(Compact::<u128>(tx.tip as u128).encode())
        );
        println!("method: {}", hex::encode(tx.method_raw()?));

        let signed_bytes = [
            vec![0x84],
            signer,
            sig_with_type,
            tx.era_raw(),
            Compact::<u32>(tx.nonce).encode(),
            Compact::<u128>(tx.tip as u128).encode(),
            tx.method_raw()?,
        ]
        .concat();

        // let signed_bytes = [
        //     vec![4u8],
        //     tx.method_raw()?,
        //     signer,
        //     sig_with_type.encode(),
        //     tx.era_raw(),
        //     Compact::<u32>(tx.nonce).encode(),
        //     Compact::<u128>(tx.tip as u128).encode(),
        // ].concat();
        // let method_raw = tx.method_raw()?;
        // let tx_out = SubstrateTxOut {
        //     method: hex::encode(method_raw),
        //     signature: hex::encode(signed_bytes)
        // };
        let tx_out = SubstrateTxOut {
            signature: hex::encode(signed_bytes.encode()),
        };
        Ok(tx_out)
    }
}
//
//impl TraitMessageSigner<TronMessageInput, TronMessageOutput> for Keystore {
//    fn sign_message(
//        &mut self,
//        symbol: &str,
//        address: &str,
//        message: &TronMessageInput,
//    ) -> Result<TronMessageOutput> {
//        let data = match message.is_hex {
//            true => {
//                let mut raw_hex: String = message.value.to_owned();
//                if raw_hex.to_uppercase().starts_with("0X") {
//                    raw_hex.replace_range(..2, "")
//                }
//                hex::decode(&raw_hex)?
//            }
//            false => message.value.as_bytes().to_vec(),
//        };
//        let header = match message.is_tron_header {
//            true => "\x19TRON Signed Message:\n32".as_bytes(),
//            false => "\x19Ethereum Signed Message:\n32".as_bytes(),
//        };
//        let to_hash = [header, &data].concat();
//
//        let hash = keccak(&to_hash);
//        let mut sign_result = self.sign_recoverable_hash(&hash[..], symbol, address, None)?;
//        sign_result[64] = sign_result[64] + 27;
//        Ok(TronMessageOutput {
//            signature: hex::encode(sign_result),
//        })
//    }
//}
