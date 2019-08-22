use std::convert::AsMut;
use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType};
use bitcoin::util::base58;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};
use byteorder::{BigEndian, ByteOrder};

use crate::Result;

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
    let childs: Vec<&str> = path.split("/").collect();
    childs
        .iter()
        .filter(|child| **child != "")
        .map(|child| {
            if child.ends_with("'") {
                let idx = child
                    .replace("'", "")
                    .parse::<u32>()
                    .map_err(|_err| format_err!("error happen when parse path from {}", child))?;
                ChildNumber::from_hardened_idx(idx).map_err(|_err| format_err!("parse idx err"))
            } else {
                let idx = child
                    .parse::<u32>()
                    .map_err(|_err| format_err!("error happen when parse path from {}", child))?;
                ChildNumber::from_normal_idx(idx).map_err(|_err| format_err!("parse idx err"))
            }
        })
        .collect::<Result<Vec<ChildNumber>>>()
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
    use crate::bips::{generate_mnemonic, get_account_path, relative_path_to_child_nums};
    use crate::{DerivationInfo, Error};
    use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};
    use std::str::FromStr;

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic_str = generate_mnemonic();
        assert_eq!(
            12,
            mnemonic_str.split_whitespace().collect::<Vec<&str>>().len()
        );
        let second_mnemonic = generate_mnemonic();
        assert_ne!(mnemonic_str, second_mnemonic);
    }

    #[test]
    fn account_path() {
        let path = "m/44'/60'/0'/0/0";
        let result = get_account_path(path);
        assert_eq!(result.unwrap(), "m/44'/60'/0'");

        let short_error = get_account_path("m/44'");
        assert_eq!(
            short_error.err().unwrap().to_string(),
            "m/44\' path is too short"
        );

        let invalid_path = get_account_path("m/44/a");
        let err = invalid_path.expect_err("should throw invalid path");
        assert_eq!("invalid child number format", format!("{}", err));
    }

    #[test]
    fn test_relative_path() {
        let derive_path = "0/1";
        let child_nums = relative_path_to_child_nums(derive_path).unwrap();
        assert_eq!(2, child_nums.len());
        assert_eq!(&ChildNumber::from_normal_idx(0).unwrap(), &child_nums[0]);
        assert_eq!(&ChildNumber::from_normal_idx(1).unwrap(), &child_nums[1]);

        let derive_path = "/0/1";
        let child_nums = relative_path_to_child_nums(derive_path).unwrap();
        assert_eq!(2, child_nums.len());
        assert_eq!(&ChildNumber::from_normal_idx(0).unwrap(), &child_nums[0]);
        assert_eq!(&ChildNumber::from_normal_idx(1).unwrap(), &child_nums[1]);

        let derive_path = "0'/1/2";
        let child_nums = relative_path_to_child_nums(derive_path).unwrap();
        assert_eq!(3, child_nums.len());
        assert_eq!(&ChildNumber::from_hardened_idx(0).unwrap(), &child_nums[0]);
        assert_eq!(&ChildNumber::from_normal_idx(1).unwrap(), &child_nums[1]);
        assert_eq!(&ChildNumber::from_normal_idx(2).unwrap(), &child_nums[2]);
    }

    #[test]
    fn test_encode_with_network() {
        let main_network_xpub_version: [u8; 4] = [0x04, 0x88, 0xb2, 0x1e];
        let main_network_xprv_version: [u8; 4] = [0x04, 0x88, 0xad, 0xe4];

        let xpub = "tpubDDDcs8o1LaKXKXaPTEVBUZJYTgNAte4xj24MtFCMsfrHku93ZZjy87CGyz93dcocR6x6JHdusHodD9EVcSQuDbmkAWznWZtvyqyMDqS6VK4";
        let epk = ExtendedPubKey::from_str(xpub).unwrap();
        let derivation_info = DerivationInfo::from(epk);
        let ret = derivation_info.encode_with_network(main_network_xpub_version);
        assert_eq!("xpub6CqzLtyKdJN53jPY13W6GdyB8ZGWuFZuBPU4Xh9DXm6Q1cULVLtsyfXSjx4G77rNdCRBgi83LByaWxjtDaZfLAKT6vFUq3EhPtNwTpJigx8", ret);

        let xprv = "tprv8g8UWPRHxaNWXZN3uoaiNpyYyaDr2j5Dvcj1vxLxKcEF653k7xcN9wq9eT73wBM1HzE9hmWJbAPXvDvaMXqGWm81UcVpHnmATfH2JJrfhGg";
        let epk = ExtendedPrivKey::from_str(xprv).unwrap();
        let derivation_info = DerivationInfo::from(epk);
        let ret = derivation_info.encode_with_network(main_network_xprv_version);
        assert_eq!("xprv9yTXj46xZJYRvk8XFEjDDBMZfSodoD3Db4ou4XvVqdjmJUJf8bGceCThjGwPvoxgvYhNhftYRoojTNNqEKVKhhrQwyHWdS37YZXbrcJr8HS", ret);
    }
}
