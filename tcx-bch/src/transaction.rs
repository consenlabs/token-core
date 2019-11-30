use crate::address::BchAddress;
use crate::Result;
use bitcoin::Transaction;
use bitcoin_hashes::sha256d;
use std::str::FromStr;
use tcx_btc_fork::bip143_with_forkid::SighashComponentsWithForkId;
use tcx_btc_fork::transaction::{LegacyTransactionSignComponent, SignHasher};
use tcx_btc_fork::PubKeyScript;
use tcx_btc_fork::{BitcoinForkTransaction, Utxo};

const BCH_FORK_ID: u32 = 0x41;

pub struct BchSignHasher {}

impl SignHasher for BchSignHasher {
    fn sign_hash(tx: &Transaction, index: usize, unspent: &Utxo) -> Result<(sha256d::Hash, u32)> {
        let addr = BchAddress::from_str(&unspent.address)?;
        let tx_in = &tx.input[index];
        let script = addr.script_pub_key();
        let shc = SighashComponentsWithForkId::new(&tx);
        let hash = shc.sighash_all(tx_in, &script, unspent.amount as u64, BCH_FORK_ID);
        Ok((hash, BCH_FORK_ID))
    }
}

pub type BchTransaction =
    BitcoinForkTransaction<BchAddress, LegacyTransactionSignComponent<BchSignHasher>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::BchAddress;

    use bch_addr::Converter;
    use std::marker::PhantomData;
    use std::str::FromStr;
    use tcx_primitive::{PrivateKey, Secp256k1PrivateKey};

    #[test]
    pub fn bch_signer() {
        let unspents = vec![Utxo {
            tx_hash: "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2".to_string(),
            vout: 0,
            amount: 100000,
            address: "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r".to_string(),
            script_pub_key: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac".to_string(),
            derived_path: "1/0".to_string(),
            sequence: 0,
        }];
        let tran =
            BitcoinForkTransaction::<BchAddress, LegacyTransactionSignComponent<BchSignHasher>> {
                to: "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s".to_string(),
                amount: 93454,
                unspents,
                memo: "".to_string(),
                fee: 6000,
                change_idx: Some(1u32),
                change_address: None,
                coin: "BITCOINCASH".to_string(),
                _marker_s: PhantomData,
                _marker_t: PhantomData,
            };
        //
        let prv_key = Secp256k1PrivateKey::from_slice(
            &hex::decode("b0dabbf9ffed224fbca3b41a9e446b3d0b6240c6d2957197a8ab75bbf2e1a5d4")
                .unwrap(),
        )
        .unwrap();

        let change_addr =
            BchAddress::from_str("qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r").unwrap();
        let expected = tran
            .sign_transaction(&vec![prv_key], change_addr.script_pub_key())
            .unwrap();
        assert_eq!(expected.signature, "0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006a473044022064fb81c11181e6604aa56b29ed65e31680fc1203f5afb6f67c5437f2d68192d9022022282d6c3c35ffdf64a427df5e134aa0edb8528efb6151cb1c3b21422fdfd6e041210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a914bedf37acf35504c9bfd18b09d989d0fb23fd269688ac00000000");
    }
}
