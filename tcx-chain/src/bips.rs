use bip39::{Language, Mnemonic, MnemonicType};

use crate::Result;
use bitcoin::util::base58;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, ChildNumber};
use byteorder::{BigEndian, ByteOrder};
use std::convert::AsMut;
use std::str::FromStr;

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).to_string()
}

pub fn get_account_path(path: &str) -> Result<String> {
    // example: m/44'/60'/0'/0/0
    let _ = bitcoin::util::bip32::DerivationPath::from_str(path)?;
    let mut childs: Vec<&str> = path.split("/").collect();

    ensure!(childs.len() >= 4, format!("{} path is too short", path));
    while childs.len() > 4 {
        childs.remove(childs.len() - 1);
    }
    Ok(childs.join("/"))
}

pub fn relative_path_to_child_nums(path: &str) -> Result<Vec<ChildNumber>> {
    let mut child_nums: Vec<ChildNumber> = vec![];
    let childs: Vec<&str> = path.split("/").collect();
    childs.iter().map(|child| {
        if child.ends_with("'") {
            let idx = child.replace("'", "").parse::<u32>()
                .map_err(|err| format_err!("error happen when parse path from {}", child))?;
            ChildNumber::from_hardened_idx(idx).map_err(|err| format_err!("parse idx err"))
        } else {
            let idx = child.parse::<u32>()
                .map_err(|err| format_err!("error happen when parse path from {}", child))?;;
            ChildNumber::from_normal_idx(idx).map_err(|err| format_err!("parse idx err"))
        }
    }).collect::<Result<Vec<ChildNumber>>>()

}

pub struct DerivationInfo {
    depth: u8,
    parent_fingerprint: [u8; 4],
    child_number: u32,
    chain_code: [u8; 32],
    key: [u8; 33],
}

impl DerivationInfo {
    pub fn encode_with_network(&self, network: [u8; 4]) -> String {
        let mut ret = [0; 78];
        ret[0..4].copy_from_slice(&network);
        ret[4] = self.depth as u8;
        ret[5..9].copy_from_slice(&self.parent_fingerprint[..]);

        BigEndian::write_u32(&mut ret[9..13], u32::from(self.child_number));

        ret[13..45].copy_from_slice(&self.chain_code[..]);
        ret[45..78].copy_from_slice(&self.key[..]);
        base58::check_encode_slice(&ret[..]).to_string()
    }
}

impl From<ExtendedPubKey> for DerivationInfo {
    fn from(epk: ExtendedPubKey) -> Self {
        DerivationInfo {
            depth: epk.depth,
            parent_fingerprint: epk.parent_fingerprint.as_bytes().clone(),
            child_number: u32::from(epk.child_number),
            chain_code: epk.chain_code.as_bytes().clone(),
            key: epk.public_key.key.serialize(),
        }
    }
}

impl From<ExtendedPrivKey> for DerivationInfo {
    fn from(epk: ExtendedPrivKey) -> Self {
        let mut key = [0u8; 33];
        key[0] = 0u8;
        key[1..33].copy_from_slice(&epk.private_key[..]);
        DerivationInfo {
            depth: epk.depth,
            parent_fingerprint: epk.parent_fingerprint.as_bytes().clone(),
            child_number: u32::from(epk.child_number),
            chain_code: epk.chain_code.as_bytes().clone(),
            key,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bips::get_account_path;
    #[test]
    fn account_path() {
        let path = "m/44'/60'/0'/0/0";
        let result = get_account_path(path);
        assert_eq!(result.unwrap(), "m/44'/60'/0'");

        let short_error = get_account_path("m/44'");
        assert_eq!(short_error.err().unwrap().to_string(), "m/44'/60'/0'");
    }
}
