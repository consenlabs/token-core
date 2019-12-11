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
pub struct CellOutput {
    #[prost(int64, tag = "1")]
    pub capacity: i64,
    #[prost(message, optional, tag = "2")]
    pub lock: ::std::option::Option<Script>,
    #[prost(message, optional, tag = "3")]
    pub r#type: ::std::option::Option<Script>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CellDep {
    #[prost(message, optional, tag = "1")]
    pub out_point: ::std::option::Option<OutPoint>,
    #[prost(string, tag = "2")]
    pub dep_type: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CachedCell {
    #[prost(bytes, tag = "1")]
    pub block_hash: std::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    pub capacity: i64,
    #[prost(message, optional, tag = "3")]
    pub lock: ::std::option::Option<Script>,
    #[prost(message, optional, tag = "4")]
    pub out_point: ::std::option::Option<OutPoint>,
    #[prost(bool, tag = "5")]
    pub cellbase: bool,
    #[prost(int32, tag = "6")]
    pub output_data_len: i32,
    #[prost(string, tag = "7")]
    pub status: std::string::String,
    #[prost(bytes, tag = "8")]
    pub data_hash: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "9")]
    pub r#type: ::std::option::Option<Script>,
    #[prost(string, tag = "10")]
    pub derive_path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    #[prost(string, tag = "1")]
    pub version: std::string::String,
    #[prost(message, repeated, tag = "2")]
    pub cell_deps: ::std::vec::Vec<CellDep>,
    #[prost(bytes, repeated, tag = "3")]
    pub header_deps: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::std::vec::Vec<CellInput>,
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::std::vec::Vec<CellOutput>,
    #[prost(message, repeated, tag = "6")]
    pub witnesses: ::std::vec::Vec<Witness>,
    #[prost(bytes, repeated, tag = "7")]
    pub outputs_data: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "8")]
    pub cached_cells: ::std::vec::Vec<CachedCell>,
    #[prost(bytes, tag = "9")]
    pub tx_hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    #[prost(message, repeated, tag = "1")]
    pub witnesses: ::std::vec::Vec<Witness>,
}
