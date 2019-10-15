use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType};

use bitcoin::util::bip32::ChildNumber;

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
mod tests {}
