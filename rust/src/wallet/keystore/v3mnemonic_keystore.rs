
use std::option;

use bitcoin_hashes::hex::{ToHex, FromHex};

pub struct V3MnemonicKeystore {
    mnemonicPath: String,
}

struct EncryptedMessage {
    enc_str: String,
    nonce: String
}

impl EncryptedMessage {
    fn new(crypto: &Crypto, password: &str, message: &str, nonceOpt: Option<&str>) -> EncryptedMessage {
//        if let Some(n) = nonce {
//
//        }
        let nonce = nonceOpt.unwrap_or(ToHex::to_hex(EncryptedMessage::random_iv()));
        let nonce_bytes: [u8] = FromHex::from_hex(nonce);

    }

}