use crate::KeyError;
use crate::Result;
use bip39::{Language, Mnemonic, MnemonicType};

use bitcoin::util::bip32::ChildNumber;

use std::convert::TryInto;
use std::str::FromStr;

#[allow(dead_code)]
pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).to_string()
}

pub fn get_account_path(path: &str) -> Result<String> {
    // example: m/44'/60'/0'/0/0
    let _ = bitcoin::util::bip32::DerivationPath::from_str(path)?;
    let mut childs: Vec<&str> = path.split('/').collect();

    ensure!(childs.len() >= 4, format!("{} path is too short", path));
    while childs.len() > 4 {
        childs.remove(childs.len() - 1);
    }
    Ok(childs.join("/"))
}

pub fn relative_path_to_child_nums(path: &str) -> Result<Vec<ChildNumber>> {
    let childs: Vec<&str> = path.split('/').collect();
    childs
        .iter()
        .filter(|child| **child != "")
        .map(|child| {
            if child.ends_with('\'') {
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DeriveJunction {
    Soft(u32),
    Hard(u32),
}

pub trait Derive: Sized {
    type Error;

    fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter) -> Result<Self>;
}

// TODO add parity string derivation path
#[allow(dead_code)]
impl DeriveJunction {
    pub fn soft(index: u32) -> Self {
        DeriveJunction::Soft(index)
    }

    pub fn hard(index: u32) -> Self {
        DeriveJunction::Hard(index)
    }

    pub fn is_soft(&self) -> bool {
        match *self {
            DeriveJunction::Soft(_) => true,
            _ => false,
        }
    }

    pub fn is_hard(&self) -> bool {
        match *self {
            DeriveJunction::Hard(_) => true,
            _ => false,
        }
    }
}

impl FromStr for DeriveJunction {
    type Err = failure::Error;

    fn from_str(inp: &str) -> Result<Self> {
        Ok(
            if inp.chars().last().map_or(false, |l| l == '\'' || l == 'h') {
                DeriveJunction::hard(
                    inp[0..inp.len() - 1]
                        .parse()
                        .map_err(|_| KeyError::InvalidChildNumberFormat)?,
                )
            } else {
                DeriveJunction::soft(
                    inp.parse()
                        .map_err(|_| KeyError::InvalidChildNumberFormat)?,
                )
            },
        )
    }
}

impl TryInto<ChildNumber> for DeriveJunction {
    type Error = failure::Error;

    fn try_into(self) -> Result<ChildNumber> {
        if let Ok(num) = match self {
            DeriveJunction::Soft(index) => ChildNumber::from_normal_idx(index),
            DeriveJunction::Hard(index) => ChildNumber::from_hardened_idx(index),
        } {
            Ok(num)
        } else {
            Err(KeyError::InvalidChildNumber.into())
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DerivePath(Vec<DeriveJunction>);

impl FromStr for DerivePath {
    type Err = failure::Error;

    fn from_str(path: &str) -> Result<Self> {
        let mut parts = path.split('/').peekable();
        // First parts must be `m`.
        if *parts.peek().unwrap() == "m" {
            parts.next();
        }

        let ret: Result<Vec<DeriveJunction>> = parts.map(str::parse).collect();
        Ok(DerivePath(ret?))
    }
}

impl ::std::iter::IntoIterator for &DerivePath {
    type Item = DeriveJunction;
    type IntoIter = ::std::vec::IntoIter<DeriveJunction>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl AsRef<[DeriveJunction]> for DerivePath {
    fn as_ref(&self) -> &[DeriveJunction] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::DerivePath;
    use crate::derive::{generate_mnemonic, get_account_path, relative_path_to_child_nums};
    use crate::DeriveJunction;
    use bitcoin::util::bip32::ChildNumber;
    use std::str::FromStr;

    #[test]
    fn derive_path_from_root() {
        if let Ok(path) = DerivePath::from_str("m/44'/159h/0") {
            let expects = vec![
                DeriveJunction::hard(44),
                DeriveJunction::hard(159),
                DeriveJunction::soft(0),
            ];

            let mut index = 0;

            for value in path.into_iter() {
                assert_eq!(expects[index], value, "should be correct path");
                index = index + 1;
            }
        } else {
            assert_eq!(0, 1, "should not be failed");
        }
    }

    #[test]
    fn derive_path_relative() {
        if let Ok(path) = DerivePath::from_str("0/0") {
            let expects = vec![DeriveJunction::soft(0), DeriveJunction::soft(0)];

            let mut index = 0;

            for value in path.into_iter() {
                assert_eq!(expects[index], value, "should be correct path");
                index = index + 1;
            }
        } else {
            assert_eq!(0, 1, "should not be failed");
        }
    }

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
