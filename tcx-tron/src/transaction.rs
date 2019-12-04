#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTxInput {
    #[prost(bytes, tag = "1")]
    pub raw_data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTxOutput {
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
