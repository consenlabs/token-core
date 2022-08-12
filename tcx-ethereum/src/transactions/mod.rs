pub mod transaction;

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
    #[prost(string, tag = "8")]
    pub access_list: std::string::String,
    #[prost(string, tag = "9")]
    pub max_priority_fee_per_gas: std::string::String,
    #[prost(string, tag = "10")]
    pub transaction_type: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthereumTxOut {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthereumMsgIn {
    #[prost(string, tag = "1")]
    pub value: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthereumMsgOut {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
}
