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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletKey {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheDerivedKeyParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub derived_key: std::string::String,
    #[prost(string, tag = "3")]
    pub temp_password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyDerivedKeyParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub derived_key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivedKeyResult {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub derived_key: std::string::String,
}
