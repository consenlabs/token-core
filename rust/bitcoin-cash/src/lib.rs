
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use secp256k1::{Secp256k1, Message};

use bitcoin::{PrivateKey, TxIn, OutPoint, Script, PublicKey, TxOut, Transaction};
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::hex::FromHex;
use std::str::FromStr;
use bitcoin_hashes::Hash;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language};

pub mod errors;
pub mod bip143_with_forkid;
pub mod hd_mnemonic_keystore;
pub mod bitcoin_cash_transaction_signer;
pub mod hard_wallet_keystore;
use bip143_with_forkid::SighashComponentsWithForkId;
use core::result;

#[macro_use] extern crate failure;
#[macro_use] extern crate hex_literal;
extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;

pub type Result<T> = result::Result<T, failure::Error>;

fn generate_address_from_wif(wif : &str) -> String {
    let s: Secp256k1<_> = Secp256k1::new();
    let prv_key = PrivateKey::from_wif(wif).unwrap();
    let pub_key = prv_key.public_key(&s);
    // Generate pay-to-pubkey-hash address
    let address = Address::p2pkh(&pub_key, Network::Bitcoin);
    println!("Script Pub Key {:?}", address.script_pubkey().to_hex());
    println!("{}", address.to_string());
    return address.to_string();
}

fn pub_key(wif: &str) -> PublicKey {
    let s = Secp256k1::new();
    let prv_key = PrivateKey::from_wif(wif).unwrap();
    let pub_key = prv_key.public_key(&s);
    return pub_key;
}

fn generate_transaction() -> String {
    let s = Secp256k1::new();
    let pri_key = PrivateKey::from_wif("L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy").unwrap();
    let pub_key = pri_key.public_key(&s);
    println!("Address {:?}", Address::p2pkh(&pub_key, Network::Bitcoin).to_string());
    let pub_key_script = Address::p2pkh(&pub_key, Network::Bitcoin).script_pubkey();
    println!("pub key script {:?}", pub_key_script.to_hex());
//    let pub_key_script_manual = Builder::new().push_key(&pub_key).into_script();
//    assert_eq!(pub_key_script, pub_key_script_manual);

    let tx_in = TxIn {
        previous_output: OutPoint {
            txid: Hash256::from_hex("115e8f72f39fad874cfab0deed11a80f24f967a84079fb56ddf53ea02e308986").unwrap(),
            vout: 0,
        },
        script_sig: Script::new(),
        sequence: 0xFFFFFFFF,
        witness: vec![]

    };

    let address = Address::from_str("1Gokm82v6DmtwKEB8AiVhm82hyFSsEvBDK").unwrap();
//    let key_hash = address.payload.
    let tx_out = TxOut {
        value: 15000,
        script_pubkey: address.script_pubkey()
    };
    let mut tx = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![tx_in],
        output: vec![tx_out]
    };
//    l
    let sig_hash_components = SighashComponentsWithForkId::new(&tx);
//    println!("sign_hash_components: {:?}", serialize_hex(&sig_hash_components));
//    println!("hash prevouts {:?}\n, hash sequence: {:?}\n, hash outputs: {:?}", sig_hash_components.hash_prevouts, sig_hash_components.hash_sequence, sig_hash_components.hash_outputs);
    let shc_hash = sig_hash_components.sighash_all(&tx.input[0], &pub_key_script, 50000, 0x01 | 0x40);
//    println!("pub_key_script: {:?}", pub_key_script.to_bytes().to_hex());
//    println!("lock time: {:?}", ))
    let shc_hash = Hash256::from_slice(&shc_hash.into_inner()).unwrap();
    println!("SegWit hash {:?}", shc_hash.into_inner().to_hex());

//    println!("before hash {:?}", tx_bytes.to_hex());


//    let hash = tx.signature_hash(0, &pub_key_script, SigHashType::All.as_u32());
//    println!("hash: {:?}", hash.into_inner().to_hex());

    let msg = Message::from_slice(&shc_hash.into_inner()).unwrap();
    let signature = s.sign(&msg, &pri_key.key);
    let signature_bytes = signature.serialize_der();
    let raw_bytes: Vec<u8> = vec![0x41];
    let sig_bytes: Vec<u8> = [signature_bytes, raw_bytes.to_vec()].concat();
    let pub_key_bytes = pri_key.public_key(&s).to_bytes();
    let sig_script = Builder::new().push_slice(&sig_bytes).push_slice(&pub_key_bytes).into_script();

    let new_tx_in = TxIn {
        previous_output: OutPoint {
            txid: Hash256::from_hex("115e8f72f39fad874cfab0deed11a80f24f967a84079fb56ddf53ea02e308986").unwrap(),
            vout: 0,
        },
        script_sig: sig_script,
        sequence: 0xFFFFFFFF,
        witness: vec![]

    };

    tx.input = vec![new_tx_in];
    let tx_bytes = serialize(&tx);
    println!("{:?}", tx_bytes.to_hex());
    return tx_bytes.to_hex();
}

fn generate_xpub(mnemonic_str: &str, path: &str) -> String {
    if let Ok(mnemonic) = Mnemonic::from_phrase(mnemonic_str, Language::English) {
        let seed = bip39::Seed::new(&mnemonic, &"");
        println!("hex: {}", seed.to_hex());
        let s = Secp256k1::new();

        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes()).unwrap();
        let path = DerivationPath::from_str(path).unwrap();

        let btc_derived= sk.derive_priv(&s, &path).unwrap();
        let extended_pub_key = ExtendedPubKey::from_private(&s, &btc_derived);
        return extended_pub_key.to_string();
    }  else {
        return String::new();
    }
}
#[cfg(test)]
mod tests {

    use bitcoin::PrivateKey;
    use secp256k1::Secp256k1;
//    use cash_addr::{encode, decode, AddressType};
    use crate::{generate_address_from_wif, generate_transaction, generate_xpub};
    use bch_addr::Converter;


    #[test]
    fn bch_address() {

        let address = generate_address_from_wif("L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy");
        let converter = Converter::new();
        let cash_addr = converter.to_cash_addr("1Gokm82v6DmtwKEB8AiVhm82hyFSsEvBDK").unwrap();
                assert_eq!("bitcoincash:qrnvl24e5kd6rpls53wmpvtfcgdmfrcfkvrmn5zj3l", cash_addr);

    }

    #[test]
    fn tx() {
        let tx_hex = generate_transaction();
        assert_eq!("01000000018689302ea03ef5dd56fb7940a867f9240fa811eddeb0fa4c87ad9ff3728f5e11000000006b483045022100bc4295d369443e2cc4e20b50a6fd8e7e16c08aabdbb42bdf167dec9d41afc3d402207a8e0ccb91438785e51203e7d2f85c4698ff81245936ebb71935e3d052876dcd4121029f50f51d63b345039a290c94bffd3180c99ed659ff6ea6b1242bca47eb93b59fffffffff01983a0000000000001976a914ad618cf4333b3b248f9744e8e81db2964d0ae39788ac00000000", tx_hex);
    }


}
