use crate::ecc::KeyError;
use crate::Result;

use bitcoin::util::bip32::ChildNumber;

use std::convert::TryInto;
use std::str::FromStr;

pub fn get_account_path(path: &str) -> Result<String> {
    // example: m/44'/60'/0'/0/0
    let _ = bitcoin::util::bip32::DerivationPath::from_str(path)?;
    let mut children: Vec<&str> = path.split('/').collect();

    ensure!(children.len() >= 4, format!("{} path is too short", path));

    while children.len() > 4 {
        children.remove(children.len() - 1);
    }
    Ok(children.join("/"))
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DeriveJunction {
    Soft(u32),
    Hard(u32),
}

pub trait Derive: Sized {
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
    use crate::derive::get_account_path;
    use crate::generate_mnemonic;
    use crate::DeriveJunction;
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
    fn derive_junction_parse_test() {
        let valid_paths = vec![
            "m/0'/0'/0'/0'/0'",
            "m/0'/0'/0'/0/0",
            "m/0'/0'/0'/0'",
            "m/0'/0'/0'",
            "m/0'/0'/0'/1'",
        ];
        for path in valid_paths {
            let ret = DerivePath::from_str(path);
            assert!(ret.is_ok());
        }

        let invalid_paths = vec!["/0'", "m/-1", "a/0", "m//0"];
        for invalid_path in invalid_paths {
            let ret = DerivePath::from_str(invalid_path);
            assert!(ret.is_err());
        }
    }
}
