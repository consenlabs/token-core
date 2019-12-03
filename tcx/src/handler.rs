use std::fs;
use std::io::Read;
use std::path::Path;

use bytes::BytesMut;
use prost::Message;
use serde_json::Value;

use tcx_bch::{BchAddress, BchExtra, BchTransaction};
use tcx_btc_fork::{
    address::BtcForkAddress, BtcForkExtra, BtcForkSegWitTransaction, BtcForkSignedTxOutput,
    BtcForkTransaction, BtcForkTxInput,
};
use tcx_chain::keystore::EmptyExtra;
use tcx_chain::keystore_guard::KeystoreGuard;
use tcx_chain::{HdKeystore, Metadata, Source};
use tcx_constants::coin_info::{coin_info_from_symbol, coin_symbol_with_param};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_tron::TrxAddress;

use crate::api::{HdStoreImportParam, WalletResult};
use crate::api::{InitTokenCoreXParam, SignTxParam};
use crate::error_handling::Result;
use crate::filemanager::KEYSTORE_MAP;
use crate::filemanager::{
    cache_keystore, find_keystore_id_by_address, flush_keystore, WALLET_FILE_DIR,
};
use tcx_chain::signer::TransactionSigner;

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: usize,
}

pub fn encode_message(msg: impl Message) -> Result<Vec<u8>> {
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}

pub fn hd_keystore_create(data: &[u8]) -> Result<Vec<u8>> {
    let buf = BytesMut::with_capacity(0);
    Ok(buf.to_vec())
}

pub fn init_token_core_x(data: &[u8]) -> Result<()> {
    let InitTokenCoreXParam {
        file_dir,
        xpub_common_key,
        xpub_common_iv,
    } = InitTokenCoreXParam::decode(data).unwrap();
    *WALLET_FILE_DIR.write().unwrap() = file_dir.to_string();
    *XPUB_COMMON_KEY_128.write().unwrap() = xpub_common_key.to_string();
    *XPUB_COMMON_IV.write().unwrap() = xpub_common_iv.to_string();

    let p = Path::new(&file_dir);
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

        let mut f = fs::File::open(fp).expect("open file");
        let mut contents = String::new();

        let _ = f.read_to_string(&mut contents);
        let v: Value = serde_json::from_str(&contents).expect("read json from content");

        let version = v["version"].as_i64().expect("version");
        if version != i64::from(HdKeystore::VERSION) {
            continue;
        }
        let keystore: HdKeystore = serde_json::from_str(&contents)?;
        cache_keystore(keystore);
    }
    Ok(())
}

pub fn hd_store_import(data: &[u8]) -> Result<Vec<u8>> {
    let param: HdStoreImportParam =
        HdStoreImportParam::decode(data).expect("import wallet from mnemonic");
    let symbol = coin_symbol_with_param(&param.chain_type, &param.network, "", &param.seg_wit);

    let mut meta = Metadata::default();
    meta.name = param.name.to_owned();
    meta.password_hint = param.password_hint.to_owned();
    meta.source = Source::Mnemonic;

    //    let meta: Metadata = serde_json::from_value(v.clone())?;
    let mut ks = HdKeystore::from_mnemonic(&param.mnemonic, &param.password, meta);

    {
        let mut guard_mut = KeystoreGuard::unlock_by_password(&mut ks, &param.password)?;

        let mut coin_info = coin_info_from_symbol(&symbol)?;
        coin_info.derivation_path = param.path.to_string();
        let account = match symbol.as_str() {
            "BITCOINCASH" | "BITCOINCASH-TESTNET" => guard_mut
                .keystore_mut()
                .derive_coin::<BchAddress, BchExtra>(&coin_info),
            "LITECOIN" | "LITECOIN-P2WPKH" | "LITECOIN-TESTNET" | "LITECOIN-TESTNET-P2WPKH" => {
                guard_mut
                    .keystore_mut()
                    .derive_coin::<BtcForkAddress, BtcForkExtra>(&coin_info)
            }
            "TRON" => guard_mut
                .keystore_mut()
                .derive_coin::<TrxAddress, EmptyExtra>(&coin_info),
            _ => Err(format_err!("{}", "chain_type_not_support")),
        }?;

        let exist_kid_opt = find_keystore_id_by_address(&account.address);
        if let Some(exist_kid) = exist_kid_opt {
            if !param.overwrite {
                return Err(format_err!("{}", "wallet_exists"));
            } else {
                guard_mut.keystore_mut().id = exist_kid;
            }
        }
    }

    let json = format!("{}", ks);
    println!("ks right: {}", json);
    flush_keystore(&ks)?;

    let extra = ::prost_types::Any {
        type_url: "imToken.api.ImportWalletFromMnemonic".to_owned(),
        value: vec![],
    };
    let wallet = WalletResult {
        id: ks.id.to_owned(),
        name: ks.meta.name.to_owned(),
        chain_type: param.chain_type.to_owned(),
        address: ks.active_accounts.first().unwrap().address.to_owned(),
        source: "MNEMONIC".to_owned(),
        created_at: ks.meta.timestamp.clone(),
        extra: Some(extra),
    };
    let mut buf = BytesMut::with_capacity(wallet.encoded_len() * 3);
    wallet.encode_raw(&mut buf);
    cache_keystore(ks.clone());
    println!("raw result: {}", hex::encode(buf.clone()));
    Ok(buf.to_vec())
}

