use crate::transaction::{SubstrateTxIn, SubstrateTxOut};
use crate::tx_serializer::ExtrinsicSignature;
use crate::{
    ACCOUNT_INDEX_FLAG, PAYLOAD_HASH_THRESHOLD, SIGNATURE_TYPE_ED25519, SIGNED_EXTRINSIC_V4,
};
use base58::FromBase58;
use codec::Compact;
use codec::Encode;
use failure::format_err;
use sp_core::{blake2_256, Pair};
use sp_keyring::ed25519::Keyring;
use tcx_chain::{ChainSigner, Keystore, TransactionSigner as TraitTransactionSigner};
use tcx_constants::Result;
use tcx_primitive::{PublicKey, Sr25519PublicKey, Ss58Codec};

impl TraitTransactionSigner<SubstrateTxIn, SubstrateTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SubstrateTxIn,
    ) -> Result<SubstrateTxOut> {
        let payload = tx.unsigned_payload()?;
        println!("payload: {}", hex::encode(payload.clone().encode()));
        let hash = if payload.len() > PAYLOAD_HASH_THRESHOLD {
            blake2_256(&payload).to_vec()
        } else {
            payload
        };
        let ed_pair = sp_core::ed25519::Pair::from_seed_slice(
            &hex::decode("febf428d82aec7dc9a3df9031868b2272488e06de5acb5e780ac19c026ae3875")
                .unwrap(),
        )
        .unwrap();
        let sig = ed_pair.sign(&hash).0.to_vec();

        let sig_with_type = [vec![SIGNATURE_TYPE_ED25519], sig].concat();

        let signer = [
            vec![ACCOUNT_INDEX_FLAG],
            FromBase58::from_base58("DvVNVvSoriGZQbMpRgmLPGzCxYM6m9jPVfLxB1krkqfb6bo")
                .map_err(|_| format_err!("parse address error"))?[1..33]
                .to_vec(),
        ]
        .concat();

        // https://github.com/polkadot-js/api/blob/master/packages/types/src/primitive/Extrinsic/Extrinsic.spec.ts#L52
        // println!("signer: {}", hex::encode(signer.clone()));
        // println!("signature: {}", hex::encode(sig_with_type.clone()));
        // println!("era: {}", hex::encode(tx.era_raw()));
        // println!("nonce: {}", hex::encode(Compact::<u32>(tx.nonce).encode()));
        // println!(
        //     "tip: {}",
        //     hex::encode(Compact::<u128>(tx.tip as u128).encode())
        // );
        // println!("method: {}", hex::encode(tx.method_raw()?));

        let signed_bytes = [
            vec![SIGNED_EXTRINSIC_V4],
            signer,
            sig_with_type,
            tx.era_raw(),
            Compact::<u32>(tx.nonce).encode(),
            Compact::<u128>(tx.tip as u128).encode(),
            tx.method_raw()?,
        ]
        .concat();

        let tx_out = SubstrateTxOut {
            signature: hex::encode(signed_bytes.encode()),
        };
        Ok(tx_out)
    }
}
