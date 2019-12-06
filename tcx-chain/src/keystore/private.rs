use super::Account;
use super::{Address, Metadata, Source};
use tcx_constants::{CoinInfo, CurveType};
use tcx_crypto::{Crypto, Pbkdf2Params};

use super::Error;
use super::Result;
use crate::keystore::{Keystore, Store};
use core::result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Map;
use tcx_crypto::hash::hex_sha256;
use tcx_primitive::{
    KeyManage, PrivateKey, PublicKey, Secp256k1PrivateKey, Secp256k1PublicKey, TypedPrivateKey,
};
use uuid::Uuid;

pub struct PrivateKeystore {
    store: Store,

    private_key: Option<Vec<u8>>,
}

impl PrivateKeystore {
    pub const VERSION: i64 = 11001i64;

    pub(crate) fn store(&self) -> &Store {
        &self.store
    }

    pub(crate) fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    pub(crate) fn from_store(store: Store) -> Self {
        PrivateKeystore {
            store,
            private_key: None,
        }
    }

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
            .store
            .active_accounts
            .iter()
            .find(|acc| acc.address == address)
            .ok_or(Error::AccountNotFound)?;

        let private_key = self.private_key.as_ref().unwrap().as_slice();

        TypedPrivateKey::from_slice(account.curve, private_key)
    }

    pub fn derive_coin<A: Address>(&mut self, coin_info: &CoinInfo) -> Result<&Account> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        let sk = self.private_key.as_ref().unwrap();

        let account = Self::private_key_to_account::<A>(coin_info, sk)?;

        self.store.active_accounts.push(account);

        Ok(self.store.active_accounts.last().unwrap())
    }

    /// Find an account by coin symbol
    pub fn account(&self, symbol: &str) -> Option<&Account> {
        self.store
            .active_accounts
            .iter()
            .find(|acc| acc.coin == symbol)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        self.store.crypto.verify_password(password)
    }

    pub fn from_private_key(private_key: &str, password: &str, source: Source) -> PrivateKeystore {
        let key_hash = hex_sha256(private_key);
        let pk_bytes = hex::decode(private_key).expect("valid private_key");
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, &pk_bytes);

        let meta = Metadata {
            source,
            ..Metadata::default()
        };

        let store = Store {
            key_hash,
            crypto,
            meta,
            id: Uuid::new_v4().to_hyphenated().to_string(),
            version: PrivateKeystore::VERSION,
            active_accounts: vec![],
        };

        PrivateKeystore {
            store,
            private_key: None,
        }
    }

    pub fn private_key_to_account<A: Address>(
        coin: &CoinInfo,
        private_key: &[u8],
    ) -> Result<Account> {
        let tsk = TypedPrivateKey::from_slice(coin.curve, private_key)?;
        let addr = A::from_public_key(&tsk.public_key(), coin)?;

        let acc = Account {
            address: addr,
            derivation_path: "".to_string(),
            curve: coin.curve,
            coin: coin.coin.to_owned(),
            network: coin.network.to_string(),
            seg_wit: coin.seg_wit.to_string(),
            ext_pub_key: "".to_string(),
        };

        Ok(acc)
    }

    pub fn private_key(&self) -> Result<String> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        let vec = self.private_key.as_ref().unwrap().to_vec();
        Ok(hex::encode(&vec))

        //        TypedPrivateKey::from_slice(CurveType::SECP256k1, &priv_key)
    }

    fn decrypt_private_key(&self, password: &str) -> Result<Vec<u8>> {
        self.store.crypto.decrypt(password)
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
        assert_eq!(keystore.store.version, 11001);
        assert_ne!(keystore.store.id, "");
        assert_eq!(keystore.store.active_accounts.len(), 0);
    }
}
