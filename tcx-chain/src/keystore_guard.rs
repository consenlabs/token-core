use crate::HdKeystore;
use crate::Result;

pub struct KeystoreGuard<'a> {
    keystore: &'a mut HdKeystore,
}

impl<'a> Drop for KeystoreGuard<'a> {
    fn drop(&mut self) {
        self.keystore.lock();
    }
}

impl<'a> KeystoreGuard<'a> {
    pub fn unlock_by_password(ks: &'a mut HdKeystore, password: &str) -> Result<KeystoreGuard<'a>> {
        ks.unlock_by_password(password)?;

        Ok(KeystoreGuard { keystore: ks })
    }

    pub fn keystore_mut(&mut self) -> &mut HdKeystore {
        self.keystore
    }

    pub fn keystore(&self) -> &HdKeystore {
        self.keystore
    }
}
