use std::fs;
use std::io::Read;
use std::path::Path;

use bytes::BytesMut;
use prost::Message;
use serde_json::Value;
use tcx_primitive::{private_key_without_version, FromHex, TypedPrivateKey};

use tcx_bch::{BchAddress, BchTransaction};
use tcx_btc_fork::{
    BtcForkAddress, BtcForkSegWitTransaction, BtcForkSignedTxOutput, BtcForkTransaction,
    BtcForkTxInput, WifDisplay,
};
use tcx_chain::{key_hash_from_mnemonic, key_hash_from_private_key, Keystore};
use tcx_chain::{Account, HdKeystore, Metadata, PrivateKeystore, Source};
use tcx_ckb::{CkbAddress, CkbTxInput};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_tron::TrxAddress;

use crate::api::keystore_common_derive_param::Derivation;
use crate::api::{
    AccountResponse, AccountsResponse, HdStoreCreateParam, HdStoreImportParam, KeyType,
    KeystoreCommonAccountsParam, KeystoreCommonDeriveParam, KeystoreCommonExistsParam,
    KeystoreCommonExistsResult, KeystoreCommonExportResult, PrivateKeyStoreExportParam,
    PrivateKeyStoreImportParam, Response, WalletKeyParam, WalletResult,
};
use crate::api::{InitTokenCoreXParam, SignParam};
use crate::error_handling::Result;
use crate::filemanager::{cache_keystore, clean_keystore, flush_keystore, WALLET_FILE_DIR};
use crate::filemanager::{delete_keystore_file, KEYSTORE_MAP};

use crate::IS_DEBUG;
use tcx_chain::Address;
use tcx_chain::{MessageSigner, TransactionSigner};
use tcx_constants::coin_info::coin_info_from_param;
use tcx_constants::CurveType;
use tcx_crypto::aes::cbc::encrypt_pkcs7;
use tcx_primitive::{Bip32DeterministicPublicKey, Ss58Codec};
use tcx_tron::transaction::{TronMessageInput, TronTxInput};

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: i64,
}

pub fn encode_message(msg: impl Message) -> Result<Vec<u8>> {
    if *IS_DEBUG.read() {
        println!("{:#?}", msg);
    }
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}

fn derive_account<'a, 'b>(keystore: &mut Keystore, derivation: &Derivation) -> Result<Account> {
    let mut coin_info = coin_info_from_param(
        &derivation.chain_type,
        &derivation.network,
        &derivation.seg_wit,
    )?;
    coin_info.derivation_path = derivation.path.to_owned();

    match derivation.chain_type.as_str() {
        "BITCOINCASH" => keystore.derive_coin::<BchAddress>(&coin_info),
        "LITECOIN" => keystore.derive_coin::<BtcForkAddress>(&coin_info),
        "TRON" => keystore.derive_coin::<TrxAddress>(&coin_info),
        "NERVOS" => keystore.derive_coin::<CkbAddress>(&coin_info),
        _ => Err(format_err!("unsupported_chain")),
    }
}

pub fn init_token_core_x(data: &[u8]) -> Result<()> {
    let InitTokenCoreXParam {
        file_dir,
        xpub_common_key,
        xpub_common_iv,
    } = InitTokenCoreXParam::decode(data).unwrap();
    *WALLET_FILE_DIR.write() = file_dir.to_string();
    *XPUB_COMMON_KEY_128.write() = xpub_common_key.to_string();
    *XPUB_COMMON_IV.write() = xpub_common_iv.to_string();

    scan_keystores()?;

    Ok(())
}

pub fn scan_keystores() -> Result<()> {
    clean_keystore();
    let file_dir = WALLET_FILE_DIR.read();
    let p = Path::new(file_dir.as_str());
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
        if version == i64::from(HdKeystore::VERSION)
            || version == i64::from(PrivateKeystore::VERSION)
        {
            let keystore = Keystore::from_json(&contents)?;
            cache_keystore(keystore);
        }
    }
    Ok(())
}

pub fn hd_store_create(data: &[u8]) -> Result<Vec<u8>> {
    let param: HdStoreCreateParam =
        HdStoreCreateParam::decode(data).expect("import wallet from mnemonic");

    let mut meta = Metadata::default();
    meta.name = param.name.to_owned();
    meta.password_hint = param.password_hint.to_owned();
    meta.source = Source::Mnemonic;

    let ks = HdKeystore::new(&param.password, meta);

    let keystore = Keystore::Hd(ks);
    flush_keystore(&keystore)?;

    let meta = keystore.meta();
    let wallet = WalletResult {
        id: keystore.id(),
        name: meta.name.to_owned(),
        source: "MNEMONIC".to_owned(),
        accounts: vec![],
        created_at: meta.timestamp.clone(),
    };
    let ret = encode_message(wallet)?;
    cache_keystore(keystore);
    Ok(ret)
}

