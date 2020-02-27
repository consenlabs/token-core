#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtrinsicEra {
    #[prost(uint64, tag = "1")]
    pub current: u64,
    #[prost(uint64, tag = "2")]
    pub period: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateTxIn {
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    #[prost(string, tag = "2")]
    pub address: std::string::String,
    #[prost(int64, tag = "3")]
    pub amount: i64,
    #[prost(message, optional, tag = "4")]
    pub era: ::std::option::Option<ExtrinsicEra>,
    #[prost(uint32, tag = "5")]
    pub nonce: u32,
    #[prost(uint64, tag = "6")]
    pub tip: u64,
    #[prost(uint32, tag = "7")]
    pub sepc_version: u32,
    #[prost(string, tag = "8")]
    pub genesis_hash: std::string::String,
    #[prost(string, tag = "9")]
    pub block_hash: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubstrateTxOut {
    ///    string signature = 2;
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
