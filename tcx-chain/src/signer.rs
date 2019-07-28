use crate::keystore::V3MnemonicKeystore;
use bitcoin::PrivateKey;
use secp256k1::{Secp256k1, SecretKey, Message};
use std::str::FromStr;
use crate::keystore::Keystore;
use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxSignResult {
    pub signature: String,
    pub tx_hash: String,
    pub wtx_id: String
}


pub trait TransactionSinger {
    fn sign_transaction(json_str: &str) -> Result<String>;
}



pub fn btc_hash_singer(hash: &[u8], password: &str, wallet: &V3MnemonicKeystore) -> Result<String> {
    let pk_str = wallet.export_private_key(password)?;
//    let prv_key = PrivateKey::from_str(pk)?;
    let s = Secp256k1::new();
    let sk = SecretKey::from_str(pk_str.as_str())?;
    let msg = Message::from_slice(hash)?;
    let signature = s.sign(&msg, &sk);
    Ok(signature.to_string())
}


mod tests {
    use crate::keystore::V3MnemonicKeystore;
    use crate::signer::btc_hash_singer;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";


    #[test]
    pub fn test_signer() {
        let wallet = V3MnemonicKeystore::new(PASSWORD, MNEMONIC, ETHEREUM_PATH);

        let w = wallet.unwrap();
        let message = vec![1u8; 32];
//        btc_hash_singer()
        let sign_result = btc_hash_singer(&message, PASSWORD, &w);
        assert!(sign_result.is_ok());
        assert_eq!("304402203577b176ec64e702e4ee61bd69e9a01c3a526d46665e75bad2830966855fa854022001b982bb738a46d8bc426a5319ea264e16f33893864de50c338a11fc5f1e9b3b0000", sign_result.unwrap());

    }

}