pub fn hd_store_import(data: &[u8]) -> Result<Vec<u8>> {
    let param: HdStoreImportParam =
        HdStoreImportParam::decode(data).expect("import wallet from mnemonic");

    let mut founded_id: Option<String> = None;
    {
        let key_hash = key_hash_from_mnemonic(&param.mnemonic)?;
        let map = KEYSTORE_MAP.read();
        if let Some(founded) = map
            .values()
            .find(|keystore| keystore.key_hash() == key_hash)
        {
            founded_id = Some(founded.id());
        }
    }

    if founded_id.is_some() && !param.overwrite {
        return Err(format_err!("{}", "wallet_exists"));
    }

    let mut meta = Metadata::default();
    meta.name = param.name.to_owned();
    meta.password_hint = param.password_hint.to_owned();
    meta.source = Source::Mnemonic;

    let ks = HdKeystore::from_mnemonic(&param.mnemonic, &param.password, meta)?;

    let mut keystore = Keystore::Hd(ks);

    if founded_id.is_some() {
        keystore.set_id(&founded_id.unwrap());
    }

    flush_keystore(&keystore)?;

    let meta = keystore.meta();
    let wallet = WalletResult {
        id: keystore.id(),
        name: meta.name.to_owned(),
        source: "MNEMONIC".to_owned(),
        accounts: vec![],
        created_at: meta.timestamp.clone(),
    };
    let ret = encode_message(wallet)?;
    cache_keystore(keystore);
    Ok(ret)
}

fn enc_xpub(xpub: &str, network: &str) -> Result<String> {
    let xpk = Bip32DeterministicPublicKey::from_hex(xpub)?;
    let ext_pub_key: String;
    if network == "MAINNET" {
        ext_pub_key = xpk.to_ss58check_with_version(&[0x04, 0x88, 0xB2, 0x1E]);
    } else {
        ext_pub_key = xpk.to_ss58check_with_version(&[0x04, 0x35, 0x87, 0xCF]);
    }

    let key = tcx_crypto::XPUB_COMMON_KEY_128.read();
    let iv = tcx_crypto::XPUB_COMMON_IV.read();
    let key_bytes = hex::decode(&*key)?;
    let iv_bytes = hex::decode(&*iv)?;
    let encrypted = encrypt_pkcs7(&ext_pub_key.as_bytes(), &key_bytes, &iv_bytes)?;
    Ok(base64::encode(&encrypted))
}

pub fn keystore_common_derive(data: &[u8]) -> Result<Vec<u8>> {
    let param: KeystoreCommonDeriveParam =
        KeystoreCommonDeriveParam::decode(data).expect("keystore_common_derive");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    keystore.unlock_by_password(&param.password)?;

    let mut account_responses: Vec<AccountResponse> = vec![];

    for derivation in param.derivations {
        let account = derive_account(keystore, &derivation)?;
        let enc_xpub = if account.ext_pub_key.is_empty() {
            Ok("".to_string())
        } else {
            enc_xpub(&account.ext_pub_key.to_string(), &account.network)
        }?;
        let account_rsp = AccountResponse {
            chain_type: derivation.chain_type.to_owned(),
            address: account.address.to_owned(),
            path: account.derivation_path.to_owned(),
            extended_xpub_key: enc_xpub,
        };
        account_responses.push(account_rsp);
    }

    let accounts_rsp = AccountsResponse {
        accounts: account_responses,
    };
    flush_keystore(keystore)?;
    encode_message(accounts_rsp)
}

pub fn hd_store_export(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).expect("keystore_common_delete");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    keystore.unlock_by_password(&param.password)?;

    let export_result = KeystoreCommonExportResult {
        id: keystore.id(),
        r#type: KeyType::Mnemonic as i32,
        value: keystore.export()?,
    };

    encode_message(export_result)
}

fn key_data_from_any_format_pk(pk: &str) -> Result<Vec<u8>> {
    let decoded = hex::decode(pk.to_string());
    if decoded.is_ok() {
        Ok(decoded.unwrap())
    } else {
        private_key_without_version(pk)
    }
}

