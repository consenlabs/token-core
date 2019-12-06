#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcxAction {
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub param: ::std::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub is_success: bool,
    #[prost(string, tag = "2")]
    pub error: std::string::String,
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
//// Hd Store
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreCreateParam {
    #[prost(string, tag = "1")]
    pub password: std::string::String,
    #[prost(string, tag = "2")]
    pub password_hint: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreImportParam {
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
pub struct HdStoreDeriveParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(message, repeated, tag = "3")]
    pub derivations: ::std::vec::Vec<hd_store_derive_param::Derivation>,
}
pub mod hd_store_derive_param {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Derivation {
        #[prost(string, tag = "1")]
        pub chain_type: std::string::String,
        #[prost(string, tag = "2")]
        pub path: std::string::String,
        #[prost(string, tag = "3")]
        pub network: std::string::String,
        #[prost(string, tag = "4")]
        pub seg_wit: std::string::String,
        #[prost(string, tag = "5")]
        pub chain_id: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcForkDeriveExtraParam {
    #[prost(string, tag = "1")]
    pub network: std::string::String,
    #[prost(string, tag = "2")]
    pub seg_wit: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountResponse {
    #[prost(string, tag = "1")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "2")]
    pub address: std::string::String,
    #[prost(string, tag = "3")]
    pub path: std::string::String,
    #[prost(string, tag = "4")]
    pub extended_xpub_key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::std::vec::Vec<AccountResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreExtendedPublicKeyParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "4")]
    pub address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreExtendedPublicKeyResponse {
    #[prost(string, tag = "1")]
    pub extended_public_key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonAccountsParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
//// Private key store
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreImportParam {
    #[prost(string, tag = "1")]
    pub private_key: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "4")]
    pub network: std::string::String,
    #[prost(string, tag = "5")]
    pub seg_wit: std::string::String,
    #[prost(bool, tag = "10")]
    pub overwrite: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreExportParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "4")]
    pub network: std::string::String,
}
//// Keystore Common
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletKeyParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExportResult {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(enumeration = "keystore_common_export_result::ExportType", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub value: std::string::String,
}
pub mod keystore_common_export_result {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExportType {
        Mnemonic = 0,
        PrivateKey = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExistsParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(enumeration = "keystore_common_exists_param::ExportType", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub value: std::string::String,
}
pub mod keystore_common_exists_param {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExportType {
        Mnemonic = 0,
        PrivateKey = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExistsResult {
    #[prost(bool, tag = "1")]
    pub is_exists: bool,
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
//// Sign Transaction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "4")]
    pub address: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub input: ::std::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletResult {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(string, tag = "3")]
    pub source: std::string::String,
    #[prost(message, repeated, tag = "4")]
    pub accounts: ::std::vec::Vec<AccountResponse>,
    #[prost(int64, tag = "5")]
    pub created_at: i64,
}
/// btc-fork
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub chain_type: std::string::String,
    #[prost(uint32, tag = "3")]
    pub external_idx: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressResult {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
    #[prost(string, tag = "2")]
    pub derived_path: std::string::String,
    #[prost(string, tag = "3")]
    pub r#type: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressExtra {
    #[prost(string, tag = "1")]
    pub enc_xpub: std::string::String,
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
