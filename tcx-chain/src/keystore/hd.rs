use bip39::{Language, Mnemonic, Seed};

use uuid::Uuid;

use super::Account;
use super::Address;
use super::Result;
use super::{Error, Metadata};

use crate::keystore::{transform_mnemonic_error, Store};

use std::collections::HashMap;

use tcx_constants::{CoinInfo, CurveType};
use tcx_crypto::hash::dsha256;
use tcx_crypto::{Crypto, Key, Pbkdf2Params};
use tcx_primitive::{
    generate_mnemonic, get_account_path, Derive, ToHex, TypedDeterministicPrivateKey,
    TypedDeterministicPublicKey, TypedPrivateKey,
};

struct Cache {
    mnemonic: String,
    keys: HashMap<String, TypedDeterministicPrivateKey>,
}

pub struct HdKeystore {
    store: Store,
    cache: Option<Cache>,
}

pub fn key_hash_from_mnemonic(mnemonic: &str) -> Result<String> {
    let mn =
        Mnemonic::from_phrase(mnemonic, Language::English).map_err(transform_mnemonic_error)?;

    let seed = Seed::new(&mn, "");

    let bytes = dsha256(seed.as_bytes())[..20].to_vec();
    Ok(hex::encode(bytes))
}

impl HdKeystore {
    pub const VERSION: i64 = 11000i64;

    pub(crate) fn store(&self) -> &Store {
        &self.store
    }

    pub(crate) fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    pub(crate) fn from_store(store: Store) -> Self {
        HdKeystore { store, cache: None }
    }

    pub(crate) fn unlock_by_password(&mut self, password: &str) -> Result<()> {
        let mnemonic_bytes = self
            .store
            .crypto
            .decrypt(Key::Password(password.to_owned()))?;
        self.cache_mnemonic(mnemonic_bytes)
    }

    pub(crate) fn unlock_by_derived_key(&mut self, derived_key: &str) -> Result<()> {
        let mnemonic_bytes = self
            .store
            .crypto
            .decrypt(Key::DerivedKey(derived_key.to_owned()))?;
        self.cache_mnemonic(mnemonic_bytes)
    }

    fn cache_mnemonic(&mut self, mnemonic_bytes: Vec<u8>) -> Result<()> {
        let mnemonic_str = String::from_utf8(mnemonic_bytes)?;

        let _mnemonic = Mnemonic::from_phrase(&mnemonic_str, Language::English)
            .map_err(transform_mnemonic_error)?;

        self.cache = Some(Cache {
            mnemonic: mnemonic_str,
            keys: HashMap::new(),
        });

        Ok(())
    }

    pub(crate) fn lock(&mut self) {
        self.cache = None;
    }

    pub(crate) fn is_locked(&self) -> bool {
        self.cache.is_none()
    }

    pub(crate) fn mnemonic(&self) -> Result<String> {
        let cache = self.cache.as_ref().ok_or(Error::KeystoreLocked)?;

        Ok(cache.mnemonic.to_string())
    }

    pub(crate) fn find_private_key(&self, symbol: &str, address: &str) -> Result<TypedPrivateKey> {
        let cache = self.cache.as_ref().ok_or(Error::KeystoreLocked)?;

        let account = self
            .account(symbol, address)
            .ok_or(Error::AccountNotFound)?;

        let root = TypedDeterministicPrivateKey::from_mnemonic(account.curve, &cache.mnemonic)?;

        Ok(root.derive(&account.derivation_path)?.private_key())
    }

    pub(crate) fn find_deterministic_public_key(
        &mut self,
        symbol: &str,
        address: &str,
    ) -> Result<TypedDeterministicPublicKey> {
        let account = self
            .account(symbol, address)
            .ok_or(Error::AccountNotFound)?;

        TypedDeterministicPublicKey::from_hex(account.curve, &account.ext_pub_key)
    }

    pub(crate) fn find_private_key_by_path(
        &mut self,
        symbol: &str,
        main_address: &str,
        relative_path: &str,
    ) -> Result<TypedPrivateKey> {
        let cache = self.cache.as_ref().ok_or(Error::KeystoreLocked)?;

        if !cache.keys.contains_key(main_address) {
            let account = self
                .account(symbol, main_address)
                .ok_or(Error::AccountNotFound)?;

            let esk = TypedDeterministicPrivateKey::from_mnemonic(account.curve, &cache.mnemonic)?;

            let k = esk.derive(&get_account_path(&account.derivation_path)?)?;

            self.cache
                .as_mut()
                .unwrap()
                .keys
                .insert(main_address.to_owned(), k);
        }

        let esk = &self.cache.as_ref().unwrap().keys[main_address];

        Ok(esk.derive(relative_path)?.private_key())
    }