fn key_hash_from_any_format_pk(pk: &str) -> Result<String> {
    let key_data = key_data_from_any_format_pk(pk)?;
    Ok(key_hash_from_private_key(&key_data))
}

pub fn private_key_store_import(data: &[u8]) -> Result<Vec<u8>> {
    let param: PrivateKeyStoreImportParam =
        PrivateKeyStoreImportParam::decode(data).expect("private_key_store_import");

    let mut founded_id: Option<String> = None;
    {
        let key_hash = key_hash_from_any_format_pk(&param.private_key)?;
        let map = KEYSTORE_MAP.read();
        if let Some(founded) = map
            .values()
            .find(|keystore| keystore.key_hash() == key_hash)
        {
            founded_id = Some(founded.id());
        }
    }

    if founded_id.is_some() && !param.overwrite {
        return Err(format_err!("{}", "wallet_exists"));
    }

    let pk_bytes = key_data_from_any_format_pk(&param.private_key)?;
    let private_key = hex::encode(pk_bytes);
    let pk_store =
        PrivateKeystore::from_private_key(&private_key, &param.password, Source::Private);

    let mut keystore = Keystore::PrivateKey(pk_store);

    keystore.unlock_by_password(&param.password)?;

    if let Some(exist_kid) = founded_id {
        keystore.set_id(&exist_kid)
    }

    flush_keystore(&keystore)?;

    let meta = keystore.meta();
    let wallet = WalletResult {
        id: keystore.id(),
        name: meta.name.to_owned(),
        source: "PRIVATE".to_owned(),
        accounts: vec![],
        created_at: meta.timestamp.clone(),
    };
    let ret = encode_message(wallet)?;
    cache_keystore(keystore);
    Ok(ret)
}

pub fn private_key_store_export(data: &[u8]) -> Result<Vec<u8>> {
    let param: PrivateKeyStoreExportParam =
        PrivateKeyStoreExportParam::decode(data).expect("private_key_store_export");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    keystore.unlock_by_password(&param.password);

    let pk_hex = keystore.export()?;

    // private_key prefix is only about chain type and network
    let coin_info = coin_info_from_param(&param.chain_type, &param.network, "")?;
    let value = if param.chain_type.as_str() == "TRON" {
        Ok(pk_hex.to_string())
    } else {
        let bytes = hex::decode(pk_hex.to_string())?;
        let typed_pk = TypedPrivateKey::from_slice(CurveType::SECP256k1, &bytes)?;
        typed_pk.fmt(&coin_info)
    }?;

    let export_result = KeystoreCommonExportResult {
        id: keystore.id(),
        r#type: KeyType::PrivateKey as i32,
        value,
    };

    encode_message(export_result)
}

pub fn keystore_common_verify(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).expect("keystore_common_delete");
    let map = KEYSTORE_MAP.read();
    let keystore: &Keystore = match map.get(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    if keystore.verify_password(&param.password) {
        let rsp = Response {
            is_success: true,
            error: "".to_owned(),
        };
        encode_message(rsp)
    } else {
        Err(format_err!("{}", "password_incorrect"))
    }
}

pub fn keystore_common_delete(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).expect("keystore_common_delete");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &Keystore = match map.get(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    if keystore.verify_password(&param.password) {
        delete_keystore_file(&param.id)?;
        map.remove(&param.id);

        let rsp = Response {
            is_success: true,
            error: "".to_owned(),
        };
        encode_message(rsp)
    } else {
        Err(format_err!("{}", "password_incorrect"))
    }
}

pub fn keystore_common_exists(data: &[u8]) -> Result<Vec<u8>> {
    let param: KeystoreCommonExistsParam =
        KeystoreCommonExistsParam::decode(data).expect("keystore_common_exists params");
    let key_hash: String;
    if param.r#type == KeyType::Mnemonic as i32 {
        key_hash = key_hash_from_mnemonic(&param.value)?;
    } else {
        key_hash = key_hash_from_any_format_pk(&param.value)?;
    }
    let map = &mut KEYSTORE_MAP.write();

    let founded: Option<&Keystore> = map
        .values()
        .find(|keystore| keystore.key_hash() == key_hash);
    let result: KeystoreCommonExistsResult;
    if let Some(ks) = founded {
        result = KeystoreCommonExistsResult {
            is_exists: true,
            id: ks.id(),
        }
    } else {
        result = KeystoreCommonExistsResult {
            is_exists: false,
            id: "".to_owned(),
        }
    }
    encode_message(result)
}

