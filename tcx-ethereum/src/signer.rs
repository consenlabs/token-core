use crate::keccak;
use crate::transactions::transaction::Transaction;
use crate::transactions::{EthereumMsgIn, EthereumMsgOut, EthereumTxIn, EthereumTxOut};
use crate::{chain_id_from_network, Error};
use core::convert::TryFrom;
use ethereum_types::{H160, H256, U256, U64};

use tcx_chain::{ChainSigner, Keystore, MessageSigner, Result, TransactionSigner};

impl TryFrom<&EthereumTxIn> for Transaction {
    type Error = Error;

    fn try_from(input: &EthereumTxIn) -> core::result::Result<Self, Self::Error> {
        let nonce = U256::from_dec_str(input.nonce.as_str()).map_err(|_| Error::InvalidNonce)?;
        let to = if input.to.len() > 0 {
            Some(H160::from_slice(
                &hex::decode(input.to.as_str()).map_err(|_| Error::InvalidTo)?,
            ))
        } else {
            None
        };
        let value = U256::from_dec_str(input.value.as_str()).map_err(|_| Error::InvalidValue)?;
        let gas_price =
            U256::from_dec_str(input.gas_price.as_str()).map_err(|_| Error::InvalidGasPrice)?;
        let gas = U256::from_dec_str(input.gas.as_str()).map_err(|_| Error::InvalidGas)?;
        let data = hex::decode(input.data.clone()).map_err(|_| Error::InvalidData)?;
        let transaction_type = if input.transaction_type.len() > 0 {
            Some(U64::from_dec_str(input.transaction_type.as_str()).map_err(|_| Error::InvalidTo)?)
        } else {
            None
        };

        let mut access_list = Vec::new();
        let mut max_priority_fee_per_gas = U256::zero();
        if let Some(t) = transaction_type {
            access_list =
                serde_json::from_str(&input.access_list).map_err(|_| Error::InvalidAccessList)?;
            if t.as_u64() == 2 {
                max_priority_fee_per_gas =
                    U256::from_dec_str(input.max_priority_fee_per_gas.as_str())
                        .map_err(|_| Error::InvalidGas)?;
            }
        }

        Ok(Transaction {
            nonce,
            to,
            value,
            gas_price,
            gas,
            data,
            transaction_type,
            access_list,
            max_priority_fee_per_gas,
        })
    }
}

impl TransactionSigner<EthereumTxIn, EthereumTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &EthereumTxIn,
    ) -> Result<EthereumTxOut> {
        let unsigned_tx = Transaction::try_from(tx)?;

        let account = self.account(symbol, address);

        if account.is_none() {
            return Err(Error::CannotFoundAccount.into());
        }

        let private_key = self
            .find_private_key(&symbol, &address)
            .map_err(|_| Error::CannotGetPrivateKey)?;

        let private_key = H256::from_slice(private_key.to_bytes().as_slice());

        let chain_id = chain_id_from_network(tx.network.as_str())?;

        let signature = hex::encode(unsigned_tx.sign(&private_key, chain_id));
        Ok(EthereumTxOut { signature })
    }
}

impl MessageSigner<EthereumMsgIn, EthereumMsgOut> for Keystore {
    fn sign_message(
        &mut self,
        symbol: &str,
        address: &str,
        message: &EthereumMsgIn,
    ) -> Result<EthereumMsgOut> {
        let mut raw_hex: String = message.value.clone();
        if raw_hex.to_uppercase().starts_with("0X") {
            raw_hex.replace_range(..2, "")
        }
        let data = hex::decode(&raw_hex)?;

        let hash = keccak(&data);
        let mut sign_result = self.sign_recoverable_hash(&hash, symbol, address, None)?;
        sign_result[64] = sign_result[64] + 27;
        Ok(EthereumMsgOut {
            signature: hex::encode(sign_result),
        })
    }
}

#[test]
fn test_sign() {
    let input = EthereumTxIn {
        nonce: "0".to_string(),
        to: "355972B9007c736515523417c96561F63db4e7bC".to_string(),
        value: "1000000000000000".to_string(),
        gas_price: "1000000010".to_string(),
        gas: "100000".to_string(),
        data: "".to_string(),
        network: "RINKEBY".to_string(),
        access_list: "[]".to_string(),
        max_priority_fee_per_gas: "1000000010".to_string(),
        transaction_type: "2".to_string(),
    };
    let raw_tx = Transaction::try_from(&input).unwrap();
    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(
        &hex::decode("2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4").unwrap(),
    );
    let private_key = H256::from_slice(&data);
    let chain_id = chain_id_from_network(input.network.as_str()).unwrap();
    let raw_rlp_bytes = raw_tx.sign(&private_key, chain_id);
    let result = "02f8720480843b9aca0a843b9aca0a830186a094355972b9007c736515523417c96561f63db4e7bc87038d7ea4c6800080c001a0b88af67e8d892c55539eb6bec47704db4b17173b8a8d8bd3a0c59c87319e150aa00baf8b065dde5601cef06f1b7ddf4ebd125c5abad2250315d092a00cc2c4a4c4";
    assert_eq!(result, hex::encode(raw_rlp_bytes));
}

#[test]
fn sign_message() {
    use tcx_primitive::{PrivateKey, Secp256k1PrivateKey};
    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(
        &hex::decode("2ff20a205fad14100db5eedf95903a9a32995dca282f96df2dbb24c8c1bc8586").unwrap(),
    );
    let sk = Secp256k1PrivateKey::from_slice(&data).unwrap();
    let message = "169538".as_bytes();
    let header = "\x19Ethereum Signed Message:\n6".as_bytes();
    let to_signed = [header.to_vec(), message.to_vec()].concat();

    let hash = keccak(&to_signed);
    let mut signed = sk.sign_recoverable(&hash).unwrap();
    signed[64] = signed[64] + 27;
    let result = "4e59b0d97fc748123e52d19d8e792982249d899195cac5b21c8ec6d47aa462f8774b7e5b45966482424e7ca28b92eacafb1a147051282f9d6f12d9b30a669f5c1c";
    assert_eq!(result, hex::encode(signed));
}
