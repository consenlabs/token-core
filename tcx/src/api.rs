#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcxAction {
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub param: ::std::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitTokenCoreXParam {
    #[prost(string, tag = "1")]
    pub file_dir: std::string::String,
    #[prost(string, tag = "2")]
    pub xpub_common_key: std::string::String,
    #[prost(string, tag = "3")]
    pub xpub_common_iv: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitTokenCoreXParamCopy {
    #[prost(string, tag = "1")]
    pub file_dir1: std::string::String,
    #[prost(string, tag = "2")]
    pub xpub_common_key: std::string::String,
    #[prost(string, tag = "3")]
    pub xpub_common_iv: std::string::String,
}
//r#"{
//"chainType":"BITCOINCASH",
//"mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
//"name":"BCH-Wallet-1",
//"network":"MAINNET",
//"overwrite":true,
//"password":"Insecure Password",
//"passwordHint":"",
//"path":"m/44'/145'/0'/0/0",
//"segWit":"NONE",
//"source":"MNEMONIC"
//}"#;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportWalletFromMnemonicParam {
    #[prost(string, tag = "1")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "2")]
    pub mnemonic: std::string::String,
    #[prost(string, tag = "3")]
    pub password: std::string::String,
    #[prost(string, tag = "4")]
    pub path: std::string::String,
    #[prost(string, tag = "5")]
    pub source: std::string::String,
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    #[prost(string, tag = "7")]
    pub network: std::string::String,
    #[prost(string, tag = "8")]
    pub seg_wit: std::string::String,
    #[prost(string, tag = "9")]
    pub password_hint: std::string::String,
    #[prost(bool, tag = "10")]
    pub overwrite: bool,
}
//{
//"address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
//"chainType": "LITECOIN",
//"createdAt": 1566455834,
//"encXPub": "GekyMLycBJlFAmob0yEGM8zrEKrBHozAKr66PrMts7k6vSBJ/8DJQW7HViVqWftKhRbPAxZ3MO0281AKvWp4qa+/Q5nqoCi5/THxRLA1wDn8gWqDJjUjaZ7kJaNnreWfUyNGUeDxnN7tHDGdW4nbtA==",
//"externalAddress": {
//"address": "mj78AbVtQ9SWnvbU7pcrueyE1krMmZtoUU",
//"derivedPath": "0/1",
//"type": "EXTERNAL"
//},
//"id": "fdb5e9d4-530d-46ed-bf4a-6a27fb8eddca",
//"name": "LTC-Wallet-1",
//"passwordHint": "",
//"source": "MNEMONIC"
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletResult {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "4")]
    pub address: std::string::String,
    #[prost(string, tag = "5")]
    pub source: std::string::String,
    #[prost(int64, tag = "6")]
    pub created_at: i64,
    #[prost(message, optional, tag = "7")]
    pub extra: ::std::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressExtra {
    #[prost(string, tag = "1")]
    pub enc_x_pub: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub external_address: ::std::option::Option<external_address_extra::ExternalAddress>,
}
pub mod external_address_extra {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExternalAddress {
        #[prost(string, tag = "1")]
        pub address: std::string::String,
        #[prost(string, tag = "2")]
        pub derived_path: std::string::String,
        #[prost(string, tag = "3")]
        pub r#type: std::string::String,
    }
}
