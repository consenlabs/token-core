use crate::serializer::Serializer;
use crate::transaction::{CachedCell, CkbTxInput, Script, Witness};

use super::Error;
use crate::hash::blake2b_256;
use tcx_chain::Result;

impl Script {
    pub fn serialize_hash_type(&self) -> Result<Vec<u8>> {
        match self.hash_type.as_str() {
            "data" => Ok(vec![0x00]),
            "type" => Ok(vec![0x01]),
            _ => Err(Error::InvalidHashType.into()),
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(Serializer::serialize_dynamic_vec(&vec![
            self.code_hash.clone(),
            self.serialize_hash_type()?,
            Serializer::serialize_fixed_vec(&vec![self.args.clone()]),
        ]))
    }

    pub fn to_hash(&self) -> Result<Vec<u8>> {
        Ok(blake2b_256(&self.serialize()?))
    }
}

impl Witness {
    pub fn serialize(&self) -> Vec<u8> {
        let inner_serialize = |x: &Vec<u8>| {
            if x.len() > 0 {
                Serializer::serialize_fixed_vec(&vec![x.clone()])
            } else {
                vec![]
            }
        };

        Serializer::serialize_dynamic_vec(&vec![
            inner_serialize(&self.lock),
            inner_serialize(&self.input_type),
            inner_serialize(&self.output_type),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{Script, Witness};

    #[test]
    fn serialize_script() {
        let script = Script {
            code_hash: hex::decode(
                "68d5438ac952d2f584abf879527946a537e82c7f3c1cbf6d8ebf9767437d8e88",
            )
            .unwrap(),
            args: hex::decode("3954acece65096bfa81258983ddb83915fc56bd8").unwrap(),
            hash_type: "type".to_string(),
        };

        assert_eq!(script.serialize().unwrap(), hex::decode("4900000010000000300000003100000068d5438ac952d2f584abf879527946a537e82c7f3c1cbf6d8ebf9767437d8e8801140000003954acece65096bfa81258983ddb83915fc56bd8").unwrap());

        let script = Script {
            code_hash: hex::decode(
                "68d5438ac952d2f584abf879527946a537e82c7f3c1cbf6d8ebf9767437d8e88",
            )
            .unwrap(),
            args: vec![],
            hash_type: "type".to_string(),
        };

        assert_eq!(script.serialize().unwrap(), hex::decode("3500000010000000300000003100000068d5438ac952d2f584abf879527946a537e82c7f3c1cbf6d8ebf9767437d8e880100000000").unwrap());
    }

    #[test]
    fn script_to_hash() {
        let script = Script {
            code_hash: hex::decode(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            args: vec![],
            hash_type: "data".to_string(),
        };

        assert_eq!(
            script.to_hash().unwrap(),
            hex::decode("77c93b0632b5b6c3ef922c5b7cea208fb0a7c427a13d50e13d3fefad17e0c590")
                .unwrap()
        );

        let script = Script {
            code_hash: hex::decode(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            args: vec![0x1],
            hash_type: "data".to_string(),
        };
        assert_eq!(
            script.to_hash().unwrap(),
            hex::decode("67951b34bce20cb71b7e235c1f8cda259628d99d94825bffe549c23b4dd2930f")
                .unwrap()
        );

        let script = Script {
            code_hash: hex::decode(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            args: vec![0x1],
            hash_type: "type".to_string(),
        };

        assert_eq!(
            script.to_hash().unwrap(),
            hex::decode("d39f84d4702f53cf8625da4411be1640b961715cb36816501798fedb70b6e0fb")
                .unwrap()
        );
    }

    #[test]
    fn serialize_witness() {
        let witness = Witness {
            lock: vec![],
            input_type: vec![],
            output_type: vec![],
        };

        assert_eq!(
            hex::encode(witness.serialize()),
            "10000000100000001000000010000000"
        );

        let witness = Witness {
            lock: vec![],
            input_type: vec![0x10],
            output_type: vec![0x20],
        };
        assert_eq!(
            hex::encode(witness.serialize()),
            "1a00000010000000100000001500000001000000100100000020"
        );
    }
}