    pub fn new(password: &str, meta: Metadata) -> HdKeystore {
        let mnemonic = generate_mnemonic();

        Self::from_mnemonic(&mnemonic, password, meta).unwrap()
    }

    pub fn from_mnemonic(mnemonic: &str, password: &str, meta: Metadata) -> Result<HdKeystore> {
        let mnemonic: &str = &mnemonic.split_whitespace().collect::<Vec<&str>>().join(" ");

        let key_hash = key_hash_from_mnemonic(mnemonic)?;

        let crypto: Crypto<Pbkdf2Params> = Crypto::new(password, mnemonic.as_bytes());
        Ok(HdKeystore {
            store: Store {
                key_hash,
                crypto,
                id: Uuid::new_v4().to_hyphenated().to_string(),
                version: Self::VERSION,
                active_accounts: vec![],
                meta,
            },

            cache: None,
        })
    }

    pub(crate) fn derive_coin<A: Address>(&mut self, coin_info: &CoinInfo) -> Result<Account> {
        let cache = self.cache.as_ref().ok_or(Error::KeystoreLocked)?;

        let root = TypedDeterministicPrivateKey::from_mnemonic(coin_info.curve, &cache.mnemonic)?;

        let private_key = root.derive(&coin_info.derivation_path)?.private_key();
        let public_key = private_key.public_key();

        let address = A::from_public_key(&public_key, coin_info)?;
        // todo: ext_pub_key
        let ext_pub_key = match coin_info.curve {
            CurveType::SubSr25519 => "".to_owned(),
            CurveType::ED25519 => "".to_owned(), //TODO
            _ => root
                .derive(&get_account_path(&coin_info.derivation_path)?)?
                .deterministic_public_key()
                .to_hex(),
        };
        // let ext_pub_key = root
        //     .derive(&account_path)?
        //     .deterministic_public_key()
        //     .to_hex();

        let account = Account {
            address,
            derivation_path: coin_info.derivation_path.to_string(),
            curve: coin_info.curve,
            coin: coin_info.coin.to_string(),
            network: coin_info.network.to_string(),
            ext_pub_key,
            seg_wit: coin_info.seg_wit.to_string(),
        };

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

    pub(crate) fn account(&self, symbol: &str, address: &str) -> Option<&Account> {
        self.store
            .active_accounts
            .iter()
            .find(|acc| acc.address == address && acc.coin == symbol)
    }

    pub(crate) fn verify_password(&self, password: &str) -> bool {
        self.store.crypto.verify_password(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keystore::metadata_default_time;

    use crate::Source;
    use std::string::ToString;
    use tcx_constants::{CurveType, TEST_MNEMONIC, TEST_PASSWORD};
    use tcx_primitive::TypedPublicKey;

    // A mnemonic word separated by a full-width or half-width space
    static MNEMONIC_WITH_WHITESPACE: &'static str =
        "inject　 kidney    empty   canal shadow   pact comfort wife crush horse wife sketch";
    static INVALID_MNEMONIC1: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife inject";
    static INVALID_MNEMONIC2: &'static str =
        "invalid_word kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static INVALID_MNEMONIC_LEN: &'static str =
        "inject kidney empty canal shadow pact comfort wife crush horse wife";
    #[test]
    pub fn default_meta() {
        let meta = Metadata::default();
        let expected = Metadata {
            name: String::from("Unknown"),
            password_hint: String::new(),
            timestamp: metadata_default_time(),
            source: Source::Mnemonic,
        };

        assert_eq!(meta.name, expected.name);
        assert_eq!(meta.password_hint, expected.password_hint);
        assert_eq!(meta.source, expected.source);
    }

    struct MockAddress {}
    impl Address for MockAddress {
        fn from_public_key(_pk: &TypedPublicKey, _coin: &CoinInfo) -> Result<String> {
            Ok("mock_address".to_string())
        }

        fn is_valid(_address: &str, _coin: &CoinInfo) -> bool {
            true
        }
    }

    #[test]
    pub fn new_keystore() {
        let keystore = HdKeystore::new(TEST_PASSWORD, Metadata::default());
        let store = keystore.store;

        assert_eq!(store.version, 11000);
        assert_ne!(store.id, "");
        assert_eq!(store.active_accounts.len(), 0);
    }

    #[test]
    pub fn from_invalid_mnemonic() {
        let invalid_mnemonic = vec![
            (INVALID_MNEMONIC1, "mnemonic_checksum_invalid"),
            (INVALID_MNEMONIC2, "mnemonic_word_invalid"),
            (INVALID_MNEMONIC_LEN, "mnemonic_length_invalid"),
        ];
        for (mn, err) in invalid_mnemonic {
            let ks = HdKeystore::from_mnemonic(mn, TEST_PASSWORD, Metadata::default());
            assert!(ks.is_err());
            assert_eq!(err, format!("{}", ks.err().unwrap()));
        }
    }

    #[test]
    pub fn from_blank_space_mnemonic() {
        let mut keystore =
            HdKeystore::from_mnemonic(MNEMONIC_WITH_WHITESPACE, TEST_PASSWORD, Metadata::default())
                .unwrap();
        let coin_info = CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let _ = keystore.unlock_by_password(TEST_PASSWORD).unwrap();

        let acc = keystore.derive_coin::<MockAddress>(&coin_info).unwrap();

        let expected = Account {
            address: "mock_address".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            ext_pub_key: "03a25f12b68000000044efc688fe25a1a677765526ed6737b4bfcfb0122589caab7ca4b223ffa9bb37029d23439ecb195eb06a0d44a608960d18702fd97e19c53451f0548f568207af77".to_string(),
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
            curve: CurveType::SECP256k1,
            coin: "BITCOIN".to_string(),
        };

        assert_eq!(acc, expected);
        assert_eq!(
            keystore.account("BITCOIN", "mock_address").unwrap(),
            &expected
        );
    }

    #[test]
    pub fn from_mnemonic() {
        let mut keystore =
            HdKeystore::from_mnemonic(TEST_MNEMONIC, TEST_PASSWORD, Metadata::default()).unwrap();
        assert_eq!(keystore.store.version, 11000);
        assert_ne!(keystore.store.id, "");
        let decrypted_bytes = keystore
            .store
            .crypto
            .decrypt(Key::Password(TEST_PASSWORD.to_owned()))
            .unwrap();
        let decrypted_mnemonic = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(decrypted_mnemonic, TEST_MNEMONIC);
        assert_eq!(keystore.store.active_accounts.len(), 0);

        keystore.unlock_by_password(TEST_PASSWORD).unwrap();

        let mnemonic = keystore.mnemonic().unwrap();
        assert_eq!(mnemonic, TEST_MNEMONIC);

        let wrong_password_err = keystore.unlock_by_password("WrongPassword").err().unwrap();
        assert_eq!(format!("{}", wrong_password_err), "password_incorrect");
    }

    //    #[test]
    //    pub fn generate_seed() {
    //        let mnemonic = Mnemonic::from_phrase(
    //            "favorite liar zebra assume hurt cage any damp inherit rescue delay panic",
    //            Language::English,
    //        )
    //        .unwrap();
    //
    //        //        let entropy = mnemonic.entropy();
    //
    //        let seed = bip39::Seed::new(&mnemonic, &"").as_bytes().to_vec();
    //
    //        assert_eq!(
    //            "235c69907d33b85f27bd78e73ff5d0c67bd4894515cc30c77f4391859bc1a3f2",
    //            hex::encode(seed)
    //        );
    //    }

    #[test]
    pub fn derive_key_at_paths() {
        let mut keystore =
            HdKeystore::from_mnemonic(TEST_MNEMONIC, TEST_PASSWORD, Metadata::default()).unwrap();
        let coin_info = CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let _ = keystore.unlock_by_password(TEST_PASSWORD).unwrap();

        let acc = keystore.derive_coin::<MockAddress>(&coin_info).unwrap();

        let expected = Account {
            address: "mock_address".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            ext_pub_key: "03a25f12b68000000044efc688fe25a1a677765526ed6737b4bfcfb0122589caab7ca4b223ffa9bb37029d23439ecb195eb06a0d44a608960d18702fd97e19c53451f0548f568207af77".to_string(),
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
            curve: CurveType::SECP256k1,
            coin: "BITCOIN".to_string(),
        };

        assert_eq!(acc, expected);
        assert_eq!(
            keystore.account("BITCOIN", "mock_address").unwrap(),
            &expected
        );
        assert_eq!(keystore.store.active_accounts.len(), 1);
    }
}
