#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutPoint {
    #[prost(bytes, tag = "1")]
    pub tx_hash: std::vec::Vec<u8>,
    #[prost(int32, tag = "2")]
    pub index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(bytes, tag = "1")]
    pub lock: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub input_type: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub output_type: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Script {
    #[prost(bytes, tag = "1")]
    pub args: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub code_hash: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub hash_type: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CellInput {
    #[prost(message, optional, tag = "1")]
    pub previous_output: ::std::option::Option<OutPoint>,
    #[prost(string, tag = "2")]
    pub since: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CachedCell {
    #[prost(int64, tag = "1")]
    pub capacity: i64,
    #[prost(message, optional, tag = "2")]
    pub lock: ::std::option::Option<Script>,
    #[prost(message, optional, tag = "3")]
    pub r#type: ::std::option::Option<Script>,
    #[prost(message, optional, tag = "4")]
    pub out_point: ::std::option::Option<OutPoint>,
    #[prost(string, tag = "5")]
    pub derive_path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CkbTxInput {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::std::vec::Vec<CellInput>,
    #[prost(message, repeated, tag = "2")]
    pub witnesses: ::std::vec::Vec<Witness>,
    #[prost(message, repeated, tag = "3")]
    pub cached_cells: ::std::vec::Vec<CachedCell>,
    #[prost(bytes, tag = "4")]
    pub tx_hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CkbTxOutput {
    #[prost(bytes, tag = "1")]
    pub tx_hash: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub witnesses: ::std::vec::Vec<Witness>,
}
