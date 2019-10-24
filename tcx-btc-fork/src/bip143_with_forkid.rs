use bitcoin_hashes::{sha256d, Hash};

use bitcoin::blockdata::script::Script;
use bitcoin::blockdata::transaction::{Transaction, TxIn};
use bitcoin::consensus::encode::Encodable;
use std::io::Cursor;

/// Parts of a sighash which are common across inputs or signatures, and which are
/// sufficient (in conjunction with a private key) to sign the transaction
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SighashComponentsWithForkId {
    tx_version: u32,
    tx_locktime: u32,
    /// Hash of all the previous outputs
    pub hash_prevouts: sha256d::Hash,
    /// Hash of all the input sequence nos
    pub hash_sequence: sha256d::Hash,
    /// Hash of all the outputs in this transaction
    pub hash_outputs: sha256d::Hash,
}

impl SighashComponentsWithForkId {
    /// Compute the sighash components from an unsigned transaction and auxiliary
    /// information about its inputs.
    /// For the generated sighashes to be valid, no fields in the transaction may change except for
    /// script_sig and witnesses.
    pub fn new(tx: &Transaction) -> SighashComponentsWithForkId {
        let hash_prevouts = {
            let mut enc = sha256d::Hash::engine();
            for txin in &tx.input {
                txin.previous_output.consensus_encode(&mut enc).unwrap();
            }
            sha256d::Hash::from_engine(enc)
        };

        let hash_sequence = {
            let mut enc = sha256d::Hash::engine();
            for txin in &tx.input {
                txin.sequence.consensus_encode(&mut enc).unwrap();
            }
            sha256d::Hash::from_engine(enc)
        };

        let hash_outputs = {
            let mut enc = sha256d::Hash::engine();
            for txout in &tx.output {
                txout.consensus_encode(&mut enc).unwrap();
            }
            sha256d::Hash::from_engine(enc)
        };

        SighashComponentsWithForkId {
            tx_version: tx.version,
            tx_locktime: tx.lock_time,
            hash_prevouts,
            hash_sequence,
            hash_outputs,
        }
    }

    /// Compute the BIP143 sighash for a `SIGHASH_ALL` signature for the given
    /// input.
    pub fn sighash_all(
        &self,
        txin: &TxIn,
        witness_script: &Script,
        value: u64,
        fork_id: u32,
    ) -> sha256d::Hash {
        let mut enc = sha256d::Hash::engine();
        let mut encoder: Cursor<Vec<u8>> = Cursor::new(vec![]);
        self.tx_version.consensus_encode(&mut enc).unwrap();
        self.tx_version.consensus_encode(&mut encoder).unwrap();

        self.hash_prevouts.consensus_encode(&mut enc).unwrap();
        self.hash_prevouts.consensus_encode(&mut encoder).unwrap();
        self.hash_sequence.consensus_encode(&mut enc).unwrap();
        self.hash_sequence.consensus_encode(&mut encoder).unwrap();
        txin.previous_output.consensus_encode(&mut enc).unwrap();
        txin.previous_output.consensus_encode(&mut encoder).unwrap();
        witness_script.consensus_encode(&mut enc).unwrap();
        witness_script.consensus_encode(&mut encoder).unwrap();
        value.consensus_encode(&mut enc).unwrap();
        value.consensus_encode(&mut encoder).unwrap();
        txin.sequence.consensus_encode(&mut enc).unwrap();
        txin.sequence.consensus_encode(&mut encoder).unwrap();
        self.hash_outputs.consensus_encode(&mut enc).unwrap();
        self.hash_outputs.consensus_encode(&mut encoder).unwrap();
        self.tx_locktime.consensus_encode(&mut enc).unwrap();
        self.tx_locktime.consensus_encode(&mut encoder).unwrap();
        fork_id.consensus_encode(&mut enc).unwrap(); // hashtype
        fork_id.consensus_encode(&mut encoder).unwrap(); // hashtype
        sha256d::Hash::hash(&encoder.into_inner())
    }

    #[warn(dead_code)]
    fn sign_hash_type_with_fork_id(sign_hash_type: u32) -> u32 {
        let fork_value = sign_hash_type >> 8;
        let new_fork_value = 0x00ff_0000 | (fork_value ^ 0xdead);
        (new_fork_value << 8) | (sign_hash_type & 0xff)
    }
}
