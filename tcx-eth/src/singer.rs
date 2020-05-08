use tcx_chain::{Keystore, TransactionSigner};

use crate::address::EthAddress;
use tcx_primitive::{
    Bip32DeterministicPublicKey, Derive, DerivePath, DeterministicPublicKey, FromHex, PrivateKey,
    PublicKey, TypedDeterministicPublicKey, TypedPrivateKey,
};

use crate::transaction::{EthTxInput, EthTxOutput};
use keccak_hash::keccak256;
use num_bigint::{BigInt, BigUint};
use rlp::{Rlp, RlpStream};
use tcx_constants::Result;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "invalid_big_num_in_hex# hex: {}", _0)]
    InvalidBigNumInHex(String),
    #[fail(display = "invalid_big_num_in_num# address: {}", _0)]
    InvalidBigNumInDigit(String),
}

struct SignatureData {
    v: int32,
    r: Vec<u8>,
    s: Vec<u8>,
}

// private static SignatureData createEip155SignatureData(SignatureData signatureData, int chainId) {
// int v = signatureData.getV() + (chainId * 2) + 8;
//
// return new SignatureData(v, signatureData.getR(), signatureData.getS());
// }

impl SignatureData {
    fn to_eip155(&self, chain_id: int32) -> Self {
        let v = self.v + (chain_id * 2) + 8;
        return SignatureData {
            v,
            r: self.r.clone(),
            s: self.s.clone(),
        };
    }

    fn from_raw(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 65 {
            return format_err!("invalid raw signature");
        }
        return Ok(SignatureData {
            v: bytes[64] as i32,
            r: bytes[0..32].to_vec(),
            s: bytes[33..64].to_vec(),
        });
    }
}

impl Default for SignatureData {
    fn default() -> Self {
        SignatureData {
            v: 0,
            r: vec![],
            s: vec![],
        }
    }
}

impl EthTxInput {
    // fn rlp_encode_without_sig(&self) -> Result<RlpStream> {
    //     let mut ret:  rlp::RlpStream= RlpStream::new();
    //     let nonce_bi = parse_big_int(&self.nonce)?;
    //     ret.append(&nonce_bi.to_bytes_le());
    //     let gas_price_bi = parse_big_int(&self.gas_price)?;
    //     ret.append(&gas_price_bi.to_bytes_le());
    //     let gas_limit_bi = parse_big_int(&self.gas_limit)?;
    //     ret.append(&gas_limit_bi.to_bytes_le());
    //     ret.append(&self.to);
    //     let value_bi = parse_big_int(&self.value)?;
    //     ret.append(&value_bi.to_bytes_le());
    //
    //     let data_bytes = hex::decode(&self.data)?;
    //     ret.append(&data_bytes);
    //
    //
    //     Ok(ret)
    // }
    //
    // fn append_sig_to_rlp(stream: &RlpStream, signature_data: &SignatureData) -> Result<RlpStream> {
    //     let mut ret: RlpStream = RlpStream::from(stream)
    //     if signature_data.v > 0 {
    //         // r and s should be trim leading zeros
    //         stream.append(&signature_data.v);
    //         stream.append(&signature_data.r);
    //         stream.append(&signature_data.s);
    //     }
    //     Ok(stream)
    // }

    fn encode_to_rlp(&self, signature_data: Option<&SignatureData>) -> Result<Vec<u8>> {
        let mut ret: rlp::RlpStream = RlpStream::new();
        let nonce_bi = parse_big_int(&self.nonce)?;
        ret.append(&nonce_bi.to_bytes_le());
        let gas_price_bi = parse_big_int(&self.gas_price)?;
        ret.append(&gas_price_bi.to_bytes_le());
        let gas_limit_bi = parse_big_int(&self.gas_limit)?;
        ret.append(&gas_limit_bi.to_bytes_le());
        ret.append(&self.to);
        let value_bi = parse_big_int(&self.value)?;
        ret.append(&value_bi.to_bytes_le());

        let data_bytes = hex::decode(&self.data)?;
        ret.append(&data_bytes);

        if let Some(sig) = signature_data {
            // r and s should be trim leading zeros
            ret.append(&sig.v);
            ret.append(&sig.r);
            ret.append(&sig.s);
        }
        Ok(ret.out())
    }
}

fn parse_big_int(value: &str) -> Result<BigInt> {
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

fn calc_recover_id(sig_data: &SignatureData, value: &[u8], prv_key: &TypedPrivateKey) -> int32 {
    // int recId = -1;
    // for (int i = 0; i < 4; i++) {
    //     ECKey recoverKey = ECKey.recoverFromSignature(i, sig, Sha256Hash.wrap(value), compressed);
    //     if (recoverKey != null && recoverKey.getPubKeyPoint().equals(ecKey.getPubKeyPoint())) {
    //         recId = i;
    //         break;
    //     }
    // }
    // if (recId == -1) {
    //     throw new RuntimeException(
    //         "Could not construct a recoverable key. This should never happen.");
    // }
    //
    // int headerByte = recId + 27;
    // if (compressed) {
    //     headerByte += 4;
    // }
    //
    // // 1 header + 32 bytes for R + 32 bytes for S
    // byte v = (byte) headerByte;
    // byte[] r = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.r, 32);
    // byte[] s = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.s, 32);
    //
    // return new SignatureData(v, r, s);
    let mut rec_id = -1i32;
    for i in 0..4 {
        prv_key
    }
}
impl TransactionSigner<EthTxInput, EthTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &EthTxInput,
    ) -> Result<EthTxOutput> {
        // let sig_data = SignatureData::default();
        let encoded_tx = tx.encode_to_rlp(None)?;
        let mut hashed_tx = [0u8; 32];
        keccak_hash::keccak_256(&encoded_tx, &mut hashed_tx);
        let pk = self.find_private_key(symbol, address)?;
        let signed_tx = pk.sign(&encoded_tx)?;
        let sig_data = SignatureData::from_raw(&signed_tx)?;

        // let sig_data =
    }
}
