use super::Account;
use super::{Address, Metadata};
use tcx_constants::CoinInfo;
use tcx_crypto::{Crypto, Key, Pbkdf2Params};

use super::Error;
use super::Result;
use crate::keystore::Store;

use tcx_crypto::hash::dsha256;
use tcx_primitive::TypedPrivateKey;
use uuid::Uuid;

pub fn key_hash_from_private_key(data: &[u8]) -> String {
    hex::encode(dsha256(data)[..20].to_vec())
}

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

    pub(crate) fn unlock_by_password(&mut self, password: &str) -> Result<()> {
        self.private_key = Some(self.decrypt_private_key(Key::Password(password.to_owned()))?);

        Ok(())
    }

    pub(crate) fn unlock_by_derived_key(&mut self, derived_key: &str) -> Result<()> {
        self.private_key = Some(self.decrypt_private_key(Key::DerivedKey(derived_key.to_owned()))?);

        Ok(())
    }

    pub(crate) fn lock(&mut self) {
        self.private_key = None;
    }

    pub(crate) fn is_locked(&self) -> bool {
        self.private_key.is_none()
    }

    pub(crate) fn find_private_key(&self, address: &str) -> Result<TypedPrivateKey> {
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

    pub(crate) fn derive_coin<A: Address>(&mut self, coin_info: &CoinInfo) -> Result<Account> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);

        if self.store.active_accounts.len() > 0
            && self.store.active_accounts[0].curve != coin_info.curve
        {
            return Err(Error::PkstoreCannotAddOtherCurveAccount.into());
        }

        let sk = self.private_key.as_ref().unwrap();

        let account = Self::private_key_to_account::<A>(coin_info, sk)?;
        if let Some(_) = self
            .store
            .active_accounts
            .iter()
            .find(|x| x.address == account.address && x.coin == account.coin)
        {
            return Ok(account);
        } else {
            self.store.active_accounts.push(account.clone());
            Ok(account)
        }
    }

    /// Find an account by coin symbol
    pub(crate) fn account(&self, symbol: &str, address: &str) -> Option<&Account> {
        self.store
            .active_accounts
            .iter()
            .find(|acc| acc.coin == symbol && acc.address == address)
    }

    pub(crate) fn verify_password(&self, password: &str) -> bool {
        self.store.crypto.verify_password(password)
    }

    pub fn from_private_key(private_key: &str, password: &str, meta: Metadata) -> PrivateKeystore {
        let key_data: Vec<u8> = hex::decode(private_key).expect("hex can't decode");
        let key_hash = key_hash_from_private_key(&key_data);
        //        let pk_bytes = hex::decode(private_key).expect("valid private_key");
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, &key_data);

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

    pub(crate) fn private_key_to_account<A: Address>(
        coin: &CoinInfo,
        private_key: &[u8],
    ) -> Result<Account> {
        let tsk = TypedPrivateKey::from_slice(coin.curve, private_key)?;
        let pub_key = tsk.public_key();
        let addr = A::from_public_key(&pub_key, coin)?;

        let acc = Account {
            address: addr,
            derivation_path: "".to_string(),
            curve: coin.curve,
            coin: coin.coin.to_owned(),
            network: coin.network.to_string(),
            seg_wit: coin.seg_wit.to_string(),
            ext_pub_key: "".to_string(),
            public_key: Some(hex::encode(pub_key.to_bytes())),
        };

        Ok(acc)
    }

    pub(crate) fn private_key(&self) -> Result<String> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);
        let vec = self.private_key.as_ref().unwrap().to_vec();
        Ok(hex::encode(&vec))
    }

    fn decrypt_private_key(&self, key: Key) -> Result<Vec<u8>> {
        self.store.crypto.decrypt(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Metadata, PrivateKeystore, Source};
    use tcx_constants::TEST_PASSWORD;

    #[test]
    pub fn from_private_key_test() {
        let meta = Metadata {
            name: "from_private_key_test".to_string(),
            source: Source::Private,
            ..Metadata::default()
        };
        let keystore = PrivateKeystore::from_private_key(
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            TEST_PASSWORD,
            meta,
        );
        assert_eq!(keystore.store.version, 11001);
        assert_ne!(keystore.store.id, "");
        assert_eq!(keystore.store.active_accounts.len(), 0);
    }
}
