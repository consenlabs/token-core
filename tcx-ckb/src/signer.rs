use tcx_chain::{Keystore, Result, TransactionSigner};

use crate::hash::new_blake2b;
use crate::serializer::Serializer;
use crate::transaction::{CachedCell, CkbTxInput, CkbTxOutput, OutPoint, Witness};
use crate::{hex_to_bytes, Error};
use std::collections::HashMap;
use tcx_chain::ChainSigner;

use lazy_static::lazy_static;

pub struct CkbTxSigner<'a> {
    ks: &'a mut dyn ChainSigner,
    symbol: &'a str,
    address: &'a str,
}

lazy_static! {
    pub static ref SIGNATURE_PLACEHOLDER: String = "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_owned();
}

impl<'a> CkbTxSigner<'a> {
    pub fn sign_witnesses(
        &mut self,
        tx_hash: &[u8],
        witnesses: &Vec<Witness>,
        input_cells: &Vec<&CachedCell>,
    ) -> Result<Vec<String>> {
        // tx_hash must be 256 bit length
        if tx_hash.len() != 32 {
            return Err(Error::InvalidTxHash.into());
        }

        if witnesses.len() == 0 {
            return Err(Error::WitnessEmpty.into());
        }

        let grouped_scripts = self.group_script(input_cells)?;

        let mut raw_witnesses: Vec<String> = vec![];
        for w in witnesses.iter() {
            raw_witnesses.push(format!("0x{}", hex::encode(w.to_raw()?)));
        }

        for item in grouped_scripts.iter() {
            let mut ws = vec![];
            ws.extend(item.1.iter().map(|i| &witnesses[*i]));

            if witnesses.len() > input_cells.len() {
                ws.extend(&witnesses[input_cells.len()..]);
            }

            let path = &input_cells[item.1[0]].derived_path;

            let signed_witness = self.sign_witness_group(tx_hash, &ws, path)?;
            raw_witnesses[item.1[0]] = format!("0x{}", hex::encode(signed_witness.serialize()?));
        }

        Ok(raw_witnesses)
    }

    pub fn sign_witness_group(
        &mut self,
        tx_hash: &[u8],
        witness_group: &Vec<&Witness>,
        path: &str,
    ) -> Result<Witness> {
        if witness_group.len() == 0 {
            return Err(Error::WitnessGroupEmpty.into());
        }

        let first = &witness_group[0];

        let mut empty_witness = Witness {
            lock: SIGNATURE_PLACEHOLDER.clone(),
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
            let bytes = w.to_raw()?;
            s.update(&Serializer::serialize_u64(bytes.len() as u64));
            s.update(&bytes);
        }

        let mut result = [0u8; 32];
        s.finalize(&mut result);

        let opt_path = if path.len() > 0 { Some(path) } else { None };

        empty_witness.lock = format!(
            "0x{}",
            hex::encode(self.ks.sign_recoverable_hash(
                &result,
                self.symbol,
                self.address,
                opt_path
            )?)
        );

        Ok(empty_witness)
    }

