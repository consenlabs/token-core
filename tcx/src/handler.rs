use std::fs;
use std::io::Read;
use std::path::Path;

use bytes::BytesMut;
use prost::Message;
use serde_json::Value;
use tcx_primitive::{get_account_path, private_key_without_version, FromHex, TypedPrivateKey};

use tcx_bch::{BchAddress, BchTransaction};
use tcx_btc_fork::{
    BtcForkAddress, BtcForkSegWitTransaction, BtcForkSignedTxOutput, BtcForkTransaction,
    BtcForkTxInput, WifDisplay,
};
use tcx_chain::{key_hash_from_mnemonic, key_hash_from_private_key, Keystore, KeystoreGuard};
use tcx_chain::{Account, HdKeystore, Metadata, PrivateKeystore, Source};
use tcx_ckb::{CkbAddress, CkbTxInput};
use tcx_crypto::{XPUB_COMMON_IV, XPUB_COMMON_KEY_128};
use tcx_tron::TrxAddress;

use crate::api::keystore_common_derive_param::Derivation;
use crate::api::sign_param::Key;
use crate::api::{
    AccountResponse, AccountsResponse, DerivedKeyResult, ExportPrivateKeyParam, HdStoreCreateParam,
    HdStoreImportParam, KeyType, KeystoreCommonAccountsParam, KeystoreCommonDeriveParam,
    KeystoreCommonExistsParam, KeystoreCommonExistsResult, KeystoreCommonExportResult,
    PrivateKeyStoreExportParam, PrivateKeyStoreImportParam, Response, WalletKeyParam, WalletResult,
};
use crate::api::{InitTokenCoreXParam, SignParam};
use crate::error_handling::Result;
use crate::filemanager::{cache_keystore, clean_keystore, flush_keystore, WALLET_FILE_DIR};
use crate::filemanager::{delete_keystore_file, KEYSTORE_MAP};

use crate::IS_DEBUG;
use tcx_chain::tcx_ensure;
use tcx_chain::Address;
use tcx_chain::{MessageSigner, TransactionSigner};
use tcx_constants::coin_info::coin_info_from_param;
use tcx_constants::CurveType;
use tcx_crypto::aes::cbc::encrypt_pkcs7;
use tcx_crypto::KDF_ROUNDS;
use tcx_primitive::{Bip32DeterministicPublicKey, Ss58Codec};
use tcx_substrate::{SubstrateAddress, SubstrateRawTxIn, SubstrateTxIn};
use tcx_tron::transaction::{TronMessageInput, TronTxInput};

pub(crate) fn encode_message(msg: impl Message) -> Result<Vec<u8>> {
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
        "POLKADOT" | "KUSAMA" => keystore.derive_coin::<SubstrateAddress>(&coin_info),
        _ => Err(format_err!("unsupported_chain")),
    }
}

pub fn init_token_core_x(data: &[u8]) -> Result<()> {
    let InitTokenCoreXParam {
        file_dir,
        xpub_common_key,
        xpub_common_iv,
        is_debug,
    } = InitTokenCoreXParam::decode(data).unwrap();
    *WALLET_FILE_DIR.write() = file_dir.to_string();
    *XPUB_COMMON_KEY_128.write() = xpub_common_key.to_string();
    *XPUB_COMMON_IV.write() = xpub_common_iv.to_string();

    if is_debug {
        *IS_DEBUG.write() = is_debug;
        if is_debug {
            *KDF_ROUNDS.write() = 1024;
        }
    }
    scan_keystores()?;

    Ok(())
}

