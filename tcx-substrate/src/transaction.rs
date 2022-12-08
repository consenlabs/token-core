#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateKeystoreParam {
    #[prost(string, tag = "1")]
    pub keystore: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub overwrite: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSubstrateKeystoreResult {
    #[prost(string, tag = "1")]
    pub keystore: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateRawTxIn {
    #[prost(string, tag = "1")]
    pub raw_data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateTxOut {
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
}
