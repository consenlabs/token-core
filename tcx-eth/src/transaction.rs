#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTxInput {
    #[prost(string, tag = "1")]
    pub nonce: std::string::String,
    #[prost(string, tag = "2")]
    pub gas_price: std::string::String,
    #[prost(string, tag = "3")]
    pub gas_limit: std::string::String,
    #[prost(string, tag = "4")]
    pub to: std::string::String,
    #[prost(string, tag = "5")]
    pub value: std::string::String,
    #[prost(string, tag = "6")]
    pub data: std::string::String,
    #[prost(uint32, tag = "7")]
    pub chain_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTxOutput {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
    #[prost(string, tag = "2")]
    pub tx_hash: std::string::String,
}
