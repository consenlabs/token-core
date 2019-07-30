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
        let xpub = C::extended_pub_key(path, &seed)?;

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
        // todo network
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

//    fn load(keystore: &HdKeystore) -> Result<BchCoin<'z, C, A>> {
//        let mut iter = keystore.active_accounts.iter();
//        let acc = match iter.find(|a| a.coin == SYMBOL) {
//            Some(acc) => Ok(acc),
//            _ => Err(format_err!("{}", "keystore_not_contains_account"))
//        }?;
//        Ok(BchCoin{
//            account: acc,
//            keystore,
//            curve_type: PhantomData,
//            address_type: PhantomData
//        })
//    }
//

    fn append_account(keystore: &'z mut HdKeystore, password: &str, path: &str) -> Result<BchCoin<'z, C, A>> {
        let seed = keystore.seed(password)?;

        let main_key = C::key_at_path(path, &seed)?;
        let address = Self::derive_address(&main_key)?;
        let xpub = C::extended_pub_key(path, &seed)?;
        let account = Account {
            derivation_path: path.clone().to_string(),
            extended_public_key: xpub,
            coin: SYMBOL.to_string(),
            address,
            extra: "".to_string(),
        };

        let acc = keystore.append_account(account);
        let coin = BchCoin {
            keystore: keystore,
            curve_type: PhantomData,
            address_type: PhantomData,
        };
        Ok(coin)
    }

    fn key(&self, password: &str) -> Result<Vec<u8>> {
        let seed = self.keystore.seed(password)?;
//        Ok(C::key_at_path(&self.account.derivation_path, &seed)?)
        Ok(C::key_at_path("", &seed)?)
    }

    fn extended_private_key(&self, password: &str) -> Result<String> {
        let seed = self.keystore.seed(password)?;
        Ok(C::extended_prv_key(&self.account().derivation_path, &seed)?)
//        Ok(C::extended_prv_key("", &seed)?)
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

impl Address for BchAddress {
    fn is_valid(addr: &str) -> bool {
        BtcAddress::from_str(addr).is_ok()
    }

    fn from_public_key(pub_key: &[u8]) -> Result<String> {
        let pub_key = PublicKey::from_slice(pub_key)?;
        Ok(BtcAddress::p2wpkh(&pub_key, Network::Bitcoin).to_string())
    }
}




