use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use secp256k1::Secp256k1;
use bitcoin::PrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language};
use std::str::FromStr;
use bitcoin_hashes::hex::{ToHex, FromHex};


use crate::foundation::crypto::encpair::EncPair;
use crate::foundation::crypto::crypto::{Crypto, Pbkdf2Params};
use crate::foundation::utils::token_error::TokenError;

use uuid::Uuid;

pub struct V3MnemonicKeystore {
    id: String,
    version: i32,
    address: String,
    crypto: Crypto<Pbkdf2Params>,
    mnemonic_path: String,
    enc_mnemonic: EncPair,
}

impl V3MnemonicKeystore {
    pub fn new(password: &str, mnemonic: &str, path: &str) -> Result<V3MnemonicKeystore, TokenError> {
        let prv_key = Self::generate_prv_key_from_mnemonic(mnemonic, path)?;
        let crypto : Crypto<Pbkdf2Params> = Crypto::new(password, &prv_key.to_bytes());
        let enc_mnemonic = crypto.derive_enc_pair(password, mnemonic.as_bytes());

        let keystore = V3MnemonicKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 3,
            address: Self::address_from_private_key(&prv_key),
            crypto,
            mnemonic_path: String::from(path),
            enc_mnemonic
        };
        return Ok(keystore);

    }

    fn generate_prv_key_from_mnemonic(mnemonic_str: &str, path: &str) -> Result<PrivateKey, TokenError> {
         if let Ok(mnemonic) = Mnemonic::from_phrase(mnemonic_str, Language::English) {
             let seed = bip39::Seed::new(&mnemonic, &"");
             println!("hex: {}", seed.to_hex());
             let s = Secp256k1::new();
             let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes()).unwrap();

             let path = DerivationPath::from_str(path).unwrap();
             let main_address_pk = sk.derive_priv(&s, &path).unwrap();
             return Ok(main_address_pk.private_key);
         } else {
             return Err(TokenError::from("invalid_mnemonic"));
         }
    }

    fn address_from_private_key(pk: &PrivateKey) -> String {
        let s = Secp256k1::new();
        let pub_key = pk.public_key(&s);
        // Generate pay-to-pubkey-hash address
        let address = Address::p2pkh(&pub_key, Network::Bitcoin);
        return address.to_string();
    }

    pub fn export_private_key(&self, password: &str) -> Result<String, TokenError> {
        let pk_bytes = self.crypto.decrypt(password)?;
        let pk = pk_bytes.to_hex();
        return Ok(pk);
    }
}

fn generate_address_from_wif() {
    let s = Secp256k1::new();
    let prv_key = PrivateKey::from_wif("cT4fTJyLd5RmSZFHnkGmVCzXDKuJLbyTt7cy77ghTTCagzNdPH1j").unwrap();
    let pub_key = prv_key.public_key(&s);
    // Generate pay-to-pubkey-hash address
    let address = Address::p2pkh(&pub_key, Network::Testnet);
    println!("{}", address.to_string());
}



#[cfg(test)]
mod tests {
    use super::*;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";



    #[test]
    pub fn new_v3_mnemonic_keystore() {
        let keystore = V3MnemonicKeystore::new(&PASSWORD, &MNEMONIC, &ETHEREUM_PATH);

        assert!(keystore.is_ok());

        let keystore = keystore.unwrap();
        assert_eq!("16Hp1Ga779iaTe1TxUFDEBqNCGvfh3EHDZ", keystore.address);

    }

}