pub(crate) fn scan_keystores() -> Result<()> {
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

pub(crate) fn hd_store_create(data: &[u8]) -> Result<Vec<u8>> {
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

pub(crate) fn hd_store_import(data: &[u8]) -> Result<Vec<u8>> {
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
        return Err(format_err!("{}", "address_already_exist"));
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

#[deprecated(
    since = "2.5.1",
    note = "Please use the export_mnemonic function instead"
)]
#[allow(deprecated)]
pub(crate) fn hd_store_export(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).expect("hd_store_export");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;

    let export_result = KeystoreCommonExportResult {
        id: guard.keystore().id(),
        r#type: KeyType::Mnemonic as i32,
        value: guard.keystore().export()?,
    };

    encode_message(export_result)
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

pub(crate) fn keystore_common_derive(data: &[u8]) -> Result<Vec<u8>> {
    let param: KeystoreCommonDeriveParam =
        KeystoreCommonDeriveParam::decode(data).expect("keystore_common_derive");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let mut guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;

    let mut account_responses: Vec<AccountResponse> = vec![];

    for derivation in param.derivations {
        let account = derive_account(guard.keystore_mut(), &derivation)?;
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
    flush_keystore(guard.keystore())?;
    encode_message(accounts_rsp)
}

pub(crate) fn export_mnemonic(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).expect("export_mnemonic");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;

    tcx_ensure!(
        guard.keystore().determinable(),
        format_err!("{}", "private_keystore_cannot_export_mnemonic")
    );

    let export_result = KeystoreCommonExportResult {
        id: guard.keystore().id(),
        r#type: KeyType::Mnemonic as i32,
        value: guard.keystore().export()?,
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

pub(crate) fn private_key_store_import(data: &[u8]) -> Result<Vec<u8>> {
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
        return Err(format_err!("{}", "address_already_exist"));
    }

    let pk_bytes = key_data_from_any_format_pk(&param.private_key)?;
    let private_key = hex::encode(pk_bytes);
    let pk_store =
        PrivateKeystore::from_private_key(&private_key, &param.password, Source::Private);

    let mut keystore = Keystore::PrivateKey(pk_store);

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

#[deprecated(
    since = "2.5.1",
    note = "Please use the export_private_key function instead"
)]
pub(crate) fn private_key_store_export(data: &[u8]) -> Result<Vec<u8>> {
    let param: PrivateKeyStoreExportParam =
        PrivateKeyStoreExportParam::decode(data).expect("private_key_store_export");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;

    let pk_hex = guard.keystore().export()?;

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
        id: guard.keystore().id(),
        r#type: KeyType::PrivateKey as i32,
        value,
    };

    encode_message(export_result)
}

pub(crate) fn export_private_key(data: &[u8]) -> Result<Vec<u8>> {
    let param: ExportPrivateKeyParam =
        ExportPrivateKeyParam::decode(data).expect("export_private_key");
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let mut guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;

    let pk_hex = if param.path.is_empty() {
        guard
            .keystore_mut()
            .export_private_key(&param.chain_type, &param.main_address, None)?
    } else {
        // get the relative path
        let mut relative_path: &str = param.path.as_str();
        if param.path.starts_with("m") {
            let acc_path = get_account_path(relative_path)?;
            relative_path = &relative_path[acc_path.len()..];
        }

        if relative_path.starts_with("/") {
            relative_path = &relative_path[1..];
        }

        guard.keystore_mut().export_private_key(
            &param.chain_type,
            &param.main_address,
            Some(relative_path),
        )?
    };

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
        id: guard.keystore().id(),
        r#type: KeyType::PrivateKey as i32,
        value,
    };

    encode_message(export_result)
}

