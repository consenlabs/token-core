use std::fs;
use std::io::Read;
use std::path::Path;

use bytes::BytesMut;
use prost::Message;
use serde_json::Value;

use tcx_bch::{BchAddress, BchExtra};
use tcx_btc_fork::{address::BtcForkAddress, BtcForkExtra};
use tcx_chain::keystore::EmptyExtra;
use tcx_chain::keystore_guard::KeystoreGuard;
use tcx_chain::{HdKeystore, Metadata, Source};
use tcx_constants::coin_info::{coin_info_from_symbol, coin_symbol_with_param};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_tron::TrxAddress;

use crate::api::InitTokenCoreXParam;
use crate::api::{ImportWalletFromMnemonicParam, WalletResult};
use crate::error_handling::Result;
use crate::filemanager::{
    cache_keystore, find_keystore_id_by_address, flush_keystore, WALLET_FILE_DIR,
};

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: usize,
}

pub fn hd_keystore_create(data: &[u8]) -> Result<Vec<u8>> {
    let buf = BytesMut::with_capacity(0);
    Ok(buf.to_vec())
}

pub fn init_token_core_x_pb_internal(buf: Buffer) -> Result<()> {
    //    let file_dir = v["fileDir"].as_str().expect("fileDir");
    //    let xpub_common_key = v["xpubCommonKey128"].as_str().expect("XPubCommonKey128");
    //    let xpub_common_iv = v["xpubCommonIv"].as_str().expect("xpubCommonIv");
    //    let param = prost::de
    //    let param: InitTokenCoreXParam = InitTokenCoreXParam::decode(&param).unwrap();
    let data = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
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

//#[no_mangle]
//pub unsafe extern "C" fn import_wallet_from_mnemonic_pb(buf: Buffer) -> *const c_char {
//    let data = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
//    let json = landingpad(|| import_wallet_from_mnemonic_pb_internal(data));
//    CString::new(json).expect("ret json").into_raw()
//}
//
pub fn import_wallet_from_mnemonic_pb_internal(data: &[u8]) -> Result<Vec<u8>> {
    //    let password = v["password"].as_str().unwrap();
    //    let mnemonic = v["mnemonic"].as_str().unwrap();
    //    let path = v["path"].as_str().unwrap();
    //    let overwrite = v["overwrite"].as_bool().unwrap();
    let param: ImportWalletFromMnemonicParam =
        ImportWalletFromMnemonicParam::decode(data).expect("import wallet from mnemonic");
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
