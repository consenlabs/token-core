pub mod ctr {
    use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
    use bitcoin_hashes::hex::ToHex;
    use crypto::{aes, aes::KeySize, symmetriccipher::{Decryptor, Encryptor}};

    pub fn encrypt_nopadding(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut encrypter = aes::ctr(KeySize::KeySize128, key, &iv);
        let mut buffer_reader = RefReadBuffer::new(&data);
        let mut ret = vec![0u8; data.len()];
        let mut buffer_writer = RefWriteBuffer::new(&mut ret);
        encrypter.encrypt(&mut buffer_reader, &mut buffer_writer, true);
        return ret;
    }

    pub fn decrypt_nopadding(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut decryptor = aes::ctr(KeySize::KeySize128, key, &iv);
        let mut buffer_reader = RefReadBuffer::new(&data);
        let mut ret = vec![0u8; data.len()];
        let mut buffer_writer = RefWriteBuffer::new(&mut ret);
        decryptor.decrypt(&mut buffer_reader, &mut buffer_writer, true);
        return ret;
    }
}