pub(crate) fn keystore_common_verify(data: &[u8]) -> Result<Vec<u8>> {
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

pub(crate) fn keystore_common_delete(data: &[u8]) -> Result<Vec<u8>> {
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

pub(crate) fn keystore_common_exists(data: &[u8]) -> Result<Vec<u8>> {
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

pub(crate) fn keystore_common_accounts(data: &[u8]) -> Result<Vec<u8>> {
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

pub(crate) fn sign_tx(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignParam = SignParam::decode(data).expect("SignTxParam");

    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let mut guard = match param.key.clone().unwrap() {
        Key::Password(password) => KeystoreGuard::unlock_by_password(keystore, &password)?,
        Key::DerivedKey(derived_key) => {
            KeystoreGuard::unlock_by_derived_key(keystore, &derived_key)?
        }
    };

    match param.chain_type.as_str() {
        "BITCOINCASH" | "LITECOIN" => sign_btc_fork_transaction(&param, guard.keystore_mut()),
        "TRON" => sign_tron_tx(&param, guard.keystore_mut()),
        "NERVOS" => sign_nervos_ckb(&param, guard.keystore_mut()),
        "POLKADOT" | "KUSAMA" => sign_substrate_tx_raw(&param, guard.keystore_mut()),
        _ => Err(format_err!("unsupported_chain")),
    }
}

pub(crate) fn sign_btc_fork_transaction(
    param: &SignParam,
    keystore: &mut Keystore,
) -> Result<Vec<u8>> {
    let input: BtcForkTxInput =
        BtcForkTxInput::decode(&param.input.as_ref().expect("tx_input").value.clone())
            .expect("BitcoinForkTransactionInput");
    let coin = coin_info_from_param(&param.chain_type, &input.network, &input.seg_wit)?;

    let signed_tx: BtcForkSignedTxOutput = if param.chain_type.as_str() == "BITCOINCASH" {
        if !BchAddress::is_valid(&input.to, &coin) {
            return Err(format_err!("address_invalid"));
        }
        let tran = BchTransaction::new(input, coin);
        keystore.sign_transaction(&param.chain_type, &param.address, &tran)?
    } else if input.seg_wit.as_str() != "NONE" {
        if !BtcForkAddress::is_valid(&input.to, &coin) {
            return Err(format_err!("address_invalid"));
        }
        let tran = BtcForkSegWitTransaction::new(input, coin);
        keystore.sign_transaction(&param.chain_type, &param.address, &tran)?
    } else {
        if !BtcForkAddress::is_valid(&input.to, &coin) {
            return Err(format_err!("address_invalid"));
        }
        let tran = BtcForkTransaction::new(input, coin);
        keystore.sign_transaction(&param.chain_type, &param.address, &tran)?
    };
    encode_message(signed_tx)
}

pub(crate) fn sign_nervos_ckb(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: CkbTxInput =
        CkbTxInput::decode(&param.input.as_ref().expect("tx_iput").value.clone())
            .expect("CkbTxInput");
    let signed_tx = keystore.sign_transaction(&param.chain_type, &param.address, &input)?;
    encode_message(signed_tx)
}

pub(crate) fn sign_tron_tx(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: TronTxInput =
        TronTxInput::decode(&param.input.as_ref().expect("tx_input").value.clone())
            .expect("TronTxInput");
    let signed_tx = keystore.sign_transaction(&param.chain_type, &param.address, &input)?;

    encode_message(signed_tx)
}

pub(crate) fn tron_sign_message(data: &[u8]) -> Result<Vec<u8>> {
    let param: SignParam = SignParam::decode(data).expect("SignParam");

    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let mut guard = match param.key.unwrap() {
        Key::Password(password) => KeystoreGuard::unlock_by_password(keystore, &password)?,
        Key::DerivedKey(derived_key) => {
            KeystoreGuard::unlock_by_derived_key(keystore, &derived_key)?
        }
    };

    let input: TronMessageInput =
        TronMessageInput::decode(param.input.expect("TronMessageInput").value.clone())
            .expect("TronMessageInput");
    let signed_tx = guard
        .keystore_mut()
        .sign_message(&param.chain_type, &param.address, &input)?;
    encode_message(signed_tx)
}

pub(crate) fn get_derived_key(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).unwrap();
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let dk = keystore.get_derived_key(&param.password)?;

    let ret = DerivedKeyResult {
        id: param.id.to_owned(),
        derived_key: dk,
    };
    encode_message(ret)
}

pub(crate) fn sign_substrate_tx(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: SubstrateTxIn =
        SubstrateTxIn::decode(&param.input.as_ref().expect("tx_input").value.clone())
            .expect("SubstrateTxIn");
    let signed_tx = keystore.sign_transaction(&param.chain_type, &param.address, &input)?;
    encode_message(signed_tx)
}

pub(crate) fn sign_substrate_tx_raw(param: &SignParam, keystore: &mut Keystore) -> Result<Vec<u8>> {
    let input: SubstrateRawTxIn =
        SubstrateRawTxIn::decode(&param.input.as_ref().expect("raw_tx_input").value.clone())
            .expect("SubstrateTxIn");
    let signed_tx = keystore.sign_transaction(&param.chain_type, &param.address, &input)?;
    encode_message(signed_tx)
}

pub(crate) fn unlock_then_crash(data: &[u8]) -> Result<Vec<u8>> {
    let param: WalletKeyParam = WalletKeyParam::decode(data).unwrap();
    let mut map = KEYSTORE_MAP.write();
    let keystore: &mut Keystore = match map.get_mut(&param.id) {
        Some(keystore) => Ok(keystore),
        _ => Err(format_err!("{}", "wallet_not_found")),
    }?;

    let _guard = KeystoreGuard::unlock_by_password(keystore, &param.password)?;
    panic!("test_unlock_then_crash");
}
