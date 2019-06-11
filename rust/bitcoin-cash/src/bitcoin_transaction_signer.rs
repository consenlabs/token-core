use tcx_chain::{TxSignResult, TransactionSinger, Keystore, Metadata, Source};
use secp256k1::{Secp256k1, Message};
use bitcoin_hashes::Hash;
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin_hashes::hex::FromHex;
use bitcoin::{Address, PrivateKey, TxOut, TxIn, OutPoint, Script, Transaction};
use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ExtendedPrivKey, ChildNumber};
use crate::bip143_with_forkid::SighashComponentsWithForkId;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use std::str::FromStr;

const DUST: u64 = 546;

struct Utxo {
    tx_hash: String,
    vout: i32,
    amount: i64,
    address: String,
    script_pub_key: String,
    derived_path: String,
    sequence: i64
}

struct BitcoinTransaction  {
    pub to: String,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub memo: String,
    pub fee: i64,
    pub change_idx: u32,

    change_address: Address,
    network: Network,
    prv_keys: Vec<String>
}

impl BitcoinTransaction {

    fn collect_prv_keys_and_address(&self, password: &str, wallet: &Keystore) -> (Address, Vec<PrivateKey>) {
        let metadata = wallet.get_metadata();
//        let network = match metadata.network.to_uppercase().as_str() {
//            "MAINNET" => Network::Bitcoin,
//            _ => Network::Testnet
//        };

        match metadata.source {
            Source::Wif => {
                let change_addr = Address::from_str(&wallet.get_address()).unwrap();
                let wif = wallet.decrypt_cipher_text(password);
                let prv_key = PrivateKey::from_wif(&wif).unwrap();
                (change_addr, vec![prv_key])

            },
            _ => {
                let xprv = wallet.decrypt_cipher_text(password);
                let xprv_key = ExtendedPrivKey::from_str(&xprv).unwrap();
                let s = Secp256k1::new();
                let change_key = xprv_key.ckd_priv(&s, ChildNumber::from(0)).unwrap();
                let index_key = change_key.ckd_priv(&s, ChildNumber::from(self.change_idx)).unwrap();
                let index_pub_key = index_key.private_key.public_key(&s);
                let change_addr = Address::p2pkh(&index_pub_key, Network::Bitcoin);

                let mut prv_keys :Vec<PrivateKey> = vec![];
                for unspent in &self.unspents {
                    let derived_path = unspent.derived_path.trim();
                    let path_with_space = derived_path.replace("/", " ");
                    let path_idxs: Vec<&str> = path_with_space.split(" ").collect();
//                    let account_idx = path_idxs[0].parse::<u32>().unwrap();
//                    let index_idx = path_idxs[1].parse::<u32>().unwrap();

                    let account_key = xprv_key.ckd_priv(&s, ChildNumber::from_str(&path_idxs[0]).unwrap()).unwrap();
                    let unspent_index_key = account_key.ckd_priv(&s, ChildNumber::from_str(&path_idxs[1]).unwrap()).unwrap();
                    prv_keys.push(unspent_index_key.private_key);
                }
                (change_addr, prv_keys)
            }

        }
    }

    fn sign_hash(&self, pri_key: &PrivateKey, hash: &[u8]) -> Script {
        let s = Secp256k1::new();
        let msg = Message::from_slice(hash).unwrap();
        let signature = s.sign(&msg, &pri_key.key);
        let signature_bytes = signature.serialize_der();
        let raw_bytes: Vec<u8> = vec![0x41];
        let sig_bytes: Vec<u8> = [signature_bytes, raw_bytes.to_vec()].concat();
        let pub_key_bytes = pri_key.public_key(&s).to_bytes();
        Builder::new().push_slice(&sig_bytes).push_slice(&pub_key_bytes).into_script()
    }
}

impl TransactionSinger for BitcoinTransaction {
    fn sign_transaction(&self, chain_id: &str, password: &str, wallet: &Keystore) -> TxSignResult {
        let mut total_amount = 0;

        for unspent in &self.unspents {
            total_amount += unspent.amount;
        }
        if total_amount < self.amount {
            // todo: throw error;
        }


        let (change_addr, prv_keys) = self.collect_prv_keys_and_address(password, wallet);

        let mut tx_outs: Vec<TxOut> = vec![];
        let receiver_tx_out = TxOut {
            value: self.amount as u64,
            script_pubkey: change_addr.script_pubkey()
        };
        tx_outs.push(receiver_tx_out);
        let change_amount = (total_amount - self.amount);
        if change_amount > DUST as i64 {
            let change_tx_out = TxOut {
                value: change_amount as u64,
                script_pubkey: change_addr.script_pubkey()
            };
            tx_outs.push(change_tx_out);
        }

        let mut tx_inputs: Vec<TxIn> = vec![];

        for unspent in &self.unspents {
            tx_inputs.push(TxIn {
                previous_output: OutPoint {
                    txid: Hash256::from_hex(&unspent.tx_hash).unwrap(),
                    vout: unspent.vout as u32,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF,
                witness: vec![]
            });
        }


        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: tx_inputs,
            output: tx_outs
        };

        let sig_hash_components = SighashComponentsWithForkId::new(&tx);

        let mut script_sigs: Vec<Script> = vec![];
        for i in 0..tx.input.len() {
            let tx_in = &tx.input[i];
            let unspent = &self.unspents[i];
            let script_bytes = unspent.script_pub_key.as_bytes().to_vec();
            let script = Builder::from(script_bytes).into_script();
            let shc_hash = sig_hash_components.sighash_all(tx_in, &script, unspent.amount as u64, 0x01|0x40);
            let prv_key = prv_keys[i];
            script_sigs.push(self.sign_hash(&prv_key, &shc_hash.into_inner()));
        }

        let signed_tx = Transaction {
            version: tx.version,
            lock_time: tx.lock_time,
            input: tx.input.iter().enumerate().map(|(i, txin)| TxIn { script_sig: script_sigs[i].clone(), witness: vec![], .. *txin }).collect(),
            output: tx.output.clone(),
        };


        let tx_bytes = serialize(&signed_tx);

        TxSignResult {
            signature: tx_bytes.to_hex(),
            tx_hash: signed_tx.txid().into_inner().to_hex(),
            wtx_id: "".to_string()
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Language;
    use crate::hd_mnemonic_keystore::HdMnemonicKeystore;

    static PASSWORD: &'static str = "Insecure Pa55w0rd";
    static MNEMONIC: &'static str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";
    static BCH_MAIN_PATH: &'static str = "m/44'/145'/0'";


    #[test]
    pub fn bch_signer() {
        let meta = Metadata::default();

        let keystore = HdMnemonicKeystore::new(meta, &PASSWORD, &MNEMONIC, &BCH_MAIN_PATH).unwrap();

        println!("{:?}", keystore.unwrap().export_json());
//        assert!((&keystore.is_ok()))
//        assert!(keystore.is_ok());
//
////        let keystore = keystore.unwrap();
//        assert_eq!("16Hp1Ga779iaTe1TxUFDEBqNCGvfh3EHDZ", keystore.unwrap().address);
    }
}