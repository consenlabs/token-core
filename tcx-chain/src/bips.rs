use std::convert::AsMut;
use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType};
use bitcoin::util::base58;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};

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
    use crate::bips::get_account_path;
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
    }
}
