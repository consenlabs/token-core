use std::option;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use secp256k1::Secp256k1;
use bitcoin::PrivateKey;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
//use bitcoin::util::
use bip39::{Mnemonic, Language};
use std::str::FromStr;
use bitcoin_hashes::hex::{ToHex, FromHex};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use crate::foundation::utils;
use crate::foundation::crypto::encpair::EncPair;
use crate::foundation::crypto::crypto::{Crypto, Pbkdf2Params};
use crate::foundation::utils::token_error::TokenError;
use std::fmt::Error;
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
    fn new(password: &str, mnemonic: &str, path: &str) -> Result<V3MnemonicKeystore, TokenError> {
        let prv_key = Self::generate_prv_key_from_mnemonic(mnemonic, path)?;
        let crypto : Crypto<Pbkdf2Params> = Crypto::new(password, &prv_key.to_bytes());
        let enc_mnemonic = crypto.derive_enc_pair(password, mnemonic.as_bytes());

        let keystore = V3MnemonicKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 3,
            address: Self::address_from_private_key(&prv_key),
            crypto: crypto,
            mnemonic_path: String::from(path),
            enc_mnemonic: enc_mnemonic
        };
        return Ok(keystore);

    }

    fn generate_prv_key_from_mnemonic(mnemonic_str: &str, path: &str) -> Result<PrivateKey, TokenError> {
         if let Ok(mnemonic) = Mnemonic::from_phrase(mnemonic_str, Language::English) {
             let seed = bip39::Seed::new(&mnemonic, &"");
             println!("hex: {}", seed.to_hex());
             let s = Secp256k1::new();
             let mut sk = ExtendedPrivKey::new_master(Network::Bitcoin, mnemonic.entropy()).unwrap();

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
        println!("{}", address.to_string());
        return address.to_string();
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


