use serde::{Deserialize, Serialize};
use tcx_crypto::{EncPair, Crypto, Pbkdf2Params, aes};
use bip39::{Mnemonic, Language};
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use uuid::Uuid;
use secp256k1::{Secp256k1, Message, All};
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::hex::FromHex;
use bitcoin::network::constants::Network;
use bitcoin::Address;
use std::str::FromStr;
//use crate::errors::{Error, Result};
use crate::Result;
use tcx_chain::{Metadata, Keystore};
use core::result;


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdMnemonicKeystore {
    version: i32,
    pub id: String,
    pub address: String,
    mnemonic_path: String,
    enc_mnemonic: EncPair,
    xpub: String,
    crypto: Crypto<Pbkdf2Params>,
    metadata: Metadata,
}


impl HdMnemonicKeystore {
    pub fn new(metadata: Metadata, password: &str, mnemonic: &str, path: &str) -> Result<HdMnemonicKeystore>{
        let network = match metadata.network.to_lowercase().as_ref() {
            "testnet" => Network::Testnet,
            _ => Network::Bitcoin
        };
        let s = Secp256k1::new();
        let mn = Mnemonic::from_phrase(mnemonic, Language::English).map_err(| _ | format_err!("invalid_mnemonic"))?;
        let p = DerivationPath::from_str(path)?;

        let root_xprv = Self::gen_extend(&mn, network)?;
        let xprv = root_xprv.derive_priv(&s, &p)?;
        let xpub= ExtendedPubKey::from_private(&s, &xprv);


        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, &xprv.to_string().as_bytes());
        let enc_mnemonic = crypto.derive_enc_pair(password, mnemonic.as_bytes());
        let main_address_path = DerivationPath::from_str(&(path.to_owned() + "/0/0"))?;
        let main_addr_prv = root_xprv.derive_priv(&s, &main_address_path)?;
        let main_addr_pub = ExtendedPubKey::from_private(&s, &main_addr_prv);

        let main_addr = Address::p2pkh(&main_addr_pub.public_key, Network::Bitcoin);
        let xpub = Self::generate_enc_xpub(&xpub.to_string());
        Ok(HdMnemonicKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: 44,
            address: main_addr.to_string(),
            crypto,
            mnemonic_path: String::from(path),
            enc_mnemonic,
            metadata,
            xpub,
        })
    }

    fn gen_extend(mnemonic: &Mnemonic, network: Network) -> Result<ExtendedPrivKey> {
        let seed = bip39::Seed::new(mnemonic, &"");
        Ok(ExtendedPrivKey::new_master(network, seed.as_bytes())?)

    }

    fn generate_enc_xpub(xpub: &str) -> String {
        let key = hex!("B888D25EC8C12BD5043777B1AC49F872");
        let iv = hex!("9C0C30889CBCC5E01AB5B2BB88715799");
        aes::cbc::encrypt_pkcs7(&xpub.as_bytes(), &key, &iv).to_hex()
    }

    pub fn export_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Keystore for HdMnemonicKeystore {
    fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn get_address(&self) -> String {
        self.address.clone()
    }

    fn decrypt_cipher_text(&self, password: &str) -> Result<Vec<u8>>  {
        self.crypto.decrypt(password)
    }

    fn export_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn get_id(&self) -> String {
        self.id.to_owned()
    }

    fn clone_box(&self) -> Box<Keystore> {
        Box::new(self.clone()) as Box<Keystore>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Language;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static ETHEREUM_PATH: &'static str = "m/44'/60'/0'/0/0";


    #[test]
    pub fn new_hd_mnemonic_keystore() {
        let meta = Metadata::default();

        let keystore = HdMnemonicKeystore::new(meta, &PASSWORD, &MNEMONIC, &ETHEREUM_PATH);
        println!("{:?}", keystore.unwrap().export_json());
//        assert!((&keystore.is_ok()))
//        assert!(keystore.is_ok());
//
////        let keystore = keystore.unwrap();
//        assert_eq!("16Hp1Ga779iaTe1TxUFDEBqNCGvfh3EHDZ", keystore.unwrap().address);
    }
}