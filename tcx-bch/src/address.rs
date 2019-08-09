use tcx_chain::curve::{Secp256k1Curve, PublicKey, Secp256k1PublicKey, CurveType};
use tcx_chain::{Coin, HdKeystore, Account};
use crate::Result;
use bitcoin::network::constants::Network;
use bitcoin::{Address as BtcAddress, PublicKey as BtcPublicKey, PrivateKey};
use tcx_chain::keystore::{KeyType, Address, Extra, CoinInfo};
use secp256k1::{SecretKey, Secp256k1};
use bitcoin_hashes::hex::ToHex;
use serde_json::Value;
use crate::transaction::{Utxo, BitcoinCashTransaction};
use std::str::FromStr;
use std::marker::PhantomData;
use bip39::{Mnemonic, Language, Seed};
use bch_addr::Converter;
use tcx_chain::bips::DerivationInfo;
use std::mem;



pub struct BchAddress {}

impl BchAddress {
    const XPUB_VERSION: [u8;4] = [0x04, 0x88, 0xb2, 0x1e];
    const XPRV_VERSION: [u8;4] = [0x04, 0x88, 0xad, 0xe4];

    pub fn is_main_net(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_mainnet_addr(addr)
    }
}

impl Address for BchAddress {

    fn is_valid(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(addr)
    }

    fn from_public_key(pub_key: &impl PublicKey) -> Result<String> {
//        let pub_key: &Secp256k1PublicKey = &pub_key;
        let pub_key: Secp256k1PublicKey =  Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
//        let pub_key = pub_key as &Secp256k1PublicKey;
        let legacy = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
        let convert = Converter::new();
        convert.to_cash_addr(&legacy.to_string()).map_err(|err| format_err!("{}", "generate_address_failed"))
    }
}

pub struct BchTestNetAddress {}

impl BchTestNetAddress {
    const XPUB_VERSION: [u8;4] = [0x04, 0x35, 0x87, 0xCF];
    const XPRV_VERSION: [u8;4] = [0x04, 0x35, 0x83, 0x94];
}

impl Address for BchTestNetAddress {

    fn is_valid(address: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(address)
    }

    fn from_public_key(pub_key: &impl PublicKey) -> Result<String> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let legacy = BtcAddress::p2pkh(&pub_key, Network::Testnet);
        let convert = Converter::new();
        convert.to_cash_addr(&legacy.to_string()).map_err(|err| format_err!("{}", "generate_address_failed"))
    }

    fn extended_public_key_version() -> [u8;4] {
        BchTestNetAddress::XPUB_VERSION
    }
    fn extended_private_key_version() -> [u8;4] {
        BchTestNetAddress::XPRV_VERSION
    }

}


#[cfg(test)]
mod tests {
    use tcx_chain::{HdKeystore, Metadata, Account};
    use crate::address::{BchAddress, ExtendedPubKeyExtra};
    use tcx_chain::curve::{Secp256k1Curve, CurveType, PublicKeyType};
    use tcx_chain::coin::Coin;
    use serde_json::Value;
    use tcx_chain::keystore::CoinInfo;

    const PASSWORD: &str = "Insecure Password";
    const BIP_PATH: &str = "m/44'/145'/0'";
    const MNEMONIC: &str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    fn bch_create() {

        let mut meta = Metadata::default();
        meta.name = "CreateTest".to_string();

        let mut keystore = HdKeystore::new("Insecure Password", meta);

//        let coin = BchCoin::<Secp256k1Curve, BchAddress>::append_account(&mut keystore, PASSWORD, BIP_PATH);
        let bch_coin = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };
        let coin: &Account = keystore.derive_coin::<BchAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD).unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();
        assert!(!address.is_empty());
        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BCH", coin);
    }

    #[test]
    fn bch_recover() {
        let mut meta = Metadata::default();
        meta.name = "RecoverTest".to_string();

        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let bch_coin = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };

        let coin: &Account = keystore.derive_coin::<BchAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD).unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();

        assert_eq!("bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885", address);

        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BCH", coin);

        let extra = account["extra"].as_object().expect("extra");
        let xpub = extra["xpub"].as_str().expect("xpub");
        assert_eq!("xpub6Bmkv3mmRZZWoFSBdj9vDMqR2PCPSP6DEj8u3bBuv44g3Ncnro6cPVqZAw6wTEcxHQuodkuJG4EmAinqrrRXGsN3HHnRRMtAvzfYTiBATV1", xpub)
    }


}



