use tcx_chain::curve::{Curve, Secp256k1Curve};
use tcx_chain::{Coin, HdKeystore, Account};
use crate::Result;
use bitcoin::network::constants::Network;
use bitcoin::{Address as BtcAddress, PublicKey, PrivateKey};
use tcx_chain::keystore::{KeyType, Address};
use secp256k1::{SecretKey, Secp256k1};
use bitcoin_hashes::hex::ToHex;
use serde_json::Value;
use crate::bch_transaction::{Utxo, BitcoinCashTransaction};
use std::str::FromStr;
use std::marker::PhantomData;
use bip39::{Mnemonic, Language};
use bch_addr::Converter;
use tcx_chain::bips::DerivationInfo;

const SYMBOL: &'static str = "BCH";
const PATH: &'static str = "m/44'/145'/0'/0/0";

pub struct BchCoin<'z, C: Curve, A: Address> {
    //    derivation_path: String,
    keystore: &'z HdKeystore,
    curve_type: PhantomData<C>,
    address_type: PhantomData<A>,
}

impl<'z, C, A> BchCoin<'z, C, A> where C: Curve, A: Address {
    // can't use associate when use PhantomData
//    const SYMBOL: &'static str = "BCH";
}


impl<'z, C, A> Coin<'z> for BchCoin<'z, C, A> where C: Curve, A: Address {
    fn mnemonic_to_account(mnemonic: &str, path: &str) -> Result<Account> {
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English).map_err(|_| format_err!("invalid_mnemonic"))?;
        let seed = bip39::Seed::new(&mnemonic, &"");
        let main_key = C::key_at_path(path, &seed)?;
        let address = Self::derive_address(&main_key)?;
        let derivation_info = C::extended_pub_key(path, &seed)?;
        let xpub = A::extended_public_key(&derivation_info);
        Ok(Account {
            derivation_path: path.clone().to_string(),
            extended_public_key: xpub,
            coin: SYMBOL.to_string(),
            address,
            extra: "".to_string(),
        })
    }

    fn account(&self) -> &Account {
        self.keystore.account(SYMBOL).unwrap()
    }

    fn derive_address(prv_key: &[u8]) -> Result<String> {
        let pub_key = C::public_key(prv_key)?;
        Ok(A::from_public_key(&pub_key)?)
    }

    fn load(keystore: &'z HdKeystore) -> Result<BchCoin<'z, C, A>> {
        let mut iter = keystore.active_accounts.iter();
        let acc = match iter.find(|a| a.coin == SYMBOL) {
            Some(acc) => Ok(acc),
            _ => Err(format_err!("{}", "keystore_not_contains_account"))
        }?;
        Ok(BchCoin {
            keystore,
            curve_type: PhantomData,
            address_type: PhantomData,
        })
    }

    fn append_account(keystore: &'z mut HdKeystore, password: &str, path: &str) -> Result<BchCoin<'z, C, A>> {
        let seed = keystore.seed(password)?;

        let main_key = C::key_at_path(path, &seed)?;
        let address = Self::derive_address(&main_key)?;
        let derivation_info = C::extended_pub_key(path, &seed)?;
        let xpub = A::extended_public_key(&derivation_info);
        let account = Account {
            derivation_path: path.clone().to_string(),
            extended_public_key: xpub,
            coin: SYMBOL.to_string(),
            address,
            extra: "".to_string(),
        };

        let acc = keystore.append_account(account);
        let coin = BchCoin {
            keystore,
            curve_type: PhantomData,
            address_type: PhantomData,
        };
        Ok(coin)
    }

    fn key(&self, password: &str) -> Result<Vec<u8>> {
        let seed = self.keystore.seed(password)?;
        Ok(C::key_at_path(&self.account().derivation_path, &seed)?)
//        Ok(C::key_at_path("", &seed)?)
    }

    fn extended_private_key(&self, password: &str) -> Result<String> {
        let seed = self.keystore.seed(password)?;
        let derivation_info = C::extended_prv_key(&self.account().derivation_path, &seed)?;
        Ok(A::extended_private_key(&derivation_info))
    }

    fn extended_public_key(&self) -> String {
        self.account().extended_public_key.clone().to_string()
//        "".to_string()
    }

    fn sign_transaction(&self, json: &str) -> Result<String> {
        let v: Value = serde_json::from_str(json).unwrap();
        let unspents: Vec<Utxo> = serde_json::from_value(v["outputs"].clone()).unwrap();
        let internal_used = v["internalUsed"].as_i64().unwrap();
        let change_idx = internal_used + 1;
        let to = v["to"].as_str().unwrap();
        let amount = v["amount"].as_str().unwrap().parse::<i64>().unwrap();
        let fee = v["fee"].as_str().unwrap().parse::<i64>().unwrap();
        let password = v["password"].as_str().unwrap();
        let chain_id = v["chainId"].as_str().unwrap();
        let xprv = self.extended_private_key(password)?;
        let bch_tran = BitcoinCashTransaction {
            to: to.to_owned(),
            amount,
            unspents,
            memo: "".to_string(),
            fee,
            change_idx: change_idx as u32,
        };

        let ret = bch_tran.sign_transaction(chain_id, &xprv)?;
        Ok(serde_json::to_string(&ret)?)
    }
}

pub struct BchAddress {}

impl BchAddress {
    const XPUB_VERSION: [u8;4] = [0x04, 0x88, 0xb2, 0x1e];
    const XPRV_VERSION: [u8;4] = [0x04, 0x88, 0xad, 0xe4];
}

impl Address for BchAddress {
    fn is_valid(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(addr)
    }

    fn from_public_key(pub_key: &[u8]) -> Result<String> {
        let pub_key = PublicKey::from_slice(pub_key)?;
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

    fn from_public_key(public_key: &[u8]) -> Result<String> {
        let pub_key = PublicKey::from_slice(public_key)?;
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



