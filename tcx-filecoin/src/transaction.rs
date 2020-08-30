use tcx_chain::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct UnsignedMessage {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(string, tag = "2")]
    pub to: std::string::String,
    #[prost(string, tag = "3")]
    pub from: std::string::String,
    #[prost(int64, tag = "4")]
    pub nonce: i64,
    #[prost(string, tag = "5")]
    pub value: std::string::String,
    #[prost(string, tag = "6")]
    pub gas_price: std::string::String,
    #[prost(string, tag = "7")]
    pub gas_limit: i64,
    #[propst(int64, tag = "8")]
    pub method: i64,
    #[prost(string, tag = "9")]
    pub params: [u8],
}

impl UnsignedMessage {
    pub fn to_cobr(&self) -> Result<[u8]> {
        Ok([0u8])
    }
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct SignedMessage {
    #[prost(string, tag = "1")]
    pub signature: std::string::String,

    #[prost(string, tag = "2")]
    pub message: UnsignedMessage,
}

#[cfg(test)]
mod tests {

}





