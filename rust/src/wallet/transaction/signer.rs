use crate::wallet::keystore::v3mnemonic_keystore::V3MnemonicKeystore;
use crate::foundation::utils::token_error::TokenError;
use bitcoin::PrivateKey;
use secp256k1::{Secp256k1, SecretKey, Message};
use std::str::FromStr;

pub fn btc_hash_singer(hash: &[u8], password: &str, wallet: &V3MnemonicKeystore) -> Result<String, TokenError> {
    let pk_str = wallet.export_private_key(password)?;
//    let prv_key = PrivateKey::from_str(pk)?;
    let s = Secp256k1::new();
    let sk = SecretKey::from_str(pk_str.as_str())?;
    let msg = Message::from_slice(hash)?;
    let signature = s.sign(&msg, &sk);
    return Ok(signature.to_string());
//    Secp256k1::sign
}

mod tests {
    use crate::wallet::keystore::v3mnemonic_keystore::V3MnemonicKeystore;
    use crate::wallet::transaction::signer::btc_hash_singer;

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
        assert_eq!("3045022100c2289b6343e703743f82bbb1971436a555a2ae46e2d19262e9e779e1a67afa3c022010cb3b1e35888d0bf226cb4f31a82beffa18a41ffe5d02ea85e12ff462d5161100", sign_result.unwrap());

    }
}