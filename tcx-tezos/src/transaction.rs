#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosRawTxIn {
    #[prost(string, tag = "1")]
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosTxOut {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
