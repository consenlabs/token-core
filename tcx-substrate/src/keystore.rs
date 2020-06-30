use std::convert::TryInto;
use tcx_constants::Result;
use xsalsa20poly1305::aead::{generic_array::GenericArray, Aead, NewAead};
use xsalsa20poly1305::XSalsa20Poly1305;

const NONCE_LENGTH: usize = 24;
const PKCS8_DIVIDER: [u8; 5] = [161, 35, 3, 33, 0];
const PKCS8_HEADER: [u8; 16] = [48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32];
const PUB_LENGTH: usize = 32;
const SEC_LENGTH: usize = 64;
const SEED_OFFSET: usize = PKCS8_HEADER.len();
const SEED_LENGTH: usize = 32;

// const PKCS8_HEADER: [u8;16] =[48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32];
// const

fn decrypt_content(password: &str, encrypted: &str) -> Result<String> {
    let encrypted_bytes = hex::decode(encrypted)?;
    let nonce: &[u8; 24] = &encrypted_bytes[0..NONCE_LENGTH].try_into().unwrap();
    let ciphertext = &encrypted_bytes[NONCE_LENGTH..];
    let padding_password = password_to_key(password);
    let key = GenericArray::from_slice(&padding_password);
    let cipher = XSalsa20Poly1305::new(key);
    let nonce = GenericArray::from_slice(nonce);
    let encoded = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format_err!("{}", "decrypt error"))?;

    let header = &encoded[0..PKCS8_HEADER.len()];

    assert!(header == PKCS8_HEADER, "Invalid Pkcs8 header found in body");

    let mut secret_key = &encoded[PKCS8_HEADER.len()..PKCS8_HEADER.len() + SEC_LENGTH];
    let mut div_offset: usize = SEED_OFFSET + SEC_LENGTH;
    let mut divider = &encoded[div_offset..div_offset + PKCS8_DIVIDER.len()];
    if divider != PKCS8_DIVIDER {
        div_offset = SEED_OFFSET + SEED_LENGTH;
        secret_key = &encoded[SEED_OFFSET..div_offset];
        divider = &encoded[div_offset..div_offset + PKCS8_DIVIDER.len()];
    }

    assert!(
        divider == PKCS8_DIVIDER,
        "Invalid Pkcs8 divider found in body"
    );

    Ok(hex::encode(secret_key))
}

fn password_to_key(password: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let password_bytes = password.as_bytes();
    let pwd_len = password_bytes.len();
    let iter_len = if pwd_len > 32 { 0 } else { pwd_len };
    for idx in 0..iter_len {
        key[idx] = password_bytes[idx]
    }
    key
}

#[cfg(test)]
mod test_super {
    use super::*;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::FromHex;

    #[test]
    fn test_decrypt_from_keystore() {
        let encoded = "d80bcaf72c744d5a9a6c4229280e360d98d408afbe67232c3418a2a591b3f2bf468a319b7e5c1717bb8285619a76584a7961eac2183f94cfa56ad975cb78ae87b4dc18e7c20036bd448aa52c5ee7a45c4cdf41923c8133d6bfc29c737b65dcfb357884b55fb36d4762446fb26bfd8fce49142cf0e7d3642e2095ea6e425a8e923629306875c36b72a82d517478a19c8786b1be611e77286ba6448bf93c";
        let decrypted = decrypt_content("testing", encoded).unwrap();
        assert_eq!(decrypted, "416c696365202020202020202020202020202020202020202020202020202020d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4f");
    }
}
