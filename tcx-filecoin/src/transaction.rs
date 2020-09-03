#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsignedMessage {
    #[prost(string, tag = "1")]
    pub to: std::string::String,
    #[prost(string, tag = "2")]
    pub from: std::string::String,
    #[prost(uint64, tag = "3")]
    pub nonce: u64,
    #[prost(string, tag = "4")]
    pub value: std::string::String,
    #[prost(int64, tag = "5")]
    pub gas_limit: i64,
    #[prost(string, tag = "6")]
    pub gas_fee_cap: std::string::String,
    #[prost(string, tag = "7")]
    pub gas_premium: std::string::String,
    #[prost(uint64, tag = "8")]
    pub method: u64,
    #[prost(string, tag = "9")]
    pub params: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedMessage {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub message: ::std::option::Option<UnsignedMessage>,
}
