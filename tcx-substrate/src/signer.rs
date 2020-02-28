use crate::transaction::{SubstrateRawTxIn, SubstrateTxIn, SubstrateTxOut};
use crate::tx_serializer::{hash_unsigned_payload, ExtrinsicSignature};
use crate::{
    ACCOUNT_INDEX_FLAG, PAYLOAD_HASH_THRESHOLD, SIGNATURE_TYPE_ED25519, SIGNATURE_TYPE_SR25519,
    SIGNED_EXTRINSIC_V4,
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
        let hash = tx.hash_unsigned_payload()?;

        let sig = self.sign_recoverable_hash(&hash, symbol, address, None)?;

        let sig_with_type = [vec![SIGNATURE_TYPE_SR25519], sig].concat();

        let signer = [
            vec![ACCOUNT_INDEX_FLAG],
            FromBase58::from_base58(address).map_err(|_| format_err!("parse address error"))?
                [1..33]
                .to_vec(),
        ]
        .concat();

        // https://github.com/polkadot-js/api/blob/master/packages/types/src/primitive/Extrinsic/Extrinsic.spec.ts#L52
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

impl TraitTransactionSigner<SubstrateRawTxIn, SubstrateTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SubstrateRawTxIn,
    ) -> Result<SubstrateTxOut> {
        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };
        let raw_data_bytes = hex::decode(&raw_data_bytes)?;
        let hash = hash_unsigned_payload(&raw_data_bytes)?;

        let sig = self.sign_recoverable_hash(&hash, symbol, address, None)?;

        let sig_with_type = [vec![SIGNATURE_TYPE_SR25519], sig].concat();

        let tx_out = SubstrateTxOut {
            signature: format!("0x{}", hex::encode(sig_with_type)),
        };
        Ok(tx_out)
    }
}
