use core::result;

mod address;
mod transaction;

pub type Result<T> = result::Result<T, failure::Error>;

#[macro_use]
extern crate failure;

pub use address::BchAddress;
use tcx_btc_fork::ExtendedPubKeyExtra;
pub use transaction::BchTransaction;

pub type BchExtra = ExtendedPubKeyExtra<BchAddress>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "bch_convert_to_legacy_address_failed# address: {}", _0)]
    ConvertToLegacyAddressFailed(String),
    #[fail(display = "bch_convert_to_cash_address_failed# address: {}", _0)]
    ConvertToCashAddressFailed(String),
    #[fail(display = "construct_bch_address_failed# address: {}", _0)]
    ConstructBchAddressFailed(String),
}

#[cfg(test)]
mod tests {
    use crate::{BchAddress, BchExtra};

    use serde_json::Value;
    use std::str::FromStr;
    use tcx_chain::keystore::CoinInfo;
    use tcx_chain::{HdKeystore, Metadata};
    use tcx_primitive::CurveType;

    const PASSWORD: &str = "Insecure Password";
    const BIP_PATH: &str = "m/44'/145'/0'";
    const MNEMONIC: &str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    fn bch_create() {
        let mut meta = Metadata::default();
        meta.name = "CreateTest".to_string();

        let mut keystore = HdKeystore::new("Insecure Password", meta);

        //        let coin = BchCoin::<Secp256k1Curve, BchAddress>::append_account(&mut keystore, PASSWORD, BIP_PATH);
        let bch_coin = CoinInfo {
            symbol: "BITCOINCASH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };
        let _ = keystore
            .derive_coin::<BchAddress, BchExtra>(&bch_coin, PASSWORD)
            .unwrap();
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
        assert_eq!("BITCOINCASH", coin);
    }

    #[test]
    fn bch_recover() {
        let mut meta = Metadata::default();
        meta.name = "RecoverTest".to_string();

        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let bch_coin = CoinInfo {
            symbol: "BITCOINCASH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };

        let _ = keystore
            .derive_coin::<BchAddress, BchExtra>(&bch_coin, PASSWORD)
            .unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();

        assert_eq!(
            "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885",
            address
        );

        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BITCOINCASH", coin);

        let extra = account["extra"].as_object().expect("extra");
        let enc_xpub = extra["encXPub"].as_str().expect("enc_xpub");
        assert_eq!(enc_xpub, "wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==")
    }

    #[test]
    fn extra_test() {
        let ex = BchExtra::from_xpub("tpubDCpWeoTY6x4BR2PqoTFJnEdfYbjnC4G8VvKoDUPFjt2dvZJWkMRxLST1pbVW56P7zY3L5jq9MRSeff2xsLnvf9qBBN9AgvrhwfZgw5dJG6R", "bch").unwrap();

        assert_eq!(ex.enc_xpub, "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==");

        //        let addr = ex.calc_external_address::<BchAddress>(1i64).unwrap();
        let expected = r#"
        {
            "address": "bitcoincash:qqn4as4zx0jmy02rlgv700umavxt8xtpzu5gcetg92",
            "type": "EXTERNAL",
            "derivedPath": "0/1"
        }
        "#;

        assert_eq!(
            serde_json::to_value(ex.external_address).unwrap(),
            serde_json::Value::from_str(expected).unwrap()
        );
    }
}
