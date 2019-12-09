extern crate serde_json;
extern crate tcx;
extern crate tcx_btc_fork;
extern crate tcx_chain;

use serde_json::Value;
use std::ffi::{CStr, CString};
use std::fs::{create_dir, remove_dir};
use std::path::Path;
use tcx::*;
use tcx_btc_fork::signer::Utxo;
fn main() {
    // 1) scan_wallets
    // 必须先调用scan_wallets, 设置wallet_dir
    let path = Path::new("/tmp/.imtoken");
    if path.exists() {
        let _ = remove_dir(path);
    }

    let _ = create_dir(path);
    let json = format!(
        r#"{{"fileDir": "{}","xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
            "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799","version": "0.1"}}"#,
        path.to_str().unwrap()
    );

    let json = CString::new(json).expect("CString:new failed");
    unsafe {
        init_token_core_x(json.as_ptr());
        let err = CStr::from_ptr(get_last_err_message());
        println!("scan_wallets: ");
        println!("scan_wallets(error): {:?}", err.to_str());
    }

    // 2) create_wallet
    let json = r#"{
        "passwordHint": "hint",
        "name": "urugang",
        "timestamp": 0,
        "source": "NEW_IDENTITY",
        "password": "mypass"
    }"#;
    let json = CString::new(json).expect("CString:new failed");
    let id = unsafe {
        let result = create_wallet(json.as_ptr());
        let result = CStr::from_ptr(result);
        println!("create_wallet: {:?}", result);
        let err = CStr::from_ptr(get_last_err_message());
        println!("create_wallet(error): {:?}", err.to_str());

        let result = result.to_str().unwrap();
        let value: Value = serde_json::from_str(result).unwrap();
        value["id"].to_string()
    };
    println!("{}", id);
    // 3) export mnemonic (optional)
    let json = format!("{{\"id\": {},\"password\": \"mypass\"}}", id);
    let json = CString::new(json).expect("CString:new failed");
    let mnemonic: &str = unsafe {
        let result = export_mnemonic(json.as_ptr());
        let result = CStr::from_ptr(result);
        println!("export_mnemonic: {:?}", result);
        let err = CStr::from_ptr(get_last_err_message());
        println!("export_mnemonic(error): {:?}", err.to_str());
        result.to_str().unwrap()
    };

    // 4) import wallet (optional)
    let json = format!(
        r#"{{
        "passwordHint": "hint",
        "name": "urugang",
        "timestamp": 0,
        "source": "NEW_IDENTITY",
        "password": "mypass",
        "mnemonic": "{}",
        "path": "0/1",
        "overwrite": true,
        "chainType": "BCH" 
        }}"#,
        mnemonic
    );
    let json = CString::new(json).expect("CString:new failed");
    unsafe {
        let result = import_wallet_from_mnemonic(json.as_ptr());
        let result = CStr::from_ptr(result);
        println!("import_wallet: {:?}", result);
        let err = CStr::from_ptr(get_last_err_message());
        println!("import_wallet(error): {:?}", err.to_str());
    }

    // 5) find keystore
    let json = format!(
        r#"{{
        "mnemonic": "{}",
        "network": "mainnet",
        "chainType": "BCH",
        "password": "mypass",
        "name": "urugang",
        "passwordHint": "hint",
        "timestamp": 0,
        "source": "KEYSTORE",
        "path": "0/1"
        }}"#,
        mnemonic
    );
    let json = CString::new(json).expect("CString:new failed");
    let id = unsafe {
        let result = find_wallet_by_mnemonic(json.as_ptr());
        let result = CStr::from_ptr(result);
        println!("find keystore: {:?}", result);
        let err = CStr::from_ptr(get_last_err_message());
        println!("find keystore(error): {:?}", err.to_str());

        let result = result.to_str().unwrap();
        let value: Value = serde_json::from_str(result).unwrap();
        value["id"].to_string()
    };

    // 6) sign_tx
    let mut txinputs: Vec<Utxo> = vec![];
    for _ in 0..2 {
        txinputs.push(Utxo {
            tx_hash: "115e8f72f39fad874cfab0deed11a80f24f967a84079fb56ddf53ea02e308986".to_string(), // txid
            vout: 0,       // outputIndex
            amount: 12000, // satosh
            address: "17XBj6iFEsf8kzDMGQk5ghZipxX49VXuaV".to_string(),
            script_pub_key: "76a91447862fe165e6121af80d5dde1ecb478ed170565b88ac".to_string(), // script
            derived_path: "0/1".to_string(),
            sequence: 0,
        });
    }

    let txinputs = serde_json::to_string(&txinputs).unwrap();

    let json = format!(
        r#"{{
        "id": {},
        "chainType": "BCH",
        "to": "{}",
        "amount": "1200",
        "fee": "13",
        "internalUsed": 1,
        "outputs": {:},
        "password": "mypass"
        }}"#,
        id, "qrj5mazh5vayn8jnqqsgqymnjtn7wshejyt3un6758", txinputs
    );
    let json = CString::new(json).expect("CString:new failed");
    unsafe {
        let result = sign_transaction(json.as_ptr());
        let result = CStr::from_ptr(result);
        println!("sign_tx: {:?}", result);
        let err = CStr::from_ptr(get_last_err_message());
        println!("sign_tx(error): {:?}", err.to_str());
    }
}
