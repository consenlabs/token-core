use std::ffi::{CStr, CString};

use std::os::raw::c_char;

use prost::Message;

pub mod api;

use crate::api::{Response, TcxAction};

pub mod error_handling;
pub mod handler;

use crate::error_handling::{landingpad, LAST_BACKTRACE, LAST_ERROR};
#[allow(deprecated)]
use crate::handler::{
    encode_message, export_mnemonic, export_private_key, get_derived_key, hd_store_create,
    hd_store_export, hd_store_import, keystore_common_accounts, keystore_common_delete,
    keystore_common_derive, keystore_common_exists, keystore_common_verify,
    private_key_store_export, private_key_store_import, sign_tx, tron_sign_message,
    unlock_then_crash,
};

mod filemanager;

use crate::handler::{export_substrate_keystore, import_substrate_keystore};
use parking_lot::RwLock;

extern crate serde_json;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref IS_DEBUG: RwLock<bool> = RwLock::new(false);
}

#[no_mangle]
pub unsafe extern "C" fn free_const_string(s: *const c_char) {
    if s.is_null() {
        return;
    }
    CStr::from_ptr(s);
}

/// dispatch protobuf rpc call
///
#[allow(deprecated)]
#[no_mangle]
pub unsafe extern "C" fn call_tcx_api(hex_str: *const c_char) -> *const c_char {
    let hex_c_str = CStr::from_ptr(hex_str);
    let hex_str = hex_c_str.to_str().expect("parse_arguments to_str");

    let data = hex::decode(hex_str).expect("parse_arguments hex decode");
    let action: TcxAction = TcxAction::decode(data.as_slice()).expect("decode tcx api");
    let reply: Vec<u8> = match action.method.to_lowercase().as_str() {
        "init_token_core_x" => landingpad(|| {
            handler::init_token_core_x(&action.param.unwrap().value).unwrap();
            Ok(vec![])
        }),
        "scan_keystores" => landingpad(|| {
            handler::scan_keystores().unwrap();
            Ok(vec![])
        }),
        "hd_store_create" => landingpad(|| hd_store_create(&action.param.unwrap().value)),
        "hd_store_import" => landingpad(|| hd_store_import(&action.param.unwrap().value)),
        "hd_store_export" => landingpad(|| hd_store_export(&action.param.unwrap().value)),
        "export_mnemonic" => landingpad(|| export_mnemonic(&action.param.unwrap().value)),
        "keystore_common_derive" => {
            landingpad(|| keystore_common_derive(&action.param.unwrap().value))
        }

        "private_key_store_import" => {
            landingpad(|| private_key_store_import(&action.param.unwrap().value))
        }
        "private_key_store_export" => {
            landingpad(|| private_key_store_export(&action.param.unwrap().value))
        }
        "export_private_key" => landingpad(|| export_private_key(&action.param.unwrap().value)),
        "keystore_common_verify" => {
            landingpad(|| keystore_common_verify(&action.param.unwrap().value))
        }
        "keystore_common_delete" => {
            landingpad(|| keystore_common_delete(&action.param.unwrap().value))
        }
        "keystore_common_exists" => {
            landingpad(|| keystore_common_exists(&action.param.unwrap().value))
        }
        "keystore_common_accounts" => {
            landingpad(|| keystore_common_accounts(&action.param.unwrap().value))
        }

        "sign_tx" => landingpad(|| sign_tx(&action.param.unwrap().value)),

        "tron_sign_msg" => landingpad(|| tron_sign_message(&action.param.unwrap().value)),

        "substrate_keystore_import" => {
            landingpad(|| import_substrate_keystore(&action.param.unwrap().value))
        }

        "substrate_keystore_export" => {
            landingpad(|| export_substrate_keystore(&action.param.unwrap().value))
        }

        // !!! WARNING !!! used for `cache_dk` feature
        "get_derived_key" => landingpad(|| get_derived_key(&action.param.unwrap().value)),
        // !!! WARNING !!! used for test only
        "unlock_then_crash" => landingpad(|| unlock_then_crash(&action.param.unwrap().value)),
        _ => landingpad(|| Err(format_err!("unsupported_method"))),
    };

    let ret_str = hex::encode(reply);
    CString::new(ret_str).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn clear_err() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
    LAST_BACKTRACE.with(|e| {
        *e.borrow_mut() = None;
    });
}

