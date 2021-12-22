#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthereumTxIn {
    #[prost(string, tag = "1")]
    pub nonce: std::string::String,
    #[prost(string, tag = "2")]
    pub to: std::string::String,
    #[prost(string, tag = "3")]
    pub value: std::string::String,
    #[prost(string, tag = "4")]
    pub gas_price: std::string::String,
    #[prost(string, tag = "5")]
    pub gas: std::string::String,
    #[prost(string, tag = "6")]
    pub data: std::string::String,
    #[prost(string, tag = "7")]
    pub network: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthereumTxOut {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