    fn group_script(
        &mut self,
        input_cells: &Vec<&CachedCell>,
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

impl TransactionSigner<CkbTxInput, CkbTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &CkbTxInput,
    ) -> Result<CkbTxOutput> {
        if tx.witnesses.len() == 0 {
            return Err(Error::RequiredWitness.into());
        }

        let find_cache_cell = |x: &OutPoint| -> Result<&CachedCell> {
            for y in tx.cached_cells.iter() {
                if y.out_point.is_some() {
                    let point = y.out_point.as_ref().unwrap();
                    if point.index == x.index && point.tx_hash == x.tx_hash {
                        return Ok(y);
                    }
                }
            }

            Err(Error::CellInputNotCached.into())
        };

        let mut input_cells: Vec<&CachedCell> = vec![];

        for x in tx.inputs.iter() {
            if x.previous_output.is_none() {
                return Err(Error::InvalidOutputPoint.into());
            }

            input_cells.push(find_cache_cell(x.previous_output.as_ref().unwrap())?);
        }

        if tx.witnesses.len() < input_cells.len() || input_cells.len() == 0 {
            return Err(Error::InvalidInputCells.into());
        }

        let mut signer = CkbTxSigner {
            ks: self,
            symbol,
            address,
        };

        let signed_witnesses =
            signer.sign_witnesses(&hex_to_bytes(&tx.tx_hash)?, &tx.witnesses, &input_cells)?;

        let tx_output = CkbTxOutput {
            tx_hash: tx.tx_hash.clone(),
            witnesses: signed_witnesses,
        };

        Ok(tx_output)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::CkbAddress;
    use crate::transaction::{CachedCell, CellInput, CkbTxInput, OutPoint, Script, Witness};
    use tcx_chain::{Keystore, Metadata, TransactionSigner};
    use tcx_constants::{CoinInfo, CurveType};

    #[test]
    fn test_sign_transaction() {
        let tx_hash = "0x719933ec055272734ab709a80492edb44c083e6b675e5c37e5bb3f720fe88e5e";

        let witnesses = vec![Witness::default(), Witness::default(), Witness::default()];

        let cached_cells = vec![
            CachedCell {
                out_point: Some({
                    OutPoint {
                        tx_hash:
                            "0x67b35360a09ecbdaf7cef55bb9b58b194d1e067007c67d67520ee730fcd1f252"
                                .to_owned(),
                        index: 0,
                    }
                }),
                lock: Some(Script {
                    args: "0xb1e8f5e7b4be7867ca6cd556ee3954a325979f45".to_owned(),
                    code_hash: "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
                        .to_owned(),
                    hash_type: "type".to_string(),
                }),
                ..CachedCell::default()
            },
            CachedCell {
                out_point: Some({
                    OutPoint {
                        tx_hash:
                            "0x67b35360a09ecbdaf7cef55bb9b58b194d1e067007c67d67520ee730fcd1f252"
                                .to_owned(),
                        index: 1,
                    }
                }),
                lock: Some(Script {
                    args: "0xb1e8f5e7b4be7867ca6cd556ee3954a325979f45".to_owned(),
                    code_hash: "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
                        .to_owned(),
                    hash_type: "type".to_string(),
                }),
                ..CachedCell::default()
            },
            CachedCell {
                out_point: Some({
                    OutPoint {
                        tx_hash:
                            "0x67b35360a09ecbdaf7cef55bb9b58b194d1e067007c67d67520ee730fcd1f252"
                                .to_owned(),
                        index: 2,
                    }
                }),
                lock: Some(Script {
                    args: "0xb1e8f5e7b4be7867ca6cd556ee3954a325979f45".to_owned(),
                    code_hash: "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
                        .to_owned(),
                    hash_type: "type".to_string(),
                }),
                ..CachedCell::default()
            },
        ];

        let inputs = vec![
            CellInput {
                previous_output: Some(OutPoint {
                    tx_hash: "0x67b35360a09ecbdaf7cef55bb9b58b194d1e067007c67d67520ee730fcd1f252"
                        .to_owned(),
                    index: 0,
                }),
                since: "".to_owned(),
            },
            CellInput {
                previous_output: Some(OutPoint {
                    tx_hash: "0x67b35360a09ecbdaf7cef55bb9b58b194d1e067007c67d67520ee730fcd1f252"
                        .to_owned(),
                    index: 1,
                }),
                since: "".to_string(),
            },
            CellInput {
                previous_output: Some(OutPoint {
                    tx_hash: "0x67b35360a09ecbdaf7cef55bb9b58b194d1e067007c67d67520ee730fcd1f252"
                        .to_owned(),
                    index: 2,
                }),
                since: "".to_string(),
            },
        ];

        let tx_input = CkbTxInput {
            inputs,
            witnesses,
            tx_hash: tx_hash.clone().to_owned(),
            cached_cells,
            ..CkbTxInput::default()
        };

        let coin_info = CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "m/44'/309'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        let mut ks = Keystore::from_mnemonic(
            "inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
            "Password",
            Metadata::default(),
        )
        .unwrap();

        ks.unlock_by_password("Password").unwrap();

        let account = ks.derive_coin::<CkbAddress>(&coin_info).unwrap().clone();

        assert_eq!(
            account.address,
            "ckt1qyqtr684u76tu7r8efkd24hw8922xfvhnazskzdzy6"
        );

        let tx_output = ks
            .sign_transaction("NERVOS", &account.address, &tx_input)
            .unwrap();
        assert_eq!(tx_output.witnesses[0], "0x55000000100000005500000055000000410000009b87828a6274850b4c8724a286b882aae3ace127c124e4f6687070c09e2533c80b33ace45005a4912f4d092e31f017a8dc9f2f97ef66fb5e2b5e9314ade9b60e00");
    }

