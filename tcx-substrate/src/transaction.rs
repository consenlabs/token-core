#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateRawTxIn {
    #[prost(string, tag = "1")]
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateTxOut {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
