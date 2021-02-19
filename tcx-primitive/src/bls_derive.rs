use bip39::{Language, Mnemonic};
use num_bigint::BigUint;

use super::Result;
use crate::bls::{BLSPrivateKey, BLSPublicKey};
use crate::ecc::KeyError;
use crate::{Derive, DeterministicPrivateKey, DeterministicPublicKey, FromHex, PrivateKey, ToHex};
use num_traits::{FromPrimitive, Num, Pow};
use ring::digest;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct BLSDeterministicPrivateKey(pub BigUint);

#[derive(Clone)]
pub struct BLSDeterministicPublicKey();

impl Derive for BLSDeterministicPrivateKey {
    fn derive(&self, path: &str) -> Result<Self> {
        let mut parts = path.split('/').peekable();
        if *parts.peek().unwrap() == "m" {
            parts.next();
        }

        let result = parts
            .map(str::parse)
            .collect::<std::result::Result<Vec<BigUint>, _>>();
        if result.is_err() {
            return Err(KeyError::InvalidDerivationPathFormat.into());
        }

        let children_nums = result.unwrap();

        let mut children_key = self.0.clone();
        for index in children_nums {
            children_key = derive_child(children_key, index);
        }

        Ok(BLSDeterministicPrivateKey(children_key))
    }
}

impl DeterministicPrivateKey for BLSDeterministicPrivateKey {
    type DeterministicPublicKey = BLSDeterministicPublicKey;
    type PrivateKey = BLSPrivateKey;

    fn from_seed(seed: &[u8]) -> Result<Self> {
        let master_sk = derive_master_sk(seed);
        if master_sk.is_err() {
            return Err(failure::err_msg("invalid seed"));
        }

        Ok(BLSDeterministicPrivateKey(master_sk.unwrap()))
    }

    fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        let mn = Mnemonic::from_phrase(mnemonic, Language::English)?;
        let seed = bip39::Seed::new(&mn, "");
        BLSDeterministicPrivateKey::from_seed(seed.as_bytes())
    }

    fn private_key(&self) -> Self::PrivateKey {
        BLSPrivateKey::from_slice(&self.0.to_bytes_le()).unwrap()
    }

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey {
        panic!("not supported")
    }
}

impl Derive for BLSDeterministicPublicKey {}

impl FromHex for BLSDeterministicPublicKey {
    fn from_hex(_: &str) -> Result<Self> {
        panic!("not supported")
    }
}

impl ToHex for BLSDeterministicPublicKey {
    fn to_hex(&self) -> String {
        panic!("not supported")
    }
}

impl DeterministicPublicKey for BLSDeterministicPublicKey {
    type PublicKey = BLSPublicKey;

    fn public_key(&self) -> Self::PublicKey {
        panic!("not supported")
    }
}

// copy from https://github.com/ChainSafe/rust-bls-derivation/blob/master/src/key_derivation.rs
// and follow the latest EIP-2333
const DIGEST_SIZE: usize = 32;
const NUM_DIGESTS: usize = 255;
const OUTPUT_SIZE: usize = DIGEST_SIZE * NUM_DIGESTS;

fn hkdf(salt: &[u8], ikm: &[u8], info: &[u8], okm: &mut [u8]) {
    let mut extractor = hkdf::HkdfExtract::<Sha256>::new(Some(salt));
    extractor.input_ikm(ikm);

    let (prk, _) = extractor.finalize();
    let expander = hkdf::Hkdf::<Sha256>::from_prk(&prk).unwrap();
    expander.expand(info, okm).expect("invalid_hkdf");
}

fn flip_bits(num: BigUint) -> BigUint {
    num ^ (Pow::pow(
        &BigUint::from_u64(2).unwrap(),
        &BigUint::from_u64(256).unwrap(),
    ) - &BigUint::from_u64(1).unwrap())
}

fn ikm_to_lamport_sk(ikm: &[u8], salt: &[u8], split_bytes: &mut [[u8; DIGEST_SIZE]; NUM_DIGESTS]) {
    let mut okm = [0u8; OUTPUT_SIZE];
    hkdf(salt, ikm, b"", &mut okm);
    for r in 0..NUM_DIGESTS {
        split_bytes[r].copy_from_slice(&okm[r * DIGEST_SIZE..(r + 1) * DIGEST_SIZE])
    }
}

fn parent_sk_to_lamport_pk(parent_sk: BigUint, index: BigUint) -> Vec<u8> {
    let mut salt = index.to_bytes_be();
    while salt.len() < 4 {
        salt.insert(0, 0x00);
    }
    let ikm = parent_sk.to_bytes_be();
    let mut lamport_0 = [[0u8; DIGEST_SIZE]; NUM_DIGESTS];
    ikm_to_lamport_sk(ikm.as_slice(), salt.as_slice(), &mut lamport_0);

    let not_ikm = flip_bits(parent_sk).to_bytes_be();
    let mut lamport_1 = [[0u8; DIGEST_SIZE]; NUM_DIGESTS];
    ikm_to_lamport_sk(not_ikm.as_slice(), salt.as_slice(), &mut lamport_1);

    let mut combined = [[0u8; DIGEST_SIZE]; NUM_DIGESTS * 2];
    combined[..NUM_DIGESTS].clone_from_slice(&lamport_0[..NUM_DIGESTS]);
    combined[NUM_DIGESTS..NUM_DIGESTS * 2].clone_from_slice(&lamport_1[..NUM_DIGESTS]);

    let mut flattened_key = [0u8; OUTPUT_SIZE * 2];
    for i in 0..NUM_DIGESTS * 2 {
        let mut sha256 = Sha256::new();
        let need_to_hash = &mut combined[i];
        sha256.update(&need_to_hash);
        let hash_ret = &sha256.finalize_fixed();
        flattened_key[i * DIGEST_SIZE..(i + 1) * DIGEST_SIZE].clone_from_slice(&hash_ret);
    }

    let mut sha256 = Sha256::new();
    for i in 0..NUM_DIGESTS * 2 {
        sha256.update(&flattened_key[i * DIGEST_SIZE..(i + 1) * DIGEST_SIZE])
    }
    sha256.finalize_fixed().to_vec()
}

