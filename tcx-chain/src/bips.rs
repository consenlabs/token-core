use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType};
use bitcoin::util::base58;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};
use byteorder::{BigEndian, ByteOrder};

use crate::Result;

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

#[cfg(test)]
mod tests {
    use crate::bips::{generate_mnemonic, get_account_path, relative_path_to_child_nums};
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

}
