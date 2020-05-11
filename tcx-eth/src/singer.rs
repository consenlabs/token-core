use tcx_chain::{Keystore, TransactionSigner};

use crate::address::EthAddress;
use tcx_primitive::{
    Bip32DeterministicPublicKey, Derive, DerivePath, DeterministicPublicKey, FromHex, PrivateKey,
    PublicKey, TypedDeterministicPublicKey, TypedPrivateKey,
};

use crate::transaction::{EthTxInput, EthTxOutput};
use crate::util::*;
use failure::Fail;
use num_bigint::{BigInt, BigUint};
use rlp::{Rlp, RlpStream};
use secp256k1::Signature;
use tcx_constants::Result;

pub struct SignatureData {
    pub v: u32,
    r: Vec<u8>,
    s: Vec<u8>,
}

impl SignatureData {
    pub fn to_eip155(&self, chain_id: u32) -> Self {
        let v = self.v + (chain_id * 2) + 8;
        return SignatureData {
            v,
            r: self.r.clone(),
            s: self.s.clone(),
        };
    }

    pub fn new(chain_id: u32) -> Self {
        return SignatureData {
            v: chain_id,
            r: vec![],
            s: vec![],
        };
    }

    pub fn from_raw(bytes: &[u8]) -> Result<Self> {
        let v: i32;
        let v = match bytes.len() {
            65 => Ok(bytes[64] as u32),
            64 => Ok(0u32),
            _ => Err(format_err!("invalid raw signature")),
        }?;
        return Ok(SignatureData {
            v,
            r: bytes[0..32].to_vec(),
            s: bytes[32..64].to_vec(),
        });
    }

    pub fn from_rsv(rs: &[u8], v: i32) -> Result<Self> {
        let rsv = [rs, &vec![v as u8]].concat();
        Self::from_raw(&rsv)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        [self.r.clone(), self.s.clone(), vec![self.v as u8]].concat()
    }
}

impl Clone for SignatureData {
    fn clone(&self) -> Self {
        SignatureData {
            v: self.v,
            r: self.r.clone(),
            s: self.s.clone(),
        }
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
    fn encode_to_rlp(&self, signature_data: &SignatureData) -> Result<Vec<u8>> {
        let list_len: usize = if signature_data.v > 0 { 9 } else { 6 };
        let mut ret: rlp::RlpStream = RlpStream::new_list(list_len);
        let nonce_bi = parse_big_int(&self.nonce)?;
        ret.append(&nonce_bi.to_bytes_be().1);
        let gas_price_bi = parse_big_int(&self.gas_price)?;
        ret.append(&gas_price_bi.to_bytes_be().1);
        let gas_limit_bi = parse_big_int(&self.gas_limit)?;
        ret.append(&gas_limit_bi.to_bytes_be().1);
        let address_bytes = hex::decode(&self.to)?;
        ret.append(&address_bytes);
        let value_bi = parse_big_int(&self.value)?;
        ret.append(&value_bi.to_bytes_be().1);

        let data_bytes = hex::decode(&self.data)?;
        ret.append(&data_bytes);

        if signature_data.v > 0 {
            ret.append(&(signature_data.v as u32));
            ret.append(&signature_data.r);
            ret.append(&signature_data.s);
        }
        Ok(ret.out())
    }

    fn sign_tx_by_pk(&self, prv_key: &TypedPrivateKey) -> Result<Vec<u8>> {
        let encoded_tx = self.encode_to_rlp(&SignatureData::new(self.chain_id))?;
        let sig_data = sign_message(&encoded_tx, prv_key)?;
        self.encode_to_rlp(&sig_data.to_eip155(self.chain_id))
    }
}

impl TransactionSigner<EthTxInput, EthTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &EthTxInput,
    ) -> Result<EthTxOutput> {
        let pk = self.find_private_key(symbol, address)?;
        let signed_tx = tx.sign_tx_by_pk(&pk)?;
        let mut tx_hash = tiny_keccak::keccak256(&signed_tx);
        Ok(EthTxOutput {
            signature: hex::encode(signed_tx),
            tx_hash: hex::encode(tx_hash),
        })
    }
}

fn ec_sign(data: &str, prv_key: &TypedPrivateKey) -> Result<Vec<u8>> {
    let data_bytes = data_to_bytes(data);
    let sig = sign_message(&data_bytes, prv_key)?;
    Ok(sig.to_vec())
}

fn personal_sign(data: &str, prv_key: &TypedPrivateKey) -> Result<Vec<u8>> {
    let data_bytes = data_to_bytes(data);
    let msg_len = data_bytes.len();
    let header_msg = format!("\u{0019}Ethereum Signed Message:\n{}", msg_len);
    let header_msg_bytes = header_msg.as_bytes();
    let data_to_sign = [header_msg_bytes, data_bytes.as_slice()].concat();
    let sig = sign_message(&data_to_sign, prv_key)?;
    Ok(sig.to_vec())
}

