use core::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
//use std::sync::RwLock;
use parking_lot::RwLock;
use tcx_chain::Keystore;

use crate::error_handling::Result;

lazy_static! {
    pub static ref KEYSTORE_MAP: RwLock<HashMap<String, Keystore>> = RwLock::new(HashMap::new());
    pub static ref WALLET_FILE_DIR: RwLock<String> = RwLock::new("../test-data".to_string());
    pub static ref IS_DEBUG: RwLock<bool> = RwLock::new(false);
}

pub fn clean_keystore() {
    KEYSTORE_MAP.write().clear()
}

pub fn cache_keystore(keystore: Keystore) {
    KEYSTORE_MAP
        .write()
        .insert(keystore.id().to_owned(), keystore);
}

pub fn find_keystore_id_by_address(address: &str) -> Option<String> {
    let map = KEYSTORE_MAP.read();
    let mut k_id: Option<String> = None;
    for (id, keystore) in map.borrow().iter() {
        let mut iter = keystore.accounts().iter();
        if iter.any(|a| a.address == address) {
            k_id = Some(id.to_string());
            break;
        }
    }
    k_id
}

pub fn flush_keystore(ks: &Keystore) -> Result<()> {
    let json = ks.to_json();

    let file_dir = WALLET_FILE_DIR.read();
    let ks_path = format!("{}/{}.json", file_dir, ks.id());
    let path = Path::new(&ks_path);
    let mut file = fs::File::create(path)?;
    let _ = file.write_all(&json.as_bytes());
    Ok(())
}

pub fn delete_keystore_file(wid: &str) -> Result<()> {
    let file_dir = WALLET_FILE_DIR.read();
    let ks_path = format!("{}/{}.json", file_dir, wid);
    let path = Path::new(&ks_path);
    fs::remove_file(path)?;
    Ok(())
}
