use failure::Fail;
use tcx_chain::{Keystore, Result, TransactionSigner};

use crate::hash::new_blake2b;
use crate::serializer::Serializer;
use crate::transaction::{CachedCell, CellInput, TxInput, TxOutput, Witness};
use crate::Error;
use std::collections::HashMap;
use tcx_chain::ChainSigner;

pub struct CkbTxSigner<'a> {
    ks: &'a mut Keystore,
    symbol: &'a str,
    address: &'a str,
}

impl<'a> CkbTxSigner<'a> {
    pub fn sign_witnesses(
        &mut self,
        tx_hash: &[u8],
        witnesses: &Vec<Witness>,
        input_cells: &Vec<CachedCell>,
    ) -> Result<Vec<Witness>> {
        // tx_hash must be 256 bit length
        if tx_hash.len() != 32 {
            return Err(Error::InvalidTxHash.into());
        }

        if witnesses.len() == 0 {
            return Err(Error::WitnessEmpty.into());
        }

        let grouped_scripts = self.group_script(input_cells)?;

        let mut raw_witnesses = witnesses.to_vec();

        for item in grouped_scripts.iter() {
            let mut ws = vec![];
            ws.extend(item.1.iter().map(|i| &witnesses[*i]));

            if witnesses.len() > input_cells.len() {
                ws.extend(&witnesses[input_cells.len()..]);
            }

            let signed_witness = self.sign_witness_group(tx_hash, &ws)?;
            raw_witnesses[item.1[0]] = signed_witness;
        }

        Ok(raw_witnesses)
    }

    pub fn sign_witness_group(
        &mut self,
        tx_hash: &[u8],
        witness_group: &Vec<&Witness>,
    ) -> Result<Witness> {
        if witness_group.len() == 0 {
            return Err(Error::WitnessGroupEmpty.into());
        }

        let first = &witness_group[0];

        let mut empty_witness = Witness {
            lock: [0u8; 130].to_vec(),
            input_type: first.input_type.clone(),
            output_type: first.output_type.clone(),
        };

        let serialized_empty_witness = empty_witness.serialize()?;
        let serialized_empty_length = serialized_empty_witness.len();

        let mut s = new_blake2b();
        s.update(tx_hash);
        s.update(&Serializer::serialize_u64(serialized_empty_length as u64));
        s.update(&serialized_empty_witness);

        for w in witness_group[1..].iter() {
            let bytes = w.serialize()?;
            s.update(&Serializer::serialize_u64(bytes.len() as u64));
            s.update(&bytes);
        }

        let mut result = [0u8; 32];
        s.finalize(&mut result);

        empty_witness.lock =
            self.ks
                .sign_recoverable_hash(&result, self.symbol, self.address, None)?;

        Ok(empty_witness)
    }

    fn group_script(
        &mut self,
        input_cells: &Vec<CachedCell>,
    ) -> Result<HashMap<Vec<u8>, Vec<usize>>> {
        let mut map: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();

        for i in 0..input_cells.len() {
            let item = &input_cells[i];
            if item.lock.is_none() {
                continue;
            }

            let hash = item.lock.as_ref().unwrap().to_hash()?;
            let indices = map.get_mut(&hash);
            if indices.is_some() {
                indices.unwrap().push(i);
            } else {
                map.insert(hash, vec![i]);
            }
        }

        Ok(map)
    }
}

impl TransactionSigner<TxInput, TxOutput> for Keystore {
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

        /*
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
        */

        let tx_output = TxOutput {
            tx_hash: vec![],
            witnesses: vec![],
        };

        Ok(tx_output)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn group_script() {}

    #[test]
    fn sign_transaction() {}

    #[test]
    fn sign_witnesses() {}

    #[test]
    fn sign_group_witness() {}
}
