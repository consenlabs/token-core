use super::Keystore;
use super::Result;

pub struct KeystoreGuard<'a> {
    keystore: Box<&'a mut dyn Keystore>,
}

impl<'a> Drop for KeystoreGuard<'a> {
    fn drop(&mut self) {
        //        self.keystore.lock();
    }
}

impl<'a> KeystoreGuard<'a> {
    pub fn unlock_by_password(
        ks: &'a mut dyn Keystore,
        password: &str,
    ) -> Result<KeystoreGuard<'a>> {
        ks.unlock_by_password(password)?;

        Ok(KeystoreGuard {
            keystore: Box::new(ks),
        })
    }

    /*
    pub fn keystore_mut(&mut self) -> &mut dyn Keystore {
        //self.keystore.as_mut()
    }

    pub fn keystore(&self) -> &dyn Keystore {
        //self.keystore.as_ref()
    }
    */
}
