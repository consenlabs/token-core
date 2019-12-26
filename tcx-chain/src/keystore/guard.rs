use super::Keystore;
use super::Result;

pub struct KeystoreGuard<'a> {
    keystore: &'a mut Keystore,
}

impl<'a> Drop for KeystoreGuard<'a> {
    fn drop(&mut self) {
        self.keystore.lock();
    }
}

impl<'a> KeystoreGuard<'a> {
    pub fn unlock_by_password(ks: &'a mut Keystore, password: &str) -> Result<KeystoreGuard<'a>> {
        ks.unlock_by_password(password)?;

        Ok(KeystoreGuard { keystore: ks })
    }

    pub fn keystore_mut(&mut self) -> &mut Keystore {
        self.keystore
    }
}