pub fn keystore_common_accounts(data: &[u8]) -> Result<Vec<u8>> {
    let param: KeystoreCommonAccountsParam =
        KeystoreCommonAccountsParam::decode(data).expect("keystore_common_accounts params");
    let map = KEYSTORE_MAP.read();
    let keystore: &Keystore = match map.get(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let mut accounts: Vec<AccountResponse> = vec![];
    for account in keystore.accounts() {
        let enc_xpub = if account.ext_pub_key.is_empty() {
            "".to_string()
        } else {
            enc_xpub(&account.ext_pub_key, &account.network)?
        };
        // let enc_xpub = enc_xpub(&account.ext_pub_key, &account.network)?;
        let acc_rsp = AccountResponse {
            chain_type: account.coin.to_owned(),
            address: account.address.to_owned(),
            path: account.derivation_path.to_owned(),
            extended_xpub_key: enc_xpub.to_owned(),
        };
        accounts.push(acc_rsp);
    }

    let accounts_rsp = AccountsResponse { accounts };
    encode_message(accounts_rsp)
}

pub fn sign_tx(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignParam = SignParam::decode(data).expect("SignTxParam");

    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    keystore.unlock_by_password(&param.password)?;
    match param.chain_type.as_str() {
        "BITCOINCASH" | "LITECOIN" => sign_btc_fork_transaction(&param, keystore),
        "TRON" => sign_tron_tx(&param, keystore),
        "NERVOS" => sign_nervos_ckb(&param, keystore),
        _ => Err(format_err!("unsupported_chain")),
    }
}

pub fn sign_btc_fork_transaction(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: BtcForkTxInput =
        BtcForkTxInput::decode(&param.input.as_ref().expect("tx_input").value.clone())
            .expect("BitcoinForkTransactionInput");
    let coin = coin_info_from_param(&param.chain_type, &input.network, &input.seg_wit)?;

    let signed_tx: BtcForkSignedTxOutput = if param.chain_type.as_str() == "BITCOINCASH" {
        if !BchAddress::is_valid(&input.to, &coin) {
            return Err(format_err!("invalid_to_address"));
        }
        let tran = BchTransaction::new(input, coin);
        keystore.sign_transaction(&param.chain_type, &param.address, &tran)?
    } else if input.seg_wit.as_str() != "NONE" {
        if !BtcForkAddress::is_valid(&input.to, &coin) {
            return Err(format_err!("invalid_to_address"));
        }
        let tran = BtcForkSegWitTransaction::new(input, coin);
        keystore.sign_transaction(&param.chain_type, &param.address, &tran)?
    } else {
        if !BtcForkAddress::is_valid(&input.to, &coin) {
            return Err(format_err!("invalid_to_address"));
        }
        let tran = BtcForkTransaction::new(input, coin);
        keystore.sign_transaction(&param.chain_type, &param.address, &tran)?
    };
    encode_message(signed_tx)
}

pub fn sign_nervos_ckb(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: CkbTxInput =
        CkbTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("CkbTxInput");
    let signed_tx = keystore.sign_transaction(&param.chain_type, &param.address, &input)?;
    encode_message(signed_tx)
}

pub fn sign_tron_tx(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: TronTxInput =
        TronTxInput::decode(&param.input.as_ref().expect("tx_input").value.clone())
            .expect("TronTxInput");
    let signed_tx = keystore.sign_transaction(&param.chain_type, &param.address, &input)?;

    encode_message(signed_tx)
}

pub fn tron_sign_message(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignParam = SignParam::decode(data).expect("SignParam");

    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    //    let guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;
    keystore.unlock_by_password(&param.password)?;
    let input: TronMessageInput =
        TronMessageInput::decode(param.input.expect("TronMessageInput").value.clone())
            .expect("TronMessageInput");
    let signed_tx = keystore.sign_message(&param.chain_type, &param.address, &input)?;
    encode_message(signed_tx)
}

#[cfg(test)]
mod tests {
    use crate::api::keystore_common_derive_param::Derivation;
    use crate::api::{
        AccountsResponse, HdStoreCreateParam, HdStoreImportParam, InitTokenCoreXParam, KeyType,
        KeystoreCommonAccountsParam, KeystoreCommonDeriveParam, KeystoreCommonExistsParam,
        KeystoreCommonExistsResult, KeystoreCommonExportResult, PrivateKeyStoreExportParam,
        PrivateKeyStoreImportParam, Response, SignParam, WalletKeyParam, WalletResult,
    };
    use crate::filemanager::KEYSTORE_MAP;
    use crate::handler::{
        encode_message, hd_store_create, hd_store_export, keystore_common_accounts,
        keystore_common_delete, keystore_common_derive, keystore_common_exists,
        keystore_common_verify, private_key_store_export, private_key_store_import, scan_keystores,
        sign_tx,
    };
    use crate::handler::{hd_store_import, init_token_core_x};
    use prost::Message;

    use std::fs::remove_file;
    use std::path::Path;

    use std::{fs, panic};
    use tcx_btc_fork::transaction::BtcForkTxInput;
    use tcx_btc_fork::transaction::Utxo;

    use tcx_ckb::{CachedCell, CellInput, CkbTxInput, CkbTxOutput, OutPoint, Script, Witness};
    use tcx_tron::transaction::{TronTxInput, TronTxOutput};

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    static OTHER_MNEMONIC: &'static str =
        "calm release clay imitate top extend close draw quiz refuse shuffle injury";

    fn setup() {
        let p = Path::new("/tmp/imtoken/wallets");
        if !p.exists() {
            fs::create_dir_all(p).expect("shoud create filedir");
        }

        *tcx_crypto::KDF_ROUNDS.write() = 1024;
        let param = InitTokenCoreXParam {
            file_dir: "/tmp/imtoken/wallets".to_string(),
            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
        };

        init_token_core_x(&encode_message(param).unwrap()).expect("should init tcx");
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

    #[test]
    pub fn test_scan_keystores() {
        let param = InitTokenCoreXParam {
            file_dir: "../test-data".to_string(),
            xpub_common_key: "B888D25EC8C12BD5043777B1AC49F872".to_string(),
            xpub_common_iv: "9C0C30889CBCC5E01AB5B2BB88715799".to_string(),
        };

        init_token_core_x(&encode_message(param).unwrap()).expect("should init tcx");
        let mut keystore_count;
        {
            let mut map = KEYSTORE_MAP.write();
            keystore_count = map.len();
            map.clear();
            assert_eq!(0, map.len());
        }
        scan_keystores().expect("should rescan keystores");
        {
            let map = KEYSTORE_MAP.write();

            assert_eq!(keystore_count, map.len());
        }
    }

    #[test]
    pub fn test_hd_store_create() {
        run_test(|| {
            let param = HdStoreCreateParam {
                password: PASSWORD.to_string(),
                password_hint: "".to_string(),
                name: "aaa".to_string(),
            };
            let ret = hd_store_create(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

            assert!(import_result.accounts.is_empty());
            assert_eq!(import_result.name, "aaa");
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_hd_store_import() {
        run_test(|| {
            let param = HdStoreImportParam {
                mnemonic: MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };

            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

            let derivation = Derivation {
                chain_type: "BITCOINCASH".to_string(),
                path: "m/44'/145'/0'/0/0".to_string(),
                network: "MAINNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let result: AccountsResponse = AccountsResponse::decode(&ret).unwrap();
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
                    password: PASSWORD.to_string(),
                    source: "MNEMONIC".to_string(),
                    name: "test-wallet".to_string(),
                    password_hint: "imtoken".to_string(),
                    overwrite: true,
                };

                let ret = hd_store_import(&encode_message(param).unwrap());
                assert!(ret.is_err());
            }
        })
    }

    #[test]
    pub fn test_hd_store_import_ltc() {
        run_test(|| {
            let param = HdStoreImportParam {
                mnemonic: MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };

            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

            let derivation = Derivation {
                chain_type: "LITECOIN".to_string(),
                path: "m/44'/1'/0'/0/0".to_string(),
                network: "TESTNET".to_string(),
                seg_wit: "NONE".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let result: AccountsResponse = AccountsResponse::decode(&ret).unwrap();
            assert_eq!(result.accounts.first().unwrap().chain_type, "LITECOIN");
            assert_eq!(
                result.accounts.first().unwrap().address,
                "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN"
            );
            remove_created_wallet(&import_result.id);
        })
    }

    fn import_default_wallet() -> WalletResult {
        let param = HdStoreImportParam {
            mnemonic: MNEMONIC.to_string(),
            password: PASSWORD.to_string(),
            source: "MNEMONIC".to_string(),
            name: "test-wallet".to_string(),
            password_hint: "imtoken".to_string(),
            overwrite: true,
        };
        let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
        WalletResult::decode(&ret).unwrap()
    }

    #[test]
    pub fn test_hd_store_export() {
        run_test(|| {
            let wallet = import_default_wallet();

            let param = WalletKeyParam {
                id: wallet.id.to_string(),
                password: PASSWORD.to_string(),
            };
            let ret = hd_store_export(&encode_message(param).unwrap()).unwrap();
            let result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(&ret).unwrap();

            assert_eq!(result.r#type, KeyType::Mnemonic as i32);
            assert_eq!(result.value, MNEMONIC);
        })
    }

    #[test]
    pub fn test_keystore_common_store_derive() {
        run_test(|| {
            let param = HdStoreImportParam {
                mnemonic: OTHER_MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };
            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

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
                password: PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes =
                keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes).unwrap();
            assert_eq!(5, derived_accounts.accounts.len());
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

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_hd_store_derive_invalid_param() {
        run_test(|| {
            let param = HdStoreImportParam {
                mnemonic: OTHER_MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };
            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

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
                    password: PASSWORD.to_string(),
                    derivations: vec![derivation],
                };
                let ret = keystore_common_derive(&encode_message(param).unwrap());
                assert!(ret.is_err());
            }

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_private_key_store_import() {
        run_test(|| {
            let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
                private_key: "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB".to_string(),
                password: PASSWORD.to_string(),
                overwrite: true,
            };

            let ret_bytes = private_key_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret_bytes).unwrap();

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
                password: PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes =
                keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes).unwrap();
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
                password: PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes =
                keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes).unwrap();
            assert_eq!(
                "LgGNTHMkgETS7oQcoekvACJQcH355xECog",
                derived_accounts.accounts[0].address
            );
            assert_eq!("", derived_accounts.accounts[0].extended_xpub_key);

            let param = KeystoreCommonAccountsParam {
                id: import_result.id.to_string(),
            };
            let accounts_ret = keystore_common_accounts(&encode_message(param).unwrap()).unwrap();
            let ret = AccountsResponse::decode(accounts_ret).unwrap();
            assert_eq!(5, ret.accounts.len());

            remove_created_wallet(&import_result.id);
        })
    }

    fn import_default_pk_store() -> WalletResult {
        let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
            private_key: "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB".to_string(),
            password: PASSWORD.to_string(),
            overwrite: true,
        };

        let ret = private_key_store_import(&encode_message(param).unwrap()).unwrap();
        WalletResult::decode(ret).unwrap()
    }

    #[test]
    pub fn test_private_key_store_export() {
        run_test(|| {
            let import_result: WalletResult = import_default_pk_store();
            let param: PrivateKeyStoreExportParam = PrivateKeyStoreExportParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "BITCOINCASH".to_string(),
                network: "MAINNET".to_string(),
            };
            let ret_bytes = private_key_store_export(&encode_message(param).unwrap()).unwrap();
            let export_result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(&ret_bytes).unwrap();
            assert_eq!(
                "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB",
                export_result.value
            );
            assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);

            let param: PrivateKeyStoreExportParam = PrivateKeyStoreExportParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "BITCOINCASH".to_string(),
                network: "TESTNET".to_string(),
            };
            let ret_bytes = private_key_store_export(&encode_message(param).unwrap()).unwrap();
            let export_result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(&ret_bytes).unwrap();
            assert_eq!(
                "cT4fTJyLd5RmSZFHnkGmVCzXDKuJLbyTt7cy77ghTTCagzNdPH1j",
                export_result.value
            );
            assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);

            let param: PrivateKeyStoreExportParam = PrivateKeyStoreExportParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "TRON".to_string(),
                network: "".to_string(),
            };
            let ret_bytes = private_key_store_export(&encode_message(param).unwrap()).unwrap();
            let export_result: KeystoreCommonExportResult =
                KeystoreCommonExportResult::decode(&ret_bytes).unwrap();
            assert_eq!(
                "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
                export_result.value
            );
            assert_eq!(KeyType::PrivateKey as i32, export_result.r#type);
            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_keystore_common_verify() {
        run_test(|| {
            let wallets = vec![import_default_pk_store(), import_default_wallet()];
            for wallet in wallets {
                let param: WalletKeyParam = WalletKeyParam {
                    id: wallet.id.to_string(),
                    password: PASSWORD.to_string(),
                };

                let ret_bytes = keystore_common_verify(&encode_message(param).unwrap()).unwrap();
                let result: Response = Response::decode(&ret_bytes).unwrap();
                assert!(result.is_success);

                let param: WalletKeyParam = WalletKeyParam {
                    id: wallet.id.to_string(),
                    password: "WRONG PASSWORD".to_string(),
                };

                let ret = keystore_common_verify(&encode_message(param).unwrap());
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
                password: PASSWORD.to_string(),
                overwrite: true,
            };

            let ret_bytes = private_key_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret_bytes).unwrap();

            let param: WalletKeyParam = WalletKeyParam {
                id: import_result.id.to_string(),
                password: "WRONG PASSWORD".to_string(),
            };

            let ret = keystore_common_delete(&encode_message(param).unwrap());
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "password_incorrect");

            let param: WalletKeyParam = WalletKeyParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
            };

            let ret_bytes = keystore_common_delete(&encode_message(param).unwrap()).unwrap();
            let ret: Response = Response::decode(ret_bytes).unwrap();
            assert!(ret.is_success);

            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::PrivateKey as i32,
                value: "5JZc7wGRUr4J1RHDcM9ySWKLfQ2xjRUEo612qC4RLJ3G7jzJ4qx".to_string(),
            };

            let ret_bytes = keystore_common_exists(&encode_message(param).unwrap()).unwrap();
            let ret: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(&ret_bytes).unwrap();

            assert_eq!(false, ret.is_exists);
        })
    }

    #[test]
    pub fn test_keystore_common_exists() {
        run_test(|| {
            let wallet = import_default_wallet();
            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::Mnemonic as i32,
                value: format!("{}", MNEMONIC).to_string(),
            };

            let ret_bytes = keystore_common_exists(&encode_message(param).unwrap()).unwrap();
            let result: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(&ret_bytes).unwrap();
            assert!(result.is_exists);
            assert_eq!(result.id, wallet.id);

            let wallet = import_default_pk_store();
            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::PrivateKey as i32,
                value: "L2hfzPyVC1jWH7n2QLTe7tVTb6btg9smp5UVzhEBxLYaSFF7sCZB".to_string(),
            };

            let ret_bytes = keystore_common_exists(&encode_message(param).unwrap()).unwrap();
            let result: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(&ret_bytes).unwrap();
            assert!(result.is_exists);
            assert_eq!(result.id, wallet.id);

            let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
                r#type: KeyType::PrivateKey as i32,
                value: "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6"
                    .to_string(),
            };

            let ret_bytes = keystore_common_exists(&encode_message(param).unwrap()).unwrap();
            let result: KeystoreCommonExistsResult =
                KeystoreCommonExistsResult::decode(&ret_bytes).unwrap();
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

            let ret_bytes = keystore_common_accounts(&encode_message(param).unwrap()).unwrap();
            let result: AccountsResponse = AccountsResponse::decode(&ret_bytes).unwrap();
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
                password: PASSWORD.to_string(),
                derivations,
            };
            let derived_accounts_bytes =
                keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let derived_accounts: AccountsResponse =
                AccountsResponse::decode(derived_accounts_bytes).unwrap();
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
            let param = HdStoreImportParam {
                mnemonic: MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };
            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

            let derivation = Derivation {
                chain_type: "NERVOS".to_string(),
                path: "m/44'/309'/0'/0/0".to_string(),
                network: "TESTNET".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let rsp: AccountsResponse = AccountsResponse::decode(ret).unwrap();

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
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "NERVOS".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input).unwrap(),
                }),
            };
            let tx_bytes = encode_message(tx).unwrap();
            let ret = sign_tx(&tx_bytes).unwrap();
            let output: CkbTxOutput = CkbTxOutput::decode(&ret).unwrap();
            assert_eq!("0x5500000010000000550000005500000041000000776e010ac7e7166afa50fe54cfecf0a7106a2f11e8110e071ccab67cb30ed5495aa5c5f5ca2967a2fe4a60d5ad8c811382e51d8f916ba2911552bef6dedeca8a00", output.witnesses[0]);
            assert_eq!("0x5500000010000000550000005500000041000000914591d8abd5233740207337b0588fec58cad63143ddf204970526022b6db26d68311e9af49e1625e3a90e8a66eb1694632558d561d1e5d02cc7c7254e2d546100",output.witnesses[1]);

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_sign_tron_tx() {
        run_test(|| {
            let param = HdStoreImportParam {
                mnemonic: MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };
            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

            let derivation = Derivation {
                chain_type: "TRON".to_string(),
                path: "m/44'/195'/0'/0/0".to_string(),
                network: "".to_string(),
                seg_wit: "".to_string(),
                chain_id: "".to_string(),
            };
            let param = KeystoreCommonDeriveParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let rsp: AccountsResponse = AccountsResponse::decode(ret).unwrap();

            let raw_data = "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d".to_string();
            let input = TronTxInput { raw_data };
            let input_value = encode_message(input).unwrap();
            let tx = SignParam {
                id: import_result.id.to_string(),
                password: "WRONG PASSWORD".to_string(),
                chain_type: "TRON".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value.clone(),
                }),
            };

            let tx_bytes = encode_message(tx).unwrap();
            let ret = sign_tx(&tx_bytes);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "password_incorrect");

            let tx = SignParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "TRON1".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value.clone(),
                }),
            };

            let tx_bytes = encode_message(tx).unwrap();
            let ret = sign_tx(&tx_bytes);
            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), "unsupported_chain");

            let tx = SignParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "TRON".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: input_value,
                }),
            };

            let tx_bytes = encode_message(tx).unwrap();
            let ret = sign_tx(&tx_bytes).unwrap();
            let output: TronTxOutput = TronTxOutput::decode(&ret).unwrap();
            let expected_sign = "bbf5ce0549490613a26c3ac4fc8574e748eabda05662b2e49cea818216b9da18691e78cd6379000e9c8a35c13dfbf620f269be90a078b58799b56dc20da3bdf200";
            assert_eq!(expected_sign, output.signatures[0]);
            remove_created_wallet(&import_result.id);
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
                password: PASSWORD.to_string(),
                derivations: vec![derivation],
            };

            let ret = keystore_common_derive(&encode_message(param).unwrap()).unwrap();
            let rsp: AccountsResponse = AccountsResponse::decode(ret).unwrap();

            let raw_data = "0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d".to_string();
            let input = TronTxInput { raw_data };
            let tx = SignParam {
                id: import_result.id.to_string(),
                password: PASSWORD.to_string(),
                chain_type: "TRON".to_string(),
                address: rsp.accounts.first().unwrap().address.to_string(),
                input: Some(::prost_types::Any {
                    type_url: "imtoken".to_string(),
                    value: encode_message(input).unwrap(),
                }),
            };
            let tx_bytes = encode_message(tx).unwrap();
            let ret = sign_tx(&tx_bytes).unwrap();
            let output: TronTxOutput = TronTxOutput::decode(&ret).unwrap();
            let expected_sign = "7758c92df76d50774a67fdca6c90b922fc84be68c69164d4c7f500327bfa4b9655709b6b1f88e07e3bda266d7ca4b48c934557917692f63a31e301d79d7107d001";
            assert_eq!(expected_sign, output.signatures[0]);
            //            remove_created_wallet(&import_result.id);
        })
    }
    pub fn test_sign_btc_fork_invalid_address() {
        run_test(|| {
            let chain_types = vec!["BITCOINCASH", "LITECOIN"];
            let param = HdStoreImportParam {
                mnemonic: MNEMONIC.to_string(),
                password: PASSWORD.to_string(),
                source: "MNEMONIC".to_string(),
                name: "test-wallet".to_string(),
                password_hint: "imtoken".to_string(),
                overwrite: true,
            };
            let ret = hd_store_import(&encode_message(param).unwrap()).unwrap();
            let import_result: WalletResult = WalletResult::decode(&ret).unwrap();

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
                    password: PASSWORD.to_string(),
                    derivations: vec![derivation],
                };

                let ret = keystore_common_derive(&encode_message(param).unwrap()).unwrap();
                let rsp: AccountsResponse = AccountsResponse::decode(ret).unwrap();

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
                    password: PASSWORD.to_string(),
                    chain_type: chain_type.to_string(),
                    address: rsp.accounts.first().unwrap().address.to_string(),
                    input: Some(::prost_types::Any {
                        type_url: "imtoken".to_string(),
                        value: input_value.clone(),
                    }),
                };

                let tx_bytes = encode_message(tx).unwrap();
                let ret = sign_tx(&tx_bytes);
                assert!(ret.is_err());
                assert_eq!(format!("{}", ret.err().unwrap()), "invalid_to_address");
            }

            remove_created_wallet(&import_result.id);
        })
    }

    #[test]
    pub fn test_poison_err() {
        let _ = import_default_wallet();
        let _ = import_default_pk_store();
    }

    fn remove_created_wallet(wid: &str) {
        let full_file_path = format!("{}/{}.json", "/tmp/imtoken/wallets", wid);
        let p = Path::new(&full_file_path);
        remove_file(p).expect("should remove file");
    }
}