fn hkdf_mod_r(ikm: &[u8]) -> BigUint {
    let mut okm = [0u8; 48];

    let mut tmp = ikm.to_vec();
    tmp.extend(b"\x00");

    let digest_obj = digest::digest(&digest::SHA256, b"BLS-SIG-KEYGEN-SALT-");
    hkdf(digest_obj.as_ref(), &tmp, b"\x00\x30", &mut okm); // L=48, info=I2OSP(L,2)
    let r = BigUint::from_str_radix(
        "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001",
        16,
    )
    .unwrap();

    BigUint::from_bytes_be(okm.as_ref()) % r
}

pub fn derive_child(parent_sk: BigUint, index: BigUint) -> BigUint {
    let lamp_pk = parent_sk_to_lamport_pk(parent_sk, index);
    hkdf_mod_r(lamp_pk.as_ref())
}

pub fn derive_master_sk(seed: &[u8]) -> Result<BigUint> {
    if seed.len() < 16 {
        return Err(failure::err_msg(
            "seed must be greater than or equal to 16 bytes",
        ));
    }

    Ok(hkdf_mod_r(seed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bls_derive::BLSDeterministicPrivateKey;
    use crate::{Derive, DeterministicPrivateKey, PrivateKey};
    use hex;
    use num_bigint::BigUint;

    struct TestVector {
        seed: &'static str,
        master_sk: &'static str,
        child_index: &'static str,
        child_sk: &'static str,
    }

    #[test]
    fn test_2333() {
        let test_vectors = vec!(
            TestVector{
                seed : "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04",
                master_sk : "6083874454709270928345386274498605044986640685124978867557563392430687146096",
                child_index : "0",
                child_sk : "20397789859736650942317412262472558107875392172444076792671091975210932703118",
            },
            TestVector{
                seed: "3141592653589793238462643383279502884197169399375105820974944592",
                master_sk: "29757020647961307431480504535336562678282505419141012933316116377660817309383",
                child_index: "3141592653",
                child_sk: "25457201688850691947727629385191704516744796114925897962676248250929345014287",
            },
            TestVector{
                seed: "0099FF991111002299DD7744EE3355BBDD8844115566CC55663355668888CC00",
                master_sk: "27580842291869792442942448775674722299803720648445448686099262467207037398656",
                child_index: "4294967295",
                child_sk: "29358610794459428860402234341874281240803786294062035874021252734817515685787",
            }
        );

        for t in test_vectors.iter() {
            let seed = hex::decode(t.seed).expect("invalid seed format");
            let master_sk = t
                .master_sk
                .parse::<BigUint>()
                .expect("invalid master key format");
            let child_index = t
                .child_index
                .parse::<BigUint>()
                .expect("invalid index format");
            let child_sk = t
                .child_sk
                .parse::<BigUint>()
                .expect("invalid child key format");

            let derived_master_sk = derive_master_sk(seed.as_ref()).unwrap();
            assert_eq!(
                derived_master_sk, master_sk,
                "{}",
                "derived_master_sk == master_sk"
            );
            let pk = derive_child(master_sk, child_index);
            assert_eq!(child_sk, pk);
        }
    }

    #[test]
    fn test_bls_derive() {
        let dsk = BLSDeterministicPrivateKey::from_seed(
            &hex::decode("c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04").unwrap()).unwrap();

        assert_eq!(
            hex::encode(dsk.private_key().to_bytes()),
            "7050b4223168ae407dee804d461fc3dbfe53f5dc5218debb8fab6379d559730d"
        );

        assert_eq!(
            hex::encode(dsk.derive("m/0").unwrap().private_key().to_bytes()),
            "8e0fe539158c9d590a771420cc033baedaf3749b5c08b5f85bd1e6146cbd182d"
        );
    }

    #[test]
    fn eth2_withdrawal_address_test() {
        let dsk = BLSDeterministicPrivateKey::from_seed(
            &hex::decode("ee3fce3ccf05a2b58c851e321077a63ee2113235112a16fc783dc16279ff818a549ff735ac4406c624235db2d37108e34c6cbe853cbe09eb9e2369e6dd1c5aaa").unwrap()).unwrap();
        assert_eq!(
            dsk.0,
            "18563599344197674528480235454076968403807977642577320252460493386276600523197"
                .parse::<BigUint>()
                .expect("invalid master key format")
        );

        let child_sk = hex::encode(
            dsk.derive("m/12381/3600/1/0/0")
                .unwrap()
                .private_key()
                .to_bytes(),
        );
        assert_eq!(
            child_sk,
            "ba87c3a478ee2a5a26c48918cc99be88bc648bee3d38c2d5faad41872a9e0d06"
        );
    }
}
