#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsignedMessage {
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub nonce: u64,
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub gas_limit: i64,
    #[prost(string, tag = "6")]
    pub gas_fee_cap: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub gas_premium: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub method: u64,
    #[prost(string, tag = "9")]
    pub params: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(uint32, tag = "1")]
    pub r#type: u32,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedMessage {
    #[prost(string, tag = "1")]
    pub cid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<UnsignedMessage>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
}
