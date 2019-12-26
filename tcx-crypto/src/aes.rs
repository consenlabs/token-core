pub mod ctr {
    use aes_ctr::Aes128Ctr;

    use crate::{Error, Result};
    use aes_ctr::stream_cipher::generic_array::GenericArray;
    use aes_ctr::stream_cipher::{NewStreamCipher, SyncStreamCipher};

    pub fn encrypt_nopadding(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 16 || iv.len() != 16 {
            return Err(Error::InvalidKeyIvLength.into());
        }
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut cipher = Aes128Ctr::new(key, iv);
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        cipher.apply_keystream(&mut data_copy);
        Ok(data_copy)
    }

    pub fn decrypt_nopadding(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 16 || iv.len() != 16 {
            return Err(Error::InvalidKeyIvLength.into());
        }
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut cipher = Aes128Ctr::new(key, iv);
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        cipher.apply_keystream(&mut data_copy);
        Ok(data_copy)
    }
}

pub mod cbc {
    extern crate aes_soft;
    extern crate block_modes;
    use crate::Error;
    use crate::Result;
    use aes_soft::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    pub fn encrypt_pkcs7(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes128Cbc::new_var(key, iv)?;
        Ok(cipher.encrypt_vec(data))
    }

    pub fn decrypt_pkcs7(encrypted: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes128Cbc::new_var(key, iv)?;
        cipher
            .decrypt_vec(encrypted)
            .map_err(|_| Error::InvalidCiphertext.into())
    }
}

#[cfg(test)]
mod tests {

    use crate::aes::cbc::{decrypt_pkcs7, encrypt_pkcs7};
    use crate::aes::ctr::{decrypt_nopadding, encrypt_nopadding};
    use bitcoin_hashes::hex::ToHex;

    #[test]
    fn encrypt_nopadding_test() {
        let data = "TokenCoreX".as_bytes();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = encrypt_nopadding(&data, &key, &iv).expect("encrypt nopadding data");
        let ret_hex = ret.to_hex();

        assert_eq!("e19e6c5923d33c587cf8", ret_hex);

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt_nopadding(&data, &wrong_len_key, &iv);
        assert!(ret.is_err());

        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt_nopadding(&data, &key, &wrong_len_iv);
        assert!(ret.is_err());
    }

    #[test]
    fn decrypted_data_test() {
        let data = "TokenCoreX".as_bytes();
        let encrypted_data = hex::decode("e19e6c5923d33c587cf8").unwrap();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = decrypt_nopadding(&encrypted_data, &key, &iv).expect("decrypted data error");

        assert_eq!(
            "TokenCoreX",
            String::from_utf8(ret).expect("decrypted failed")
        );

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = decrypt_nopadding(&data, &wrong_len_key, &iv);
        assert!(ret.is_err());

        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = decrypt_nopadding(&data, &key, &wrong_len_iv);
        assert!(ret.is_err());
    }

    #[test]
    fn encrypt_pkcs7_test() {
        let data = "TokenCoreX".as_bytes();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = encrypt_pkcs7(&data, &key, &iv).expect("encrypt_pkcs7");
        let ret_hex = ret.to_hex();

        assert_eq!("13d567987d7eced9c2154551bc37bc5f", ret_hex);

        let decrypted = decrypt_pkcs7(&ret, &key, &iv).unwrap();
        assert_eq!("TokenCoreX", String::from_utf8(decrypted).unwrap());

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt_pkcs7(&data, &wrong_len_key, &iv);

        assert!(ret.is_err());
        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt_pkcs7(&data, &key, &wrong_len_iv);
        assert!(ret.is_err());
    }
}
