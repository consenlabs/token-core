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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosKeystoreParam {
    #[prost(string, tag = "1")]
    pub keystore: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(bool, tag = "4")]
    pub overwrite: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTezosKeystoreResult {
    #[prost(string, tag = "1")]
    pub keystore: std::string::String,
}