pub fn sign_tx(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignTxParam = SignTxParam::decode(data).expect("SignTxParam");

    let mut map = KEYSTORE_MAP.write().unwrap();
    let keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;
    match param.chain_type.as_str() {
        "BITCOINCASH" | "LITECOIN" => sign_btc_fork_transaction(&param, &guard),
        _ => Err(format_err!("unsupported_chain")),
    }
}

pub fn sign_btc_fork_transaction(param: &SignTxParam, guard: &KeystoreGuard) -> Result<Vec<u8>> {
    let input: BtcForkTxInput =
        BtcForkTxInput::decode(&param.tx_input.as_ref().expect("tx_input").value.clone())
            .expect("BitcoinForkTransactionInput");

    let signed_tx: BtcForkSignedTxOutput = if param.chain_type.as_str() == "BITCOINCASH" {
        let tran = BchTransaction::new(input, "BITCOINCASH".to_owned());
        guard.keystore().sign_transaction(&tran)?
    } else if input.seg_wit.as_str() != "NONE" {
        let tran = BtcForkSegWitTransaction::new(input, "LITECOIN-P2WPKH".to_owned());
        guard.keystore().sign_transaction(&tran)?
    } else {
        let tran = BtcForkTransaction::new(input, "LITECOIN".to_owned());
        guard.keystore().sign_transaction(&tran)?
    };
    encode_message(signed_tx)
}

#[cfg(test)]
mod tests {
    use crate::api::{InitTokenCoreXParam, SignTxParam};
    use crate::handler::init_token_core_x;
    use crate::handler::{encode_message, sign_tx};
    use prost::Message;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use std::panic;
    use std::path::Path;
    use tcx_btc_fork::{BtcForkSignedTxOutput, BtcForkTxInput, Utxo};

    fn setup() {
        let param = InitTokenCoreXParam {
            file_dir: "../test-data".to_string(),
            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
        };

        unsafe {
            init_token_core_x(&encode_message(param).unwrap());
        }
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();
        let result = panic::catch_unwind(|| test());
        //        teardown();
        assert!(result.is_ok())
    }

    #[test]
    pub fn test_sign_tx() {
        run_test(|| {
            let utxo = Utxo {
                tx_hash: "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2"
                    .to_string(),
                vout: 0,
                amount: 100000,
                address: "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r".to_string(),
                script_pub_key: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac".to_string(),
                derived_path: "0/0".to_string(),
                sequence: 0,
            };
            let input = BtcForkTxInput {
                to: "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s".to_string(),
                amount: 93454,
                unspents: vec![utxo],
                memo: "".to_string(),
                fee: 6000,
                change_idx: 1,
                change_address: "".to_string(),
                network: "MAINNET".to_owned(),
                seg_wit: "NONE".to_owned(),
            };
            let tx = SignTxParam {
                id: "9c6cbc21-1c43-4c8b-bb7a-5e538f908819".to_string(),
                password: "Insecure Password".to_string(),
                chain_type: "BITCOINCASH".to_string(),
                tx_input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input).unwrap(),
                }),
            };
            let tx_bytes = encode_message(tx).unwrap();
            let ret = sign_tx(&tx_bytes).unwrap();
            let output: BtcForkSignedTxOutput = BtcForkSignedTxOutput::decode(&ret).unwrap();
            assert_eq!("0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006b483045022100b3d91f406cdc33eb4d8f2b56491e6c87da2372eb83f1f384fc3f02f81a5b21b50220324dd7ecdc214721c542db252078473f9e7172bf592fa55332621c3e348be45041210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a9148835a675efb0db4fd00e9eb77aff38a6d5bd767c88ac00000000", output.signature);
        })
    }
}
