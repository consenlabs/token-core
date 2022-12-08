#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutPoint {
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(string, tag = "1")]
    pub lock: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub input_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub output_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Script {
    #[prost(string, tag = "1")]
    pub args: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub code_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub hash_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CellInput {
    #[prost(message, optional, tag = "1")]
    pub previous_output: ::core::option::Option<OutPoint>,
    #[prost(string, tag = "2")]
    pub since: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CachedCell {
    #[prost(int64, tag = "1")]
    pub capacity: i64,
    #[prost(message, optional, tag = "2")]
    pub lock: ::core::option::Option<Script>,
    #[prost(message, optional, tag = "3")]
    pub out_point: ::core::option::Option<OutPoint>,
    #[prost(string, tag = "4")]
    pub derived_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CkbTxInput {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<CellInput>,
    #[prost(message, repeated, tag = "2")]
    pub witnesses: ::prost::alloc::vec::Vec<Witness>,
    #[prost(message, repeated, tag = "3")]
    pub cached_cells: ::prost::alloc::vec::Vec<CachedCell>,
    #[prost(string, tag = "4")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CkbTxOutput {
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub witnesses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