fn sign_message(msg: &[u8], prv_key: &TypedPrivateKey) -> Result<SignatureData> {
    let mut hashed_tx = tiny_keccak::keccak256(&msg);
    let signed_tx = prv_key.sign(&hashed_tx)?;
    // todo: sign should return SignatureData struct
    let sig = Signature::parse_der(&signed_tx)?;
    let mut sig_data = SignatureData::from_raw(&sig.serialize())?;
    let recover_id = calc_recover_id(&sig_data, &hashed_tx, &prv_key)?;
    sig_data.v = recover_id;
    Ok(sig_data)
}

#[cfg(test)]
mod tests {
    use crate::address::EthAddress;
    use crate::singer::{ec_sign, personal_sign, SignatureData};
    use crate::transaction::EthTxInput;
    use tcx_chain::Address;
    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::{
        PrivateKey, PublicKey, Secp256k1PrivateKey, Secp256k1PublicKey, TypedPrivateKey,
        TypedPublicKey,
    };

    fn create_eth_tx_input() -> EthTxInput {
        EthTxInput {
            nonce: "9".to_string(),
            gas_price: "20000000000".to_string(),
            gas_limit: "21000".to_string(),
            to: "3535353535353535353535353535353535353535".to_string(),
            value: "1000000000000000000".to_string(),
            data: "".to_string(),
            chain_id: 1,
        }
    }

    #[test]
    fn test_rlp_encode() {
        let tx = create_eth_tx_input();
        let sig = SignatureData {
            v: 1,
            r: vec![],
            s: vec![],
        };
        let ret = tx.encode_to_rlp(&sig).unwrap();
        assert_eq!("ec098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a764000080018080", hex::encode(ret));
    }

    #[test]
    fn test_simple_rlp_encode() {
        assert_eq!(vec![0x80], rlp::encode(&""));
        assert_eq!(hex::decode("83646f67").unwrap(), rlp::encode(&"dog"));
        assert_eq!(hex::decode("b8384c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e7365637465747572206164697069736963696e6720656c6974").unwrap(), rlp::encode(&"Lorem ipsum dolor sit amet, consectetur adipisicing elit"));
    }

    #[test]
    fn test_eip155_tx() {}

    fn eth_coin() -> CoinInfo {
        CoinInfo {
            coin: "ETH".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        }
    }

    #[test]
    fn test_sign_tx() {
        let pk = Secp256k1PrivateKey::from_slice(
            &hex::decode("4646464646464646464646464646464646464646464646464646464646464646")
                .unwrap(),
        )
        .unwrap();
        let typed_key = TypedPrivateKey::Secp256k1(pk);
        let signed_tx = create_eth_tx_input().sign_tx_by_pk(&typed_key).unwrap();
        assert_eq!("f86c098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a76400008025a028ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276a067cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83",
                   hex::encode(signed_tx))
    }

    #[test]
    fn test_etc_tx() {
        let pk = Secp256k1PrivateKey::from_slice(
            &hex::decode("4646464646464646464646464646464646464646464646464646464646464646")
                .unwrap(),
        )
        .unwrap();
        let typed_key = TypedPrivateKey::Secp256k1(pk);
        let mut etc_tx = create_eth_tx_input();
        etc_tx.chain_id = 61;
        let signed_tx = etc_tx.sign_tx_by_pk(&typed_key).unwrap();
        assert_eq!("f86d098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a764000080819da09e59aa73a10ec8fe5a97fe7560806315624c1a67aeeb59310fdc0001ba2b38a0a0719b723ff1b40c21c4235cbbbdaac0bf775be8f479c31caea806710f70f98927",
                   hex::encode(signed_tx))
    }

    #[test]
    fn test_personal_sign() {
        let data = "Hello imToken";
        let pk = Secp256k1PrivateKey::from_slice(
            &hex::decode("a392604efc2fad9c0b3da43b5f698a2e3f270f170d859912be0d54742275c5f6")
                .unwrap(),
        )
        .unwrap();
        let typed_key = TypedPrivateKey::Secp256k1(pk);
        let ret = personal_sign(data, &typed_key).unwrap();
        let test_cases = vec![
            ("Hello imToken", "1be38ff0ab0e6d97cba73cf61421f0641628be8ee91dcb2f73315e7fdf4d0e2770b0cb3cc7350426798d43f0fb05602664a28bb2c9fcf46a07fa1c8c4e322ec01b"),
            ("ef678007d18427e6022059dbc264f27507cd1ffc", "b12a1c9d3a7bb722d952366b06bd48cb35bdf69065dee92351504c3716a782493c697de7b5e59579bdcc624aa277f8be5e7f42dc65fe7fcd4cc68fef29ff28c21b")
        ];
        for case in test_cases {
            assert_eq!(
                case.1,
                hex::encode(personal_sign(case.0, &typed_key).unwrap())
            );
        }
    }

    #[test]
    fn test_ec_sign() {
        let pk = Secp256k1PrivateKey::from_slice(
            &hex::decode("3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1")
                .unwrap(),
        )
        .unwrap();
        let typed_key = TypedPrivateKey::Secp256k1(pk);
        let signature = ec_sign("imToken", &typed_key).unwrap();
        assert_eq!("7cf775589643e8b4f68f8aa3f5fe9b6b0d847612c1e1cd23af357a1bb8bfe930186444298e6126cc6eabd60a6e5bfb295b35556c8dfb9c1c614d198b91a299471b", hex::encode(signature));
    }
}
