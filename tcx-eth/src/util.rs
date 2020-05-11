use crate::singer::SignatureData;
use num_bigint::BigInt;
use tcx_constants::Result;
use tcx_primitive::{
    Bip32DeterministicPublicKey, Derive, DerivePath, DeterministicPublicKey, FromHex, PrivateKey,
    PublicKey, TypedDeterministicPublicKey, TypedPrivateKey,
};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "invalid_big_num_in_hex# hex: {}", _0)]
    InvalidBigNumInHex(String),
    #[fail(display = "invalid_big_num_in_num# address: {}", _0)]
    InvalidBigNumInDigit(String),
}

pub fn parse_big_int(value: &str) -> Result<BigInt> {
    // todo check is hex or digit
    if value.starts_with("0x") || value.starts_with("0X") {
        let value_without_0x = &value[2..];
        let bytes = hex::decode(value_without_0x)?;
        return BigInt::parse_bytes(&bytes, 16)
            .ok_or(Error::InvalidBigNumInHex(value.to_string()).into());
    } else {
        return BigInt::parse_bytes(&value.as_bytes(), 10)
            .ok_or(Error::InvalidBigNumInDigit(value.to_string()).into());
    }
}

pub fn calc_recover_id(
    sig_data: &SignatureData,
    value: &[u8],
    prv_key: &TypedPrivateKey,
) -> Result<u32> {
    let mut rec_id = 0u32;
    for i in 0..4 {
        let mut sig_data_with_recover_id: SignatureData = sig_data.clone();
        sig_data_with_recover_id.v = i;
        let pub_key = prv_key.recover(value, &sig_data_with_recover_id.to_vec())?;
        if pub_key == prv_key.as_secp256k1()?.public_key().to_uncompressed() {
            return Ok(i + 27);
        }
    }
    Err(format_err!(
        "Could not construct a recoverable key. This should never happen."
    ))
}

pub fn data_to_bytes(data: &str) -> Vec<u8> {
    if is_hex(data) {
        hex::decode(data).expect("is_hex function is sure the data is hex")
    } else {
        data.as_bytes().to_vec()
    }
}

pub fn is_hex(data: &str) -> bool {
    if data.to_ascii_lowercase().starts_with("0x") {
        return hex::decode(&data[2..]).is_ok();
    } else {
        return hex::decode(data).is_ok();
    }
}