    #[test]
    fn invalid_sign_transaction() {
        let tx_hash = "0x4a4bcfef1b7448e27edf533df2f1de9f56be05eba645fb83f42d55816797ad2a";

        let witnesses: Vec<Witness> = vec![
            Witness::default(),
            Witness::default(),
            Witness::default(),
            Witness::default(),
        ];

        let cached_cells = vec![
            CachedCell {
                out_point: Some({
                    OutPoint {
                        tx_hash:
                            "0xe3c3c5b5bd600803286c14acf09f47947735b25e5f5b5b546548c9ceca202cf9"
                                .to_owned(),
                        index: 0,
                    }
                }),
                lock: Some(Script {
                    args: "0xedb5c73f2a4ad8df23467c9f3446f5851b5e33da".to_owned(),
                    code_hash: "0x1892ea40d82b53c678ff88312450bbb17e164d7a3e0a90941aa58839f56f8df2"
                        .to_owned(),
                    hash_type: "type".to_string(),
                }),
                ..CachedCell::default()
            },
            CachedCell {
                out_point: Some({
                    OutPoint {
                        tx_hash:
                            "0xe3c3c5b5bd600803286c14acf09f47947735b25e5f5b5b546548c9ceca202cf9"
                                .to_owned(),
                        index: 1,
                    }
                }),
                lock: Some(Script {
                    args: "0xe2fa82e70b062c8644b80ad7ecf6e015e5f352f6".to_owned(),
                    code_hash: "0x1892ea40d82b53c678ff88312450bbb17e164d7a3e0a90941aa58839f56f8df2"
                        .to_owned(),
                    hash_type: "type".to_owned(),
                }),
                ..CachedCell::default()
            },
            CachedCell {
                out_point: Some({
                    OutPoint {
                        tx_hash:
                            "0xe3c3c5b5bd600803286c14acf09f47947735b25e5f5b5b546548c9ceca202cf9"
                                .to_owned(),
                        index: 2,
                    }
                }),
                lock: Some(Script {
                    args: "0xedb5c73f2a4ad8df23467c9f3446f5851b5e33da".to_owned(),
                    code_hash: "1892ea40d82b53c678ff88312450bbb17e164d7a3e0a90941aa58839f56f8df2"
                        .to_owned(),
                    hash_type: "type".to_owned(),
                }),
                ..CachedCell::default()
            },
        ];

        let inputs = vec![
            CellInput {
                previous_output: Some(OutPoint {
                    tx_hash: "0xe3c3c5b5bd600803286c14acf09f47947735b25e5f5b5b546548c9ceca202cf9"
                        .to_owned(),
                    index: 0,
                }),
                since: "".to_owned(),
            },
            CellInput {
                previous_output: Some(OutPoint {
                    tx_hash: "0xe3c3c5b5bd600803286c14acf09f47947735b25e5f5b5b546548c9ceca202cf9"
                        .to_owned(),
                    index: 1,
                }),
                since: "".to_string(),
            },
            CellInput {
                previous_output: Some(OutPoint {
                    tx_hash: "0xe3c3c5b5bd600803286c14acf09f47947735b25e5f5b5b546548c9ceca202cf9"
                        .to_owned(),
                    index: 2,
                }),
                since: "".to_string(),
            },
        ];

        let mut ks = Keystore::from_private_key(
            "dcec27d0d975b0378471183a03f7071dea8532aaf968be796719ecd20af6988f",
            "Password",
        );
        ks.unlock_by_password("Password").unwrap();

        let coin_info = CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        };

        let account = ks.derive_coin::<CkbAddress>(&coin_info).unwrap().clone();

        let invalid_input = vec![
            (
                CkbTxInput {
                    inputs: inputs.clone(),
                    witnesses: vec![],
                    tx_hash: tx_hash.clone().to_owned(),
                    cached_cells: cached_cells.clone(),
                    ..CkbTxInput::default()
                },
                "required_witness",
            ),
            (
                CkbTxInput {
                    inputs: vec![],
                    witnesses: witnesses.clone(),
                    tx_hash: tx_hash.clone().to_owned(),
                    cached_cells: cached_cells.clone(),
                    ..CkbTxInput::default()
                },
                "invalid_input_cells",
            ),
            (
                CkbTxInput {
                    inputs: inputs.clone(),
                    witnesses: witnesses.clone(),
                    tx_hash: "".to_owned(),
                    cached_cells: cached_cells.clone(),
                    ..CkbTxInput::default()
                },
                "invalid_tx_hash",
            ),
            (
                CkbTxInput {
                    inputs: inputs.clone(),
                    witnesses: witnesses.clone(),
                    tx_hash: tx_hash.clone().to_owned(),
                    cached_cells: vec![],
                    ..CkbTxInput::default()
                },
                "cell_input_not_cached",
            ),
        ];
        for (input, err) in invalid_input {
            let ret = ks.sign_transaction("NERVOS", &account.address, &input);

            assert!(ret.is_err());
            assert_eq!(format!("{}", ret.err().unwrap()), err);
        }
    }
}
