use tcx_chain::{TransactionSigner, Keystore, Result};
use failure::Fail;

use crate::transaction::{TxOutput, TxInput, CachedCell, Witness};
use std::collections::HashMap;
use crate::Error;


pub struct CkbTxSigner {

}

impl CkbTxSigner {
    pub fn sign_witnesses(&mut self, tx_hash: &[u8], witnesses: &Vec<Witness>, input_cells: &Vec<CachedCell>) -> Result<Vec<Witness>> {
        // tx_hash must be 256 bit length
        if tx_hash.len() != 32 {
            return Err(Error::InvalidTxHash.into());
        }

        if witnesses.len() == 0 {
            return Err(Error::WitnessEmpty.into());
        }

        let grouped_scripts = self.group_script(input_cells);
        let mut raw_witnesses = witnesses.clone();
        let rest_witness = witnesses.last()?;

        grouped_scripts.iter().for_each(|item| {
            let mut ws = vec![];
            ws.iter_mut().chain(item.1.iter().map(|i| input_cells[i]).collect()).chain(rest_witness.clone());

            let signed_witness = self.sign_witness_group(tx_hash,&ws)?;
            raw_witnesses[item.1[0]] = signed_witness;
        });

        Ok(raw_witnesses)
    }

    pub fn sign_witness_group(&mut self, tx_hash: &[u8], witness_group: &Vec<Witness>) -> Result<Witness> {
        if witness_group.len() == 0 {
            return Err(Error::WitnessGroupEmpty.into());
        }



        Ok(())
    }

    pub fn group_script(&mut self, input_cells: &Vec<CachedCell>) -> HashMap<Vec<u8>, Vec<u32>> {
        let mut map:HashMap<Vec<u8>, Vec<u32>> = HashMap::new();

        for i in 0..input_cells.len() {
            let item = input_cells[i];
            let hash = item.lock.to_hash();
            let mut indices = map.get_mut(&hash);
            if indices.is_some() {
                indices.unwrap().push(i as u32);
            } else {
                map.insert(hash, vec![i as u32]);
            }
        }

        map
    }
}


impl TransactionSigner<TxInput, TxOutput> for Keystore  {
    fn sign_transaction(&mut self, symbol: &str, address: &str, tx: &TxInput) -> Result<TxOutput> {
        if tx.witnesses.len() == 0 {
            return Err(Error::RequiredWitness.into());
        }

        if tx.outputs_data.len() == 0 {
            return Err(Error::RequiredOutputsData.into());
        }

        if tx.outputs_data.len() < tx.outputs.len() {
            return Err(Error::InvalidOutputsDataLength.into());
        }

        let input_cells = tx.inputs.iter().map(|x| {
            if x.previous_output.is_none() {
                return Err(Error::InvalidOutputPoint.into());
            }

            let out_point = &x.previous_output.unwrap();

            let cached_cell = tx.cached_cells.iter().filter(
                |y| {
                    if y.out_point.is_none() {
                        return Err(Error::InvalidOutputPoint).into());
                    }

                    let yout_point = &y.out_point.unwrap();
                    out_point.tx_hash == yout_point.tx_hash && out_point.index == yout_point.index
                }).first()
        });


        let tx_output = TxOutput {
            tx_hash: vec![],
            witnesses: vec![],
        };

        Ok(tx_output)
    }
}

