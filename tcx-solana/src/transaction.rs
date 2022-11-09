#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolanaTxIn {
    #[prost(bytes, tag = "1")]
    pub to: std::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    #[prost(bytes, tag = "3")]
    pub recent_blockhash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "4")]
    pub token_from: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolanaTxOut {
    #[prost(string, tag = "1")]
    pub tx: std::string::String,
}
