#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosRawTxIn {
    #[prost(string, tag = "1")]
    pub raw_data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TezosTxOut {
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub edsig: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sbytes: ::prost::alloc::string::String,
}
