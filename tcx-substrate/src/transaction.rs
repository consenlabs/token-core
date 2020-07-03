#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateKeystoreParam {
    #[prost(string, tag = "1")]
    pub keystore: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(bool, tag = "4")]
    pub r#override: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSubstrateKeystoreResult {
    #[prost(string, tag = "1")]
    pub keystore: std::string::String,
}
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
