pub mod ctr {
    use aes_ctr::Aes128Ctr;

    use aes_ctr::stream_cipher::generic_array::GenericArray;
    use aes_ctr::stream_cipher::{
        NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
    };
    use core::borrow::BorrowMut;

    pub fn encrypt_nopadding(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut cipher = Aes128Ctr::new(key, iv);
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        cipher.apply_keystream(&mut data_copy);
        return Vec::from(data_copy);
//        let mut encrypter = aes::ctr(KeySize::KeySize128, key, &iv);
//        let mut buffer_reader = RefReadBuffer::new(&data);
//        let mut ret = vec![0u8; data.len()];
//        let mut buffer_writer = RefWriteBuffer::new(&mut ret);
//        encrypter.encrypt(&mut buffer_reader, &mut buffer_writer, true);
//        return ret;
    }

    pub fn decrypt_nopadding(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
//        let mut decryptor = aes::ctr(KeySize::KeySize128, key, &iv);
//        let mut buffer_reader = RefReadBuffer::new(&data);
//        let mut ret = vec![0u8; data.len()];
//        let mut buffer_writer = RefWriteBuffer::new(&mut ret);
//        decryptor.decrypt(&mut buffer_reader, &mut buffer_writer, true);
//        return ret;
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut cipher = Aes128Ctr::new(key, iv);
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        cipher.apply_keystream(&mut data_copy);
        return Vec::from(data_copy);
    }

}

pub mod cbc {
    extern crate aes_soft;
    extern crate block_modes;
    use block_modes::{BlockMode, Cbc};
    use block_modes::block_padding::Pkcs7;
    use aes_soft::Aes128;

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    pub fn encrypt_pkcs7(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
//        let key = GenericArray::from_slice(key);
//        let iv = GenericArray::from_slice(iv);
//        let mut cipher = ::new(key, iv);
//        let mut data_copy = vec![0; data.len()];
//        data_copy.copy_from_slice(data);
//        cipher.apply_keystream(&mut data_copy);
//        return Vec::from(data_copy);
//        let mut encrypter = aes::ctr(KeySize::KeySize128, key, &iv);
//        let mut buffer_reader = RefReadBuffer::new(&data);
//        let mut ret = vec![0u8; data.len()];
//        let mut buffer_writer = RefWriteBuffer::new(&mut ret);
//        encrypter.encrypt(&mut buffer_reader, &mut buffer_writer, true);
//        return ret;
        let cipher = Aes128Cbc::new_var(key, iv).unwrap();
        let ciphertext = cipher.encrypt_vec(data);
        return ciphertext;
    }
}