#[no_mangle]
pub unsafe extern "C" fn get_last_err_message() -> *const c_char {
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            let rsp = Response {
                is_success: false,
                error: err.to_string(),
            };
            let rsp_bytes = encode_message(rsp).expect("encode error");
            let ret_str = hex::encode(rsp_bytes);
            CString::new(ret_str).unwrap().into_raw()
        } else {
            CString::new("").unwrap().into_raw()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filemanager::KEYSTORE_MAP;
    use api::sign_param::Key;
    use error_handling::Result;
    use std::ffi::{CStr, CString};
    use std::fs::remove_file;
    use std::os::raw::c_char;
    use std::panic;
    use std::path::Path;

    use crate::api::keystore_common_derive_param::Derivation;
    use crate::api::{
        AccountsResponse, DerivedKeyResult, ExportPrivateKeyParam, ExportSubstrateKeystoreResult,
        HdStoreCreateParam, ImportSubstrateKeystoreParam, InitTokenCoreXParam, KeyType,
        KeystoreCommonAccountsParam, KeystoreCommonDeriveParam, KeystoreCommonExistsParam,
        KeystoreCommonExistsResult, KeystoreCommonExportResult, PrivateKeyStoreExportParam,
        PrivateKeyStoreImportParam, Response, SignParam, WalletKeyParam,
    };
    use crate::api::{HdStoreImportParam, WalletResult};
    use crate::handler::hd_store_import;
    use crate::handler::{encode_message, private_key_store_import};
    use prost::Message;
    use tcx_chain::Keystore;
    use tcx_constants::{TEST_MNEMONIC, TEST_PASSWORD};

    use std::fs;
    use tcx_btc_fork::transaction::BtcForkTxInput;
    use tcx_btc_fork::transaction::Utxo;

    use sp_core::Public as TraitPublic;
    use sp_runtime::traits::Verify;
    use tcx_ckb::{CachedCell, CellInput, CkbTxInput, CkbTxOutput, OutPoint, Script, Witness};
    use tcx_substrate::{SubstrateRawTxIn, SubstrateTxOut};
    use tcx_tron::transaction::{TronMessageInput, TronMessageOutput, TronTxInput, TronTxOutput};

    static OTHER_MNEMONIC: &'static str =
        "calm release clay imitate top extend close draw quiz refuse shuffle injury";

    fn _to_c_char(str: &str) -> *const c_char {
        CString::new(str).unwrap().into_raw()
    }

    fn _to_str(json_str: *const c_char) -> &'static str {
        let json_c_str = unsafe { CStr::from_ptr(json_str) };
        json_c_str.to_str().unwrap()
    }

    fn setup() {
        let p = Path::new("/tmp/imtoken/wallets");
        if !p.exists() {
            fs::create_dir_all(p).expect("shoud create filedir");
        }

        let param = InitTokenCoreXParam {
            file_dir: "/tmp/imtoken/wallets".to_string(),
            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
            is_debug: true,
        };

        handler::init_token_core_x(&encode_message(param).unwrap()).expect("should init tcx");
    }

    fn teardown() {
        let p = Path::new("/tmp/imtoken/wallets");
        let walk_dir = std::fs::read_dir(p).expect("read dir");
        for entry in walk_dir {
            let entry = entry.expect("DirEntry");
            let fp = entry.path();
            if !fp
                .file_name()
                .expect("file_name")
                .to_str()
                .expect("file_name str")
                .ends_with(".json")
            {
                continue;
            }

            remove_file(fp.as_path()).expect("should remove file");
        }
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();
        let result = panic::catch_unwind(|| test());
        teardown();
        assert!(result.is_ok())
    }

    fn import_default_wallet() -> WalletResult {
        let param = HdStoreImportParam {
            mnemonic: TEST_MNEMONIC.to_string(),
            // mnemonic: TEST_MNEMONIC.to_string(),
            password: TEST_PASSWORD.to_string(),
            source: "MNEMONIC".to_string(),
            name: "test-wallet".to_string(),
            password_hint: "imtoken".to_string(),
            overwrite: true,
        };
        let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
        WalletResult::decode(ret.as_slice()).unwrap()
    }

    fn import_default_pk_store() -> WalletResult {
        let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
            private_key: "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB".to_string(),
            password: TEST_PASSWORD.to_string(),
            overwrite: true,
        };

        let ret = private_key_store_import(&encode_message(param).unwrap()).unwrap();
        WalletResult::decode(ret.as_slice()).unwrap()
    }

    fn import_and_derive(derivation: Derivation) -> WalletResult {
        let mut wallet = import_default_wallet();

        let param = KeystoreCommonDeriveParam {
            id: wallet.id.to_string(),
            password: TEST_PASSWORD.to_string(),
            derivations: vec![derivation],
        };

        let ret = call_api("keystore_common_derive", param).unwrap();
        let accounts: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();

        wallet.accounts = accounts.accounts.clone();

        wallet
    }

    fn import_pk_and_derive(derivation: Derivation) -> WalletResult {
        let mut wallet = import_default_pk_store();

        let param = KeystoreCommonDeriveParam {
            id: wallet.id.to_string(),
            password: TEST_PASSWORD.to_string(),
            derivations: vec![derivation],
        };

        let ret = call_api("keystore_common_derive", param).unwrap();
        let accounts: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();

        wallet.accounts = accounts.accounts.clone();

        wallet
    }

    fn call_api(method: &str, msg: impl Message) -> Result<Vec<u8>> {
        let param = TcxAction {
            method: method.to_string(),
            param: Some(::prost_types::Any {
                type_url: "imtoken".to_string(),
                value: encode_message(msg).unwrap(),
            }),
        };
        let _ = unsafe { clear_err() };
        let param_bytes = encode_message(param).unwrap();
        let param_hex = hex::encode(param_bytes);
        let ret_hex = unsafe { _to_str(call_tcx_api(_to_c_char(&param_hex))) };
        let err = unsafe { _to_str(get_last_err_message()) };
        if !err.is_empty() {
            let err_bytes = hex::decode(err).unwrap();
            let err_ret: Response = Response::decode(err_bytes.as_slice()).unwrap();
            Err(format_err!("{}", err_ret.error))
        } else {
            Ok(hex::decode(ret_hex).unwrap())
        }
    }

    #[test]
    fn test_call_tcx_api() {
        run_test(|| {
            let _import_param = HdStoreImportParam {
                mnemonic: TEST_MNEMONIC.to_string(),
                password: TEST_PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "call_tcx_api".to_string(),
                password_hint: "".to_string(),
                overwrite: true,
            };
            // let ret_bytes = call_api("hd_store_import", import_param).unwrap();
            let ret_bytes = hex::decode("0a2434656239623136392d323237392d343439332d616535342d62396233643761303630323512036161611a084d4e454d4f4e494328e9a1a2f305").unwrap();
            let ret: WalletResult = WalletResult::decode(ret_bytes.as_slice()).unwrap();
            assert!(ret.accounts.is_empty())
        });
    }

    #[test]
    pub fn test_scan_keystores() {
        let param = InitTokenCoreXParam {
            file_dir: "../test-data".to_string(),
            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
            is_debug: true,
        };

        handler::init_token_core_x(&encode_message(param).unwrap()).expect("should init tcx");

        let keystore_count;
        {
            let mut map = KEYSTORE_MAP.write();
            keystore_count = map.len();
            map.clear();
            assert_eq!(0, map.len());
        }
        let empty = WalletKeyParam {
            id: "".to_string(),
            password: "".to_string(),
        };
        let _ = call_api("scan_keystores", empty);
        {
            let map = KEYSTORE_MAP.write();

            assert_eq!(keystore_count, map.len());
        }
    }

    #[test]
    pub fn test_hd_store_create() {
        run_test(|| {
            let param = HdStoreCreateParam {
                password: TEST_PASSWORD.to_string(),
                password_hint: "".to_string(),
                name: "aaa".to_string(),
            };

            let ret = call_api("hd_store_create", param).unwrap();
            let import_result: WalletResult = WalletResult::decode(ret.as_slice()).unwrap();

            assert!(import_result.accounts.is_empty());
            assert_eq!(import_result.name, "aaa");
            assert_eq!(import_result.source, "MNEMONIC");
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_hd_store_import() {
        run_test(|| {
            let import_result: WalletResult = import_default_wallet();
            assert_eq!(import_result.source, "MNEMONIC");
            let derivation = Derivation {
                chain_type: "BITCOINCASH".to_string(),
                path: "m/44'/145'/0'/0/0".to_string(),
                network: "MAINNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = call_api("keystore_common_derive", param).unwrap();
            let result: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();
            assert_eq!(result.accounts.first().unwrap().chain_type, "BITCOINCASH");
            assert_eq!(
                result.accounts.first().unwrap().address,
                "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r"
            );
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_hd_store_import_invalid_params() {
        run_test(|| {
            let invalid_mnemonics = vec![
                "inject kidney empty canal shadow pact comfort wife crush horse",
                "inject kidney empty canal shadow pact comfort wife crush horse wife wife",
                "inject kidney empty canal shadow pact comfort wife crush horse hello",
            ];
            for mn in invalid_mnemonics {
                let param = HdStoreImportParam {
                    mnemonic: mn.to_string(),
                    password: TEST_PASSWORD.to_string(),
                    source: "MNEMONIC".to_string(),
                    name: "test-wallet".to_string(),
                    password_hint: "imtoken".to_string(),
                    overwrite: true,
                };

                let ret = call_api("hd_store_import", param);
                assert!(ret.is_err());
            }
        })
    }

    #[test]
    pub fn test_hd_store_import_ltc() {
        run_test(|| {
            let import_result: WalletResult = import_default_wallet();

            let derivation = Derivation {
                chain_type: "LITECOIN".to_string(),
                path: "m/44'/1'/0'/0/0".to_string(),
                network: "TESTNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = call_api("keystore_common_derive", param).unwrap();
            let result: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();
            assert_eq!(result.accounts.first().unwrap().chain_type, "LITECOIN");
            assert_eq!(
                result.accounts.first().unwrap().address,
                "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN"
            );

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_hd_store_export() {
        run_test(|| {
            let wallet = import_default_wallet();

            let param = WalletKeyParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };
            let ret = call_api("hd_store_export", param).unwrap();
            let result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(ret.as_slice()).unwrap();

            assert_eq!(result.r#type, KeyType::Mnemonic as i32);
            assert_eq!(result.value, TEST_MNEMONIC);
        })
    }

    #[test]
    pub fn test_export_mnemonic() {
        run_test(|| {
            let wallet = import_default_wallet();

            let param = WalletKeyParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };
            let ret = call_api("export_mnemonic", param).unwrap();
            let result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(ret.as_slice()).unwrap();

            assert_eq!(result.r#type, KeyType::Mnemonic as i32);
            assert_eq!(result.value, TEST_MNEMONIC);

            let wallet = import_default_pk_store();

            let param = WalletKeyParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };
            unsafe { clear_err() };
            let ret = call_api("export_mnemonic", param);
            assert!(ret.is_err());
            assert_eq!(
                format!("{}", ret.err().unwrap()),
                "private_keystore_cannot_export_mnemonic"
            );
        })
    }

    #[test]
    pub fn test_keystore_common_store_derive() {
        run_test(|| {
            let param = HdStoreImportParam {
                mnemonic: OTHER_MNEMONIC.to_string(),
                password: TEST_PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };
            let ret = call_api("hd_store_import", param).unwrap();
            let import_result: WalletResult = WalletResult::decode(ret.as_slice()).unwrap();

            let derivations = vec![
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "m/44'/2'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "m/49'/2'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "P2WPKH".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "m/49'/1'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "TRON".to_string(),
                    path: "m/44'/195'/0'/0/0".to_string(),
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "NERVOS".to_string(),
                    path: "m/44'/309'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "KUSAMA".to_string(),
                    path: "//kusama//imToken/0".to_string(),
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "POLKADOT".to_string(),
                    path: "//polkadot//imToken/0".to_string(),
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
            ];
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes = call_api("keystore_common_derive", param).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes.as_slice()).unwrap();
            assert_eq!(7, derived_accounts.accounts.len());
            assert_eq!(
                "LQ3JqCohgLQ3x1CJXYERnJTy1ySaqr1E32",
                derived_accounts.accounts[0].address
            );
            assert_eq!("/EhDRyPFcj1UGx8i+WiJSIeBSyaN0pX7Oq3wXqwO5M9T1aRhfLpsNPGAPLf07K+p+B0OdQW1ogVbDQCWkIwVXZLPY+njp9LjXaICiWGEeidR1TwBZSwOMRKE68wJWH/7puxYfY/Rq1+d2GFv6NxSCw==", derived_accounts.accounts[0].extended_xpub_key);

            assert_eq!(
                "MQUu6P7wsLQZfVZMuFWB7UXiheuVTM7RYF",
                derived_accounts.accounts[1].address
            );
            assert_eq!("A5LUzJcPB4r54wqr8EjFh9fe0L87spIN9KJKtzHV6QJXBH6GEAiYT57uftpJITx613HdIXXzi8VJ30TmG8erBF30oD1DnbDmGmDo4sdRTdQSsp9NuprhZ3Y3PR9+xzdc2tKDblRL5dLZswaPxCOQcw==", derived_accounts.accounts[1].extended_xpub_key);

            assert_eq!(
                "mvdDMnRsqjqzvCyYyRXpvscmnU1FxodhkE",
                derived_accounts.accounts[2].address
            );
            assert_eq!("eZIL4e0a8qw18Pve92iLfehteHDA+kqjwv91aKE+2hNN3arkq20yY2Mx6q4WAowFv0QRfIi6QlrhafJKUpjiC469NNZagCSHLaECYliEwmwTgC97zXmVJDB6MJi79y+mznf8G7Few8+u6UfiXELN5g==", derived_accounts.accounts[2].extended_xpub_key);

            assert_eq!(
                "TLZnqkrSNLUWNrZMug8u9b6pJ3XcTGbzDV",
                derived_accounts.accounts[3].address
            );
            assert_eq!("Sla41n5BdHqc1QmqA9DXjWNx13Fpq18u19jCaMbYbxClsPr7cr/gzXsbE+08wfNLuGgtVVY4/prpnv3/pdJ8KA/I/iOKvelKxuJgN9n2O5Q54CmObc0qJVZxcAQM0PbrKE9YJyGDkJNMLM+OmjEwjg==", derived_accounts.accounts[3].extended_xpub_key);

            assert_eq!(
                "ckt1qyqgkffut7e7md39tp5ts9vxssj7wdw8z4cquyflka",
                derived_accounts.accounts[4].address
            );

            assert_eq!(
                "HFEP5ePp69xrCLTYcDnzqJTgmH87RUKprkoRUuEmu9Tk49s",
                derived_accounts.accounts[5].address
            );
            assert_eq!(
                "13GVaZUS28zTCroTPq8dyppfm8F4cAvoJsSZ3yvmtyRYLSLJ",
                derived_accounts.accounts[6].address
            );

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_hd_store_derive_invalid_param() {
        run_test(|| {
            let import_result: WalletResult = import_default_wallet();

            let invalid_derivations = vec![
                Derivation {
                    chain_type: "WRONG_CHAIN_TYPE".to_string(),
                    path: "m/44'/2'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "WRONG/PATH".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "P2WPKH".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "49'/1'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
            ];
            for derivation in invalid_derivations {
                let param = KeystoreCommonDeriveParam {
                    id: import_result.id.to_string(),
                    password: TEST_PASSWORD.to_string(),
                    derivations: vec![derivation],
                };
                let ret = call_api("keystore_common_derive", param);
                assert!(ret.is_err());
            }

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_private_key_store_import() {
        run_test(|| {
            let import_result: WalletResult = import_default_pk_store();

            assert_eq!(0, import_result.accounts.len());

            let derivations = vec![
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "m/44'/2'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "m/49'/2'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "P2WPKH".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "LITECOIN".to_string(),
                    path: "m/49'/1'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "TRON".to_string(),
                    path: "m/44'/195'/0'/0/0".to_string(),
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "NERVOS".to_string(),
                    path: "m/44'/309'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
            ];
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes = call_api("keystore_common_derive", param).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes.as_slice()).unwrap();
            assert_eq!(5, derived_accounts.accounts.len());
            assert_eq!(
                "LgGNTHMkgETS7oQcoekvACJQcH355xECog",
                derived_accounts.accounts[0].address
            );
            assert_eq!("", derived_accounts.accounts[0].extended_xpub_key);

            assert_eq!(
                "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW",
                derived_accounts.accounts[1].address
            );
            assert_eq!("", derived_accounts.accounts[1].extended_xpub_key);

            assert_eq!(
                "n2ZNV88uQbede7C5M5jzi6SyG4GVuPpng6",
                derived_accounts.accounts[2].address
            );
            assert_eq!("", derived_accounts.accounts[2].extended_xpub_key);

            assert_eq!(
                "TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG",
                derived_accounts.accounts[3].address
            );
            assert_eq!("", derived_accounts.accounts[3].extended_xpub_key);

            assert_eq!(
                "ckt1qyqpavderq5jjxh6qhxeks4t706kglffkyassx7h5z",
                derived_accounts.accounts[4].address
            );

            // pk rederive
            let derivations = vec![Derivation {
                chain_type: "LITECOIN".to_string(),
                path: "m/44'/2'/0'/0/0".to_string(),
                network: "MAINNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            }];
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes = call_api("keystore_common_derive", param).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes.as_slice()).unwrap();
            assert_eq!(
                "LgGNTHMkgETS7oQcoekvACJQcH355xECog",
                derived_accounts.accounts[0].address
            );
            assert_eq!("", derived_accounts.accounts[0].extended_xpub_key);

            let param = KeystoreCommonAccountsParam {
                id: import_result.id.to_string(),
            };
            let accounts_ret = call_api("keystore_common_accounts", param).unwrap();
            let ret = AccountsResponse::decode(accounts_ret.as_slice()).unwrap();
            assert_eq!(5, ret.accounts.len());

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_64bytes_private_key_store_import() {
        run_test(|| {
            let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
                private_key: "416c696365202020202020202020202020202020202020202020202020202020d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4f"
                    .to_string(),
                password: TEST_PASSWORD.to_string(),
                overwrite: true,
            };

            let ret = private_key_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(ret.as_slice()).unwrap();

            assert_eq!(0, import_result.accounts.len());

            let derivations = vec![Derivation {
                chain_type: "POLKADOT".to_string(),
                path: "".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            }];
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes = call_api("keystore_common_derive", param).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes.as_slice()).unwrap();
            assert_eq!(1, derived_accounts.accounts.len());

            assert_eq!(
                "133smEABgtt8FRkZGrZfAzCV522bxo2y5FwVoTcSaY8z1nEq",
                derived_accounts.accounts[0].address
            );

            let export_param = ExportPrivateKeyParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "POLKADOT".to_string(),
                network: "".to_string(),
                main_address: "133smEABgtt8FRkZGrZfAzCV522bxo2y5FwVoTcSaY8z1nEq".to_string(),
                path: "".to_string(),
            };

            let export_pk_bytes = call_api("export_private_key", export_param).unwrap();
            let export_pk: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(export_pk_bytes.as_slice()).unwrap();
            assert_eq!(
                export_pk.value,
                "416c696365202020202020202020202020202020202020202020202020202020d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4f"
            );
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_private_key_store_export() {
        run_test(|| {
            let import_result: WalletResult = import_default_pk_store();
            let param: PrivateKeyStoreExportParam = PrivateKeyStoreExportParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "BITCOINCASH".to_string(),
                network: "MAINNET".to_string(),
            };
            let ret_bytes = call_api("private_key_store_export", param).unwrap();
            let export_result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(ret_bytes.as_slice()).unwrap();
            assert_eq!(
                "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
                export_result.value
            );
            assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);

            let param: PrivateKeyStoreExportParam = PrivateKeyStoreExportParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "BITCOINCASH".to_string(),
                network: "TESTNET".to_string(),
            };
            let ret_bytes = call_api("private_key_store_export", param).unwrap();
            let export_result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(ret_bytes.as_slice()).unwrap();
            assert_eq!(
                "cT4fTJyLd5RmSZFHnkGmVCzXDKuJLbyTt7cy77ghTTCagzNdPH1j",
                export_result.value
            );
            assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);

            let param: PrivateKeyStoreExportParam = PrivateKeyStoreExportParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "TRON".to_string(),
                network: "".to_string(),
            };
            let ret_bytes = call_api("private_key_store_export", param).unwrap();
            let export_result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(ret_bytes.as_slice()).unwrap();
            assert_eq!(
                "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
                export_result.value
            );
            assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_export_private_key() {
        run_test(|| {
            let derivations = vec![
                Derivation {
                    chain_type: "BITCOINCASH".to_string(),
                    path: "".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "BITCOINCASH".to_string(),
                    path: "".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "TRON".to_string(),
                    path: "".to_string(),
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
            ];
            let pks = vec![
                "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
                "cT4fTJyLd5RmSZFHnkGmVCzXDKuJLbyTt7cy77ghTTCagzNdPH1j",
                "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            ];

            for idx in 0..3 {
                let import_result: WalletResult = import_pk_and_derive(derivations[idx].clone());
                let acc = import_result.accounts.first().unwrap().clone();
                let param: ExportPrivateKeyParam = ExportPrivateKeyParam {
                    id: import_result.id.to_string(),
                    password: TEST_PASSWORD.to_string(),
                    chain_type: acc.chain_type.to_string(),
                    network: derivations[idx].network.to_string(),
                    main_address: acc.address.clone(),
                    path: "".to_string(),
                };
                let ret_bytes = call_api("export_private_key", param).unwrap();
                let export_result: KeystoreCommonExportResult =
                    KeystoreCommonExportResult::decode(ret_bytes.as_slice()).unwrap();

                // test export as mainnet
                assert_eq!(pks[idx], export_result.value);
                assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);
                remove_created_wallet(&import_result.id);
            }

            let wallet = import_default_pk_store();
            let param: ExportPrivateKeyParam = ExportPrivateKeyParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "LITECOIN".to_string(),
                network: "MAINNET".to_string(),
                main_address: "Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP".to_string(),
                path: "".to_string(),
            };
            let ret = call_api("export_private_key", param);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "account_not_found");
        })
    }

    #[test]
    pub fn test_export_private_key_from_hd_store() {
        run_test(|| {
            let derivations = vec![
                Derivation {
                    chain_type: "BITCOINCASH".to_string(),
                    path: "m/44'/145'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "BITCOINCASH".to_string(),
                    path: "m/44'/145'/0'/0/0".to_string(),
                    network: "MAINNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "BITCOINCASH".to_string(),
                    path: "m/44'/1'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                },
                Derivation {
                    chain_type: "TRON".to_string(),
                    path: "m/44'/195'/0'/0/0".to_string(),
                    network: "".to_string(),
                    seg_wit: "".to_string(),
                    chain_id: "".to_string(),
                },
            ];

            let pks = vec![
                "L39VXyorp19JfsEJfbD7Tfr4pBEX93RJuVXW7E13C51ZYAhUWbYa",
                "KyLGdagds7tY1vupT5Kf8C1Cc5wkzzWRK51e4vsh1svCSvYk4Abo",
                "cN4b1V3cicEexrYXiEhaWEdURyhZiVX6PzAZNFSzZaWfSNZG2cJX",
                "685634d212eabe016a1cb09d9f1ea1ea757ebe590b9a097d7b1c9379ad280171",
            ];
            let export_paths = vec![
                "m/44'/145'/0'/0/0",
                "m/44'/145'/0'/0/1",
                "m/44'/1'/0'/0/1",
                "m/44'/195'/0'/0/1",
            ];
            for idx in 0..derivations.len() {
                let import_result: WalletResult = import_and_derive(derivations[idx].clone());
                let acc = import_result.accounts.first().unwrap().clone();
                let param: ExportPrivateKeyParam = ExportPrivateKeyParam {
                    id: import_result.id.to_string(),
                    password: TEST_PASSWORD.to_string(),
                    chain_type: acc.chain_type.to_string(),
                    network: derivations[idx].network.to_string(),
                    main_address: acc.address.to_string(),
                    path: export_paths[idx].to_string(),
                };
                let ret_bytes = call_api("export_private_key", param).unwrap();
                let export_result: KeystoreCommonExportResult =
                    KeystoreCommonExportResult::decode(ret_bytes.as_slice()).unwrap();

                assert_eq!(pks[idx], export_result.value);
                assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);
                remove_created_wallet(&import_result.id);
            }

            let import_result: WalletResult = import_default_wallet();

            let param: ExportPrivateKeyParam = ExportPrivateKeyParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "LITECOIN".to_string(),
                network: "MAINNET".to_string(),
                main_address: "Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP".to_string(),
                path: "m/44'/2'/0'/0/0".to_string(),
            };
            let ret = call_api("export_private_key", param);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "account_not_found");
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_import_to_pk_which_from_hd() {
        run_test(|| {
            let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
                private_key: "L39VXyorp19JfsEJfbD7Tfr4pBEX93RJuVXW7E13C51ZYAhUWbYa".to_string(),
                password: TEST_PASSWORD.to_string(),
                overwrite: true,
            };

            let ret = private_key_store_import(&encode_message(param).unwrap()).unwrap();
            let wallet: WalletResult = WalletResult::decode(ret.as_slice()).unwrap();

            let derivation = Derivation {
                chain_type: "BITCOINCASH".to_string(),
                path: "".to_string(),
                network: "MAINNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            };

            let derive_param = KeystoreCommonDeriveParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations: vec![derivation],
            };
            let ret_bytes = keystore_common_derive(&encode_message(derive_param).unwrap()).unwrap();
            let ret: AccountsResponse = AccountsResponse::decode(ret_bytes.as_slice()).unwrap();
            assert_eq!(
                "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
                ret.accounts.first().unwrap().address
            );
            remove_created_wallet(&wallet.id);
        })
    }

    #[test]
    pub fn test_keystore_common_verify() {
        run_test(|| {
            let wallets = vec![import_default_pk_store(), import_default_wallet()];
            for wallet in wallets {
                let param: WalletKeyParam = WalletKeyParam {
                    id: wallet.id.to_string(),
                    password: TEST_PASSWORD.to_string(),
                };

                let ret_bytes = call_api("keystore_common_verify", param).unwrap();
                let result: Response = Response::decode(ret_bytes.as_slice()).unwrap();
                assert!(result.is_success);

                let param: WalletKeyParam = WalletKeyParam {
                    id: wallet.id.to_string(),
                    password: "WRONG PASSWORD".to_string(),
                };

                let ret = call_api("keystore_common_verify", param);
                assert!(ret.is_err());
                assert_eq!(format!("{}", ret.err().unwrap()), "password_incorrect");
            }
        })
    }

    #[test]
    pub fn test_keystore_common_delete() {
        run_test(|| {
            let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
                private_key: "5JZc7wGRUr4J1RHDcM9ySWKLfQ2xjRUEo612qC4RLJ3G7jzJ4qx".to_string(),
                password: TEST_PASSWORD.to_string(),
                overwrite: true,
            };

            let ret_bytes = private_key_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(ret_bytes.as_slice()).unwrap();

            let param: WalletKeyParam = WalletKeyParam {
                id: import_result.id.to_string(),
                password: "WRONG PASSWORD".to_string(),
            };

            let ret = call_api("keystore_common_delete", param);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "password_incorrect");

            let param: WalletKeyParam = WalletKeyParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };

            let ret_bytes = call_api("keystore_common_delete", param).unwrap();
            let ret: Response = Response::decode(ret_bytes.as_slice()).unwrap();
            assert!(ret.is_success);

            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::PrivateKey as i32,
                value: "5JZc7wGRUr4J1RHDcM9ySWKLfQ2xjRUEo612qC4RLJ3G7jzJ4qx".to_string(),
            };

            let ret_bytes = call_api("keystore_common_exists", param).unwrap();
            let ret: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(ret_bytes.as_slice()).unwrap();

            assert_eq!(false, ret.is_exists);
        })
    }

    #[test]
    pub fn test_keystore_common_exists() {
        run_test(|| {
            let wallet = import_default_wallet();
            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::Mnemonic as i32,
                value: format!("{}", TEST_MNEMONIC).to_string(),
            };

            let ret_bytes = call_api("keystore_common_exists", param).unwrap();
            let result: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(ret_bytes.as_slice()).unwrap();
            assert!(result.is_exists);
            assert_eq!(result.id, wallet.id);

            let wallet = import_default_pk_store();
            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::PrivateKey as i32,
                value: "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB".to_string(),
            };

            let ret_bytes = call_api("keystore_common_exists", param).unwrap();
            let result: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(ret_bytes.as_slice()).unwrap();
            assert!(result.is_exists);
            assert_eq!(result.id, wallet.id);

            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::PrivateKey as i32,
                value: "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6"
                    .to_string(),
            };

            let ret_bytes = call_api("keystore_common_exists", param).unwrap();
            let result: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(ret_bytes.as_slice()).unwrap();
            assert!(result.is_exists);
            assert_eq!(result.id, wallet.id);
        })
    }

    #[test]
    pub fn test_keystore_common_accounts() {
        run_test(|| {
            let wallet = import_default_wallet();

            let param: KeystoreCommonAccountsParam = KeystoreCommonAccountsParam {
                id: wallet.id.to_string(),
            };

            let ret_bytes = call_api("keystore_common_accounts", param).unwrap();
            let result: AccountsResponse = AccountsResponse::decode(ret_bytes.as_slice()).unwrap();
            assert_eq!(0, result.accounts.len());

            let derivations = vec![Derivation {
                chain_type: "LITECOIN".to_string(),
                path: "m/44'/2'/0'/0/0".to_string(),
                network: "MAINNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            }];
            let param = KeystoreCommonDeriveParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes = call_api("keystore_common_derive", param).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes.as_slice()).unwrap();
            assert_eq!(1, derived_accounts.accounts.len());
            assert_eq!(
                "Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP",
                derived_accounts.accounts[0].address
            );
        })
    }

    #[test]
    pub fn test_sign_ckb_tx() {
        run_test(|| {
            let derivation = Derivation {
                chain_type: "NERVOS".to_string(),
                path: "m/44'/309'/0'/0/0".to_string(),
                network: "TESTNET".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };

            let wallet = import_and_derive(derivation);
            let out_points = vec![
                OutPoint {
                    tx_hash: "0xfb9c020db967e84af1fbd755df5bc23427e2ed70f73e07895a0c394f6195f083"
                        .to_owned(),
                    index: 0,
                },
                OutPoint {
                    tx_hash: "0xfb9c020db967e84af1fbd755df5bc23427e2ed70f73e07895a0c394f6195f083"
                        .to_owned(),
                    index: 1,
                },
            ];

            let code_hash =
                "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8".to_owned();

            let input = CkbTxInput {
                inputs: vec![
                    CellInput {
                        previous_output: Some(out_points[0].clone()),
                        since: "".to_string(),
                    },
                    CellInput {
                        previous_output: Some(out_points[1].clone()),
                        since: "".to_string(),
                    },
                ],
                witnesses: vec![Witness::default(), Witness::default()],
                cached_cells: vec![
                    CachedCell {
                        capacity: 0,
                        lock: Some(Script {
                            hash_type: "type".to_string(),
                            code_hash: code_hash.clone(),
                            args: "0xb45772677603bccc71194b2557067fb361c1e093".to_owned(),
                        }),
                        out_point: Some(out_points[0].clone()),
                        derived_path: "0/1".to_string(),
                    },
                    CachedCell {
                        capacity: 0,
                        lock: Some(Script {
                            hash_type: "type".to_string(),
                            code_hash: code_hash.clone(),
                            args: "0x2d79d9ed37184c1136bcfbe229947a137f80dec0".to_owned(),
                        }),
                        out_point: Some(out_points[1].clone()),
                        derived_path: "1/0".to_string(),
                    },
                ],
                tx_hash: "0x102b8e88daadf1b035577b4d5ea4f604be965df6a918e72daeff6c0c40753401"
                    .to_owned(),
            };

            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::Password(TEST_PASSWORD.to_string())),
                chain_type: "NERVOS".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input).unwrap(),
                }),
            };

            let ret = call_api("sign_tx", tx).unwrap();
            let output: CkbTxOutput = CkbTxOutput::decode(ret.as_slice()).unwrap();
            assert_eq!("0x5500000010000000550000005500000041000000776e010ac7e7166afa50fe54cfecf0a7106a2f11e8110e071ccab67cb30ed5495aa5c5f5ca2967a2fe4a60d5ad8c811382e51d8f916ba2911552bef6dedeca8a00", output.witnesses[0]);
            assert_eq!("0x5500000010000000550000005500000041000000914591d8abd5233740207337b0588fec58cad63143ddf204970526022b6db26d68311e9af49e1625e3a90e8a66eb1694632558d561d1e5d02cc7c7254e2d546100", output.witnesses[1]);

            remove_created_wallet(&wallet.id);
        })
    }

    #[test]
    pub fn test_sign_tron_tx() {
        run_test(|| {
            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "m/44'/195'/0'/0/0".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };

            let wallet = import_and_derive(derivation);

            let raw_data = "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d".to_string();
            let input = TronTxInput { raw_data };
            let input_value = encode_message(input).unwrap();
            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::Password("WRONG PASSWORD".to_string())),
                chain_type: "TRON".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value.clone(),
                }),
            };

            let ret = call_api("sign_tx", tx);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "password_incorrect");

            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::Password(TEST_PASSWORD.to_string())),
                chain_type: "TRON1".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value.clone(),
                }),
            };

            let ret = call_api("sign_tx", tx);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "unsupported_chain");

            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::Password(TEST_PASSWORD.to_string())),
                chain_type: "TRON".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value,
                }),
            };

            let ret = call_api("sign_tx", tx).unwrap();
            let output: TronTxOutput = TronTxOutput::decode(ret.as_slice()).unwrap();
            let expected_sign = "bbf5ce0549490613a26c3ac4fc8574e748eabda05662b2e49cea818216b9da18691e78cd6379000e9c8a35c13dfbf620f269be90a078b58799b56dc20da3bdf200";
            assert_eq!(expected_sign, output.signatures[0]);
            remove_created_wallet(&wallet.id);
        })
    }

    // #[test]
    // pub fn test_sign_substrate_tx() {
    //     run_test(|| {
    //         let derivation = Derivation {
    //             chain_type: "KUSAMA".to_string(),
    //             path: "//kusama//imToken/0".to_string(),
    //             network: "".to_string(),
    //             seg_wit: "".to_string(),
    //             chain_id: "".to_string(),
    //         };
    //
    //         let wallet = import_and_derive(derivation);
    //
    //         let input = SubstrateTxIn {
    //             method: "transfer".to_string(),
    //             address: "EwDXBhgNrcNvMVhm9fRq5YCTdAsPRBPo3t4tUZ85Q9ydKNs".to_string(),
    //             amount: 10000000000,
    //             era: Some(ExtrinsicEra {
    //                 current: 1202925,
    //                 period: 2400,
    //             }),
    //             nonce: 5,
    //             tip: 10000000000,
    //             sepc_version: 1045,
    //             genesis_hash: "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
    //                 .to_string(),
    //             block_hash: "790628ced8e0649883f3dd20344d9e6b014f076e788742f0925cf3875997e883"
    //                 .to_string(),
    //         };
    //
    //         let input_value = encode_message(input).unwrap();
    //         let tx = SignParam {
    //             id: wallet.id.to_string(),
    //             password: TEST_PASSWORD.to_string(),
    //             chain_type: "KUSAMA".to_string(),
    //             address: wallet.accounts.first().unwrap().address.to_string(),
    //             input: Some(::prost_types::Any {
    //                 type_url: "imtoken".to_string(),
    //                 value: input_value.clone(),
    //             }),
    //         };
    //
    //         let ret = call_api("sign_tx", tx).unwrap();
    //         let output: SubstrateTxOut = SubstrateTxOut::decode(&ret).unwrap();
    //
    //         let expected_ret_before_sig =
    //             "550284ffce9e36de55716d91b1c50caa36a58cee6d28e532a710df0cf90609363947dd7801";
    //         let expected_ret_after_sig = "dbae140700e40b54020400ff68686f29461fcc99ab3538c391e42556e49efc1ffa7933da42335aa626fae25a0700e40b5402";
    //
    //         assert_eq!(
    //             output.signature[0..74].to_string(),
    //             expected_ret_before_sig,
    //             "before sig"
    //         );
    //         assert_eq!(
    //             output.signature[202..].to_string(),
    //             expected_ret_after_sig,
    //             "after sig"
    //         );
    //
    //         let sig_bytes = hex::decode(output.signature[74..202].to_string()).unwrap();
    //         let signature = sp_core::sr25519::Signature::from_slice(&sig_bytes);
    //
    //         let pub_key =
    //             hex::decode("ce9e36de55716d91b1c50caa36a58cee6d28e532a710df0cf90609363947dd78")
    //                 .unwrap();
    //         let singer = sp_core::sr25519::Public::from_slice(&pub_key);
    //         let msg = hex::decode("0400ff68686f29461fcc99ab3538c391e42556e49efc1ffa7933da42335aa626fae25a0700e40b5402dbae140700e40b540215040000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe790628ced8e0649883f3dd20344d9e6b014f076e788742f0925cf3875997e883").unwrap();
    //
    //         assert!(
    //             sp_core::sr25519::Signature::verify(&signature, msg.as_slice(), &singer),
    //             "assert sig"
    //         );
    //
    //         remove_created_wallet(&wallet.id);
    //     })
    // }

    #[test]
    pub fn test_import_substrate_keystore() {
        run_test(|| {
            let keystore_str: &str = r#"{
  "address": "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS",
  "encoded": "0xf7e7e89d3016c9b4d93bb1129adf69e5949ca1fb58c29da4591ddc72c52238a35835e3f2ae023f9867ff301bc4132463527ac03525eaac54664a7cb658eae68a0bbc99354222c194d6100b2bf3a492639229077a2e2818d8196e002f0b5556104be23b11633858259dbbd3f91ea1d34d6ce182b62d8381af1ef3c35e9ab1583267cfa41aa58bfd64435c2b5047baf9052f0953d9f7854d2d396dfcad13",
  "encoding": {
    "content": [
      "pkcs8",
      "sr25519"
    ],
    "type": "xsalsa20-poly1305",
    "version": "2"
  },
  "meta": {
    "genesisHash": "0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
    "name": "keystore_import",
    "tags": [],
    "whenCreated": 1593591324334
  }
}"#;

            let param = ImportSubstrateKeystoreParam {
                keystore: keystore_str.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "KUSAMA".to_string(),
                r#override: true,
            };
            // let param_bytes = encode_message(param).unwrap();
            let ret_bytes = call_api("substrate_keystore_import", param).unwrap();
            let wallet_ret: WalletResult = WalletResult::decode(ret_bytes.as_slice()).unwrap();
            let derivation = Derivation {
                chain_type: "KUSAMA".to_string(),
                path: "".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };

            let param = KeystoreCommonDeriveParam {
                id: wallet_ret.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = call_api("keystore_common_derive", param).unwrap();
            let accounts: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();

            assert_eq!(
                accounts.accounts[0].address,
                "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS"
            );

            let export_param = ExportPrivateKeyParam {
                id: wallet_ret.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                chain_type: "KUSAMA".to_string(),
                network: "".to_string(),
                main_address: "JHBkzZJnLZ3S3HLvxjpFAjd6ywP7WAk5miL7MwVCn9a7jHS".to_string(),
                path: "".to_string(),
            };
            let ret = call_api("substrate_keystore_export", export_param).unwrap();
            let keystore_ret: ExportSubstrateKeystoreResult =
                ExportSubstrateKeystoreResult::decode(ret.as_slice()).unwrap();
            // assert_eq!(keystore_ret.keystore, "");
            remove_created_wallet(&wallet_ret.id);
        })
    }

    #[test]
    pub fn test_sign_substrate_raw_tx() {
        run_test(|| {
            let derivation = Derivation {
                chain_type: "KUSAMA".to_string(),
                path: "//kusama//imToken/0".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };

            let wallet = import_and_derive(derivation);

            let unsigned_msg = "0x0600ffd7568e5f0a7eda67a82691ff379ac4bba4f9c9b859fe779b5d46363b61ad2db9e56c0703d148e25901007b000000dcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025bde8f69eeb5e065e18c6950ff708d7e551f68dc9bf59a07c52367c0280f805ec7";
            let input = SubstrateRawTxIn {
                raw_data: unsigned_msg.to_string(),
            };

            let input_value = encode_message(input).unwrap();
            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::Password(TEST_PASSWORD.to_string())),
                chain_type: "KUSAMA".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value.clone(),
                }),
            };

            let ret = call_api("sign_tx", tx).unwrap();
            let output: SubstrateTxOut = SubstrateTxOut::decode(ret.as_slice()).unwrap();

            assert_eq!(output.signature[0..4].to_string(), "0x01",);

            let sig_bytes = hex::decode(output.signature[4..].to_string()).unwrap();
            let signature = sp_core::sr25519::Signature::from_slice(&sig_bytes);

            let pub_key =
                hex::decode("90742a577c8515391a46b7881c98c80ec92fe04255bb5b5fec862c7d633ada21")
                    .unwrap();
            let singer = sp_core::sr25519::Public::from_slice(&pub_key);
            let msg = hex::decode("0600ffd7568e5f0a7eda67a82691ff379ac4bba4f9c9b859fe779b5d46363b61ad2db9e56c0703d148e25901007b000000dcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025bde8f69eeb5e065e18c6950ff708d7e551f68dc9bf59a07c52367c0280f805ec7").unwrap();

            assert!(
                sp_core::sr25519::Signature::verify(&signature, msg.as_slice(), &singer),
                "assert sig"
            );

            remove_created_wallet(&wallet.id);
        })
    }

    #[test]
    pub fn test_sign_tron_tx_by_pk() {
        run_test(|| {
            let import_result = import_default_pk_store();

            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = call_api("keystore_common_derive", param).unwrap();
            let rsp: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();

            let raw_data = "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d".to_string();
            let input = TronTxInput { raw_data };
            let tx = SignParam {
                id: import_result.id.to_string(),
                key: Some(Key::Password(TEST_PASSWORD.to_string())),
                chain_type: "TRON".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input).unwrap(),
                }),
            };

            let ret = call_api("sign_tx", tx).unwrap();
            let output: TronTxOutput = TronTxOutput::decode(ret.as_slice()).unwrap();
            let expected_sign = "7758c92df76d50774a67fdca6c90b922fc84be68c69164d4c7f500327bfa4b9655709b6b1f88e07e3bda266d7ca4b48c934557917692f63a31e301d79d7107d001";
            assert_eq!(expected_sign, output.signatures[0]);
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_sign_by_dk_in_pk_store() {
        run_test(|| {
            let import_result = import_default_pk_store();

            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = call_api("keystore_common_derive", param).unwrap();
            let rsp: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();

            let param = WalletKeyParam {
                id: import_result.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };
            let ret_bytes = get_derived_key(&encode_message(param).unwrap()).unwrap();
            let ret: DerivedKeyResult = DerivedKeyResult::decode(ret_bytes.as_slice()).unwrap();
            let raw_data = "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d".to_string();
            let input = TronTxInput { raw_data };
            let tx = SignParam {
                id: import_result.id.to_string(),
                key: Some(Key::DerivedKey(ret.derived_key)),
                chain_type: "TRON".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input.clone()).unwrap(),
                }),
            };

            let ret = call_api("sign_tx", tx).unwrap();
            let output: TronTxOutput = TronTxOutput::decode(ret.as_slice()).unwrap();
            let expected_sign = "7758c92df76d50774a67fdca6c90b922fc84be68c69164d4c7f500327bfa4b9655709b6b1f88e07e3bda266d7ca4b48c934557917692f63a31e301d79d7107d001";
            assert_eq!(expected_sign, output.signatures[0]);

            let tx = SignParam {
                id: import_result.id.to_string(),
                key: Some(Key::DerivedKey("7758c92df76d50774a67fdca6c90b922fc84be68c69164d4c7f500327bfa4b9655709b6b1f88e07e3bda266d7ca4b48c934557917692f63a31e301d79d7107d001".to_string())),
                chain_type: "TRON".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input.clone()).unwrap(),
                }),
            };

            let ret = call_api("sign_tx", tx);
            assert!(ret.is_err());
            assert_eq!("derived_key_not_matched", format!("{}", ret.err().unwrap()));

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    fn test_sign_message() {
        run_test(|| {
            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "m/44'/195'/0'/0/0".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };
            let wallet = import_and_derive(derivation);

            let input_expects = vec![
                (TronMessageInput {
                    value: "645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76"
                        .to_string(),
                    is_hex: true,
                    is_tron_header: true,
                }, "16417c6489da3a88ef980bf0a42551b9e76181d03e7334548ab3cb36e7622a484482722882a29e2fe4587b95c739a68624ebf9ada5f013a9340d883f03fcf9af1b"),
                (TronMessageInput {
                    value: "0x645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76"
                        .to_string(),
                    is_hex: true,
                    is_tron_header: true,
                }, "16417c6489da3a88ef980bf0a42551b9e76181d03e7334548ab3cb36e7622a484482722882a29e2fe4587b95c739a68624ebf9ada5f013a9340d883f03fcf9af1b"),
                (TronMessageInput {
                    value: "645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76"
                        .to_string(),
                    is_hex: true,
                    is_tron_header: false,
                }, "06ff3c5f98b8e8e257f47a66ce8e953c7a7d0f96eb6687da6a98b66a36c2a725759cab3df94d014bd17760328adf860649303c68c4fa6644d9f307e2f32cc3311c"),
                (TronMessageInput {
                    value: "abcdef"
                        .to_string(),
                    is_hex: false,
                    is_tron_header: true,
                }, "a87eb6ae7e97621b6ba2e2f70db31fe0c744c6adcfdc005044026506b70ac11a33f415f4478b6cf84af32b3b5d70a13a77e53287613449b345bb16fe012c04081b"),
            ];
            for (input, expected) in input_expects {
                let tx = SignParam {
                    id: wallet.id.to_string(),
                    key: Some(Key::Password(TEST_PASSWORD.to_string())),
                    chain_type: "TRON".to_string(),
                    address: wallet.accounts.first().unwrap().address.to_string(),
                    input: Some(::prost_types::Any {
                        type_url: "imtoken".to_string(),
                        value: encode_message(input).unwrap(),
                    }),
                };

                let sign_result = call_api("tron_sign_msg", tx).unwrap();
                let ret: TronMessageOutput =
                    TronMessageOutput::decode(sign_result.as_slice()).unwrap();
                assert_eq!(expected, ret.signature);
            }
            //            let input = TronMessageInput {
            //                value: "645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76"
            //                    .to_string(),
            //                is_hex: true,
            //                is_tron_header: true,
            //            };
        });
    }

    #[test]
    fn test_sign_by_dk_hd_store() {
        run_test(|| {
            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "m/44'/195'/0'/0/0".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };
            let wallet = import_and_derive(derivation);

            let input = TronMessageInput {
                value: "645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76"
                    .to_string(),
                is_hex: true,
                is_tron_header: true,
            };

            let dk_param = WalletKeyParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };

            let ret_bytes = get_derived_key(&encode_message(dk_param).unwrap()).unwrap();
            let ret: DerivedKeyResult = DerivedKeyResult::decode(ret_bytes.as_slice()).unwrap();

            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::DerivedKey(ret.derived_key)),
                chain_type: "TRON".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input.clone()).unwrap(),
                }),
            };

            let sign_result = call_api("tron_sign_msg", tx).unwrap();
            let ret: TronMessageOutput = TronMessageOutput::decode(sign_result.as_slice()).unwrap();
            assert_eq!("16417c6489da3a88ef980bf0a42551b9e76181d03e7334548ab3cb36e7622a484482722882a29e2fe4587b95c739a68624ebf9ada5f013a9340d883f03fcf9af1b", ret.signature);

            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::DerivedKey("7758c92df76d50774a67fdca6c90b922fc84be68c69164d4c7f500327bfa4b9655709b6b1f88e07e3bda266d7ca4b48c934557917692f63a31e301d79d7107d001".to_string())),
                chain_type: "TRON".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input).unwrap(),
                }),
            };

            let ret = call_api("tron_sign_msg", tx);
            assert!(ret.is_err());
            assert_eq!("derived_key_not_matched", format!("{}", ret.err().unwrap()));

            remove_created_wallet(&wallet.id);
        });
    }

    #[test]
    pub fn test_sign_btc_fork_invalid_address() {
        run_test(|| {
            let chain_types = vec!["BITCOINCASH", "LITECOIN"];

            let import_result: WalletResult = import_default_wallet();

            for chain_type in chain_types {
                let derivation = Derivation {
                    chain_type: chain_type.to_string(),
                    path: "m/44'/0'/0'/0/0".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                    chain_id: "".to_string(),
                };
                let param = KeystoreCommonDeriveParam {
                    id: import_result.id.to_string(),
                    password: TEST_PASSWORD.to_string(),
                    derivations: vec![derivation],
                };

                let ret = call_api("keystore_common_derive", param).unwrap();
                let rsp: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();

                let unspents = vec![Utxo {
                    tx_hash: "a477af6b2667c29670467e4e0728b685ee07b240235771862318e29ddbe58458"
                        .to_string(),
                    vout: 0,
                    amount: 1000000,
                    address: "mszYqVnqKoQx4jcTdJXxwKAissE3Jbrrc1".to_string(),
                    script_pub_key: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac"
                        .to_string(),
                    derived_path: "0/0".to_string(),
                    sequence: 0,
                }];
                let tx_input = BtcForkTxInput {
                    to: "invalid_address".to_string(),
                    amount: 500000,
                    unspents,
                    fee: 100000,
                    change_address_index: 1u32,
                    change_address: "".to_string(),
                    network: "TESTNET".to_string(),
                    seg_wit: "NONE".to_string(),
                };
                let input_value = encode_message(tx_input).unwrap();
                let tx = SignParam {
                    id: import_result.id.to_string(),
                    key: Some(Key::Password(TEST_PASSWORD.to_string())),
                    chain_type: chain_type.to_string(),
                    address: rsp.accounts.first().unwrap().address.to_string(),
                    input: Some(::prost_types::Any {
                        type_url: "imtoken".to_string(),
                        value: input_value.clone(),
                    }),
                };

                let ret = call_api("sign_tx", tx);
                assert!(ret.is_err());
                assert_eq!(format!("{}", ret.err().unwrap()), "address_invalid");
            }

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_lock_after_sign() {
        run_test(|| {
            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "m/44'/195'/0'/0/0".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };

            let wallet = import_and_derive(derivation);

            let raw_data = "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d".to_string();
            let input = TronTxInput { raw_data };
            let input_value = encode_message(input).unwrap();

            let tx = SignParam {
                id: wallet.id.to_string(),
                key: Some(Key::Password(TEST_PASSWORD.to_string())),
                chain_type: "TRON".to_string(),
                address: wallet.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value,
                }),
            };
            {
                let map = KEYSTORE_MAP.read();
                let keystore: &Keystore = map.get(&wallet.id).unwrap();
                assert!(keystore.is_locked());
            }

            let ret = call_api("sign_tx", tx).unwrap();
            let output: TronTxOutput = TronTxOutput::decode(ret.as_slice()).unwrap();
            let expected_sign = "bbf5ce0549490613a26c3ac4fc8574e748eabda05662b2e49cea818216b9da18691e78cd6379000e9c8a35c13dfbf620f269be90a078b58799b56dc20da3bdf200";
            assert_eq!(expected_sign, output.signatures[0]);

            {
                let map = KEYSTORE_MAP.read();
                let keystore: &Keystore = map.get(&wallet.id).unwrap();
                assert!(keystore.is_locked());
            }

            remove_created_wallet(&wallet.id);
        })
    }

    #[test]
    fn test_get_derived_key() {
        let param = InitTokenCoreXParam {
            file_dir: "../test-data".to_string(),
            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
            is_debug: true,
        };

        handler::init_token_core_x(&encode_message(param).unwrap()).expect("should init tcx");

        let param = WalletKeyParam {
            id: "cb1ba2d7-7b89-4595-9753-d16b6e317c6b".to_string(),
            password: "WRONG PASSWORD".to_string(),
        };

        let ret = call_api("get_derived_key", param);
        assert!(ret.is_err());
        assert_eq!(format!("{}", ret.err().unwrap()), "password_incorrect");

        let param = WalletKeyParam {
            id: "cb1ba2d7-7b89-4595-9753-d16b6e317c6b".to_string(),
            password: TEST_PASSWORD.to_string(),
        };

        let ret = call_api("get_derived_key", param).unwrap();
        let dk_ret: DerivedKeyResult = DerivedKeyResult::decode(ret.as_slice()).unwrap();
        assert_eq!(dk_ret.derived_key, "119a38ab626aaf8806e223833b29da7aa1d0623e282164d1dd73b0b5e0a88fb4b88937efadd9ca9d4ee931d7b2b33594d75ac4f4d651602819998237b27860fa");
    }
    //
    //    #[test]
    //    fn test_export_used_dk() {
    //        let param = InitTokenCoreXParam {
    //            file_dir: "../test-data".to_string(),
    //            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
    //            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
    //            is_debug: true,
    //        };
    //
    //        handler::init_token_core_x(&encode_message(param).unwrap()).expect("should init tcx");
    //
    //        let param = PrivateKeyStoreExportParam {
    //            id: "cb1ba2d7-7b89-4595-9753-d16b6e317c6b".to_string(),
    //            password: "119a38ab626aaf8806e223833b29da7aa1d0623e282164d1dd73b0b5e0a88fb4b88937efadd9ca9d4ee931d7b2b33594d75ac4f4d651602819998237b27860fa".to_string(),
    //            chain_type: "TRON".to_string(),
    //            network: "".to_string()
    //        };
    //
    //        let ret = call_api("private_key_store_export", param).unwrap();
    //        let export_ret: KeystoreCommonExportResult =
    //            KeystoreCommonExportResult::decode(ret.as_slice()).unwrap();
    //        assert_eq!(
    //            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
    //            export_ret.value
    //        );
    //    }

    #[test]
    fn test_panic_keystore_locked() {
        run_test(|| {
            let wallet = import_default_wallet();
            let param = WalletKeyParam {
                id: wallet.id.to_string(),
                password: TEST_PASSWORD.to_string(),
            };
            let _ret = call_api("unlock_then_crash", param);
            let err = unsafe { _to_str(get_last_err_message()) };
            let err_bytes = hex::decode(err).unwrap();
            let rsp: Response = Response::decode(err_bytes.as_slice()).unwrap();
            assert!(!rsp.is_success);
            assert_eq!(rsp.error, "test_unlock_then_crash");
            let map = KEYSTORE_MAP.read();
            let keystore: &Keystore = map.get(&wallet.id).unwrap();
            assert!(keystore.is_locked())
        });
    }

    fn remove_created_wallet(wid: &str) {
        let full_file_path = format!("{}/{}.json", "/tmp/imtoken/wallets", wid);
        let p = Path::new(&full_file_path);
        remove_file(p).expect("should remove file");
    }
}
