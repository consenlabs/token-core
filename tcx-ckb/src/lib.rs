mod hash;
mod transaction;
//mod signer;
mod serializer;
mod transaction_helper;

use failure::Fail;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    #[fail(display = "invalid_output_point")]
    InvalidOutputPoint,

    #[fail(display = "invalid_outputs_data_length")]
    InvalidOutputsDataLength,

    #[fail(display = "required_witness")]
    RequiredWitness,

    #[fail(display = "required_output_data")]
    RequiredOutputsData,

    #[fail(display = "witness_group_empty")]
    WitnessGroupEmpty,

    #[fail(display = "witness_empty")]
    WitnessEmpty,

    #[fail(display = "invalid_tx_hash")]
    InvalidTxHash,

    #[fail(display = "invalid_hash_type")]
    InvalidHashType,
}
