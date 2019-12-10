/// This file only contains tron related messages.
// ref: https://developers.tron.network/docs/transaction

/// FUNCTION: sign_tx(SignParam{input: TronTxInput}): TronTxOutput
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTxInput {
    #[prost(bytes, tag = "1")]
    pub raw_data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTxOutput {
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
}
/// FUNCTION: tron_sign_message(SignParam): TronMessageOutput
///
/// This api use the a common struct named `SignParam`, you should
/// build the `TronMessageInput` and put it in the `input` field
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronMessageInput {
    #[prost(string, tag = "1")]
    pub value: std::string::String,
    #[prost(bool, tag = "2")]
    pub is_hex: bool,
    #[prost(bool, tag = "3")]
    pub is_tron_header: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronMessageOutput {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
