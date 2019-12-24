use super::Account;
use super::{Address, Metadata, Source};
use tcx_constants::CoinInfo;
use tcx_crypto::{Crypto, Pbkdf2Params};

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
        self.private_key = Some(self.decrypt_private_key(password)?);

        Ok(())
    }

    pub(crate) fn lock(&mut self) {
        self.private_key = None;
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

    pub fn from_private_key(private_key: &str, password: &str, source: Source) -> PrivateKeystore {
        let key_data: Vec<u8> = hex::decode(private_key).expect("hex can't decode");
        let key_hash = key_hash_from_private_key(&key_data);
        //        let pk_bytes = hex::decode(private_key).expect("valid private_key");
        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, &key_data);

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

    pub(crate) fn private_key_to_account<A: Address>(
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

    pub(crate) fn private_key(&self) -> Result<String> {
        tcx_ensure!(self.private_key.is_some(), Error::KeystoreLocked);
        let vec = self.private_key.as_ref().unwrap().to_vec();
        Ok(hex::encode(&vec))
    }

    fn decrypt_private_key(&self, password: &str) -> Result<Vec<u8>> {
        self.store.crypto.decrypt(password)
    }
}

#[cfg(test)]
mod tests {
    use crate::{PrivateKeystore, Source};
    use tcx_constants::TEST_PASSWORD;

    static PK_STORE: &'static str = r#"
    {"id":"cb1ba2d7-7b89-4595-9753-d16b6e317c6b","version":11001,"keyHash":"4fc213ddcb6fa44a2e2f4c83d67502f88464e6ee","crypto":{"cipher":"aes-128-ctr","cipherparams":{"iv":"21cb134b52e3d76f6b0d287c884c27fb"},"ciphertext":"ce7df149b0a010165cc7bf2fdc8104f7dc0d131022aa221e5f4b909aa11ba7aa","kdf":"pbkdf2","kdfparams":{"c":1024,"prf":"hmac-sha256","dklen":32,"salt":"737cbf8e446e32fe4174cbc97efebe101d253013fcfe981b9220c99f22f4bb4e"},"mac":"a03fa095a9f36e0e12936e4d39ab3b942537ec1080fe77a30724f966f092a662"},"activeAccounts":[{"address":"TXo4VDm8Qc5YBSjPhu8pMaxzTApSvLshWG","derivationPath":"","curve":"SECP256k1","coin":"TRON","network":"","segWit":"","extPubKey":""}],"imTokenMeta":{"name":"Unknown","passwordHint":"","timestamp":1576654549,"source":"PRIVATE"}}
    "#;

    #[test]
    pub fn from_private_key_test() {
        let keystore = PrivateKeystore::from_private_key(
            "a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6",
            TEST_PASSWORD,
            Source::Private,
        );
        assert_eq!(keystore.store.version, 11001);
        assert_ne!(keystore.store.id, "");
        assert_eq!(keystore.store.active_accounts.len(), 0);
    }
}
