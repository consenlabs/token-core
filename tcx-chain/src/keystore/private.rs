use super::{Account, Extra};
use super::{Address, Metadata, Source};
use tcx_constants::{CoinInfo, CurveType};
use tcx_crypto::{Crypto, Pbkdf2Params};

use super::Error;
use super::Result;
use crate::keystore::Keystore;
use crate::EmptyExtra;
use core::result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Map;
use tcx_primitive::{
    KeyManage, PrivateKey, PublicKey, Secp256k1PrivateKey, Secp256k1PublicKey, TypedPrivateKey,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateKeystore {
    pub id: String,
    pub version: i64,
    pub crypto: Crypto<Pbkdf2Params>,
    pub active_accounts: Vec<Account>,

    #[serde(rename = "imTokenMeta")]
    pub meta: Metadata,

    #[serde(skip_serializing)]
    private_key: Option<Vec<u8>>,
}

impl PrivateKeystore {
    pub const VERSION: i64 = 11001i64;

    pub fn unlock_by_password(&mut self, password: &str) -> Result<()> {
        self.private_key = Some(self.decrypt_private_key(password)?);

        Ok(())
    }

    pub fn lock(&mut self) {
        self.private_key = None;
    }

    pub fn find_private_key(&self, address: &str) -> Result<TypedPrivateKey> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        let account = self
            .active_accounts
            .iter()
            .find(|acc| acc.address == address);
        if account.is_none() {
            return Err(Error::AccountNotFound.into());
        }

        self.get_private_key(account.unwrap().curve)
    }

    pub fn derive_coin<A: Address, E: Extra>(&mut self, coin_info: &CoinInfo) -> Result<&Account> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        let sk = self.private_key.as_ref().unwrap().as_slice();

        let account = Self::private_key_to_account::<A, E>(coin_info, sk)?;

        self.active_accounts.push(account);
        Ok(self.active_accounts.last().unwrap())
    }

    /// Find an account by coin symbol
    pub fn account(&self, symbol: &str) -> Option<&Account> {
        self.active_accounts.iter().find(|acc| acc.coin == symbol)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        self.crypto.verify_password(password)
    }

    pub fn from_private_key(private_key: &str, password: &str, source: Source) -> PrivateKeystore {
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, private_key.as_bytes());

        let meta = Metadata {
            source,
            ..Metadata::default()
        };

        PrivateKeystore {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: PrivateKeystore::VERSION,
            crypto,
            active_accounts: vec![],
            meta,
            private_key: None,
        }
    }

    pub fn get_private_key(&self, curve_type: CurveType) -> Result<TypedPrivateKey> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        let private_key = self.private_key.as_ref().unwrap().as_slice();

        TypedPrivateKey::from_slice(curve_type, private_key)
    }

    pub fn private_key_to_account<A: Address, E: Extra>(
        coin: &CoinInfo,
        private_key: &[u8],
    ) -> Result<Account> {
        let tsk = TypedPrivateKey::from_slice(coin.curve, private_key)?;
        let addr = A::from_public_key(&tsk.public_key()?.to_bytes(), Some(&coin.symbol))?;

        let extra = EmptyExtra {};

        let acc = Account {
            address: addr,
            derivation_path: "".to_string(),
            curve: coin.curve,
            coin: coin.symbol.to_owned(),
            extra: serde_json::to_value(extra.clone()).expect("extra_error"),
        };

        Ok(acc)
    }

    pub fn private_key(&self) -> Result<TypedPrivateKey> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        let priv_key = self.private_key.as_ref().unwrap().to_vec();

        TypedPrivateKey::from_slice(CurveType::SECP256k1, &priv_key)
    }

    pub fn decrypt_private_key(&self, password: &str) -> Result<Vec<u8>> {
        self.crypto.decrypt(password)
    }
}

#[cfg(test)]
mod tests {
    use crate::{PrivateKeystore, Source};
    static PASSWORD: &'static str = "Insecure Pa55w0rd";

    #[test]
    pub fn from_private_key_test() {
        let keystore = PrivateKeystore::from_private_key(
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            PASSWORD,
            Source::Private,
        );
        assert_eq!(keystore.version, 11001);
        assert_ne!(keystore.id, "");
        assert_eq!(keystore.active_accounts.len(), 0);
    }
}
