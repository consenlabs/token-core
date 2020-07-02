/// Action Wrapper
/// There is a `call_tcx_api` method in tcx which act as a endpoint like RPC. It accepts a `TcxAction` param which method field is
/// the real action and param field is the real param of that method.
/// When an error occurred, the `call_tcx_api` will return a `Response` which isSuccess field be false and error field is the reason
/// which cause the error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcxAction {
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub param: ::std::option::Option<::prost_types::Any>,
}
/// A common response when error occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub is_success: bool,
    #[prost(string, tag = "2")]
    pub error: std::string::String,
}
/// Initialization

/// FUNCTION: init_token_core_x(InitTokenCoreXParam)
///
/// initialize tcx by passing keystore folder and xpub encryption params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitTokenCoreXParam {
    #[prost(string, tag = "1")]
    pub file_dir: std::string::String,
    #[prost(string, tag = "2")]
    pub xpub_common_key: std::string::String,
    #[prost(string, tag = "3")]
    pub xpub_common_iv: std::string::String,
    #[prost(bool, tag = "4")]
    pub is_debug: bool,
}
///
///// FUNCTION: export_private_key(ExportPrivateKeyParam): KeystoreCommonExportResult
/////
///// export the private key from a private key keystore or a hd keystore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportPrivateKeyParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "4")]
    pub network: std::string::String,
    #[prost(string, tag = "5")]
    pub main_address: std::string::String,
    #[prost(string, tag = "6")]
    pub path: std::string::String,
}
///
////// Keystore Common
///
///// FUNCTION: keystore_common_verify(WalletKeyParam) -> Response
/////
///// verify the password of the keystore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletKeyParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
}
/// Hd Store

/// FUNCTION: hd_store_create(HdStoreCreateParam): WalletResult
///
/// create a new hd keystore
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
/// FUNCTION: hd_store_import(HdStoreImportParam): WalletResult
///
/// create a new hd keystore by mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreImportParam {
    #[prost(string, tag = "1")]
    pub mnemonic: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub source: std::string::String,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
    #[prost(string, tag = "5")]
    pub password_hint: std::string::String,
    #[prost(bool, tag = "6")]
    pub overwrite: bool,
}
/// FUNCTION: hd_store_derive(HdStoreDeriveParam): AccountsResponse
///
/// derive new accounts from a hd keystore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonDeriveParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(message, repeated, tag = "3")]
    pub derivations: ::std::vec::Vec<keystore_common_derive_param::Derivation>,
}
pub mod keystore_common_derive_param {
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
/// FUNCTION: hd_store_export(KeystoreCommonExportResult): KeystoreCommonExistsResult
///
/// export the mnemonic from a hd keystore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExportResult {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(enumeration = "KeyType", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub value: std::string::String,
}
/// Private Key Store

/// FUNCTION: private_key_store_import(PrivateKeyStoreImportParam): WalletResult
///
/// create a new private key keystore by a private key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreImportParam {
    #[prost(string, tag = "1")]
    pub private_key: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(bool, tag = "3")]
    pub overwrite: bool,
}
/// FUNCTION: private_key_store_export(PrivateKeyStoreExportParam): KeystoreCommonExportResult
///
/// export the private key from a private key keystore
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
/// Keystore Common

// FUNCTION: keystore_common_delete(WalletKeyParam) -> Response
//
// delete the keystore

/// FUNCTION: keystore_common_exists(KeystoreCommonExistsParam): KeystoreCommonExistsResult
///
/// Check is there a keystore was generate by the special privateKey or mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExistsParam {
    #[prost(enumeration = "KeyType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExistsResult {
    #[prost(bool, tag = "1")]
    pub is_exists: bool,
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
/// FUNCTION: keystore_common_accounts(KeystoreCommonAccountsParam): AccountsResponse
///
/// List all accounts from the keystore
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonAccountsParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
/// Sign Transaction

/// FUNCTION: sign_tx(SignParam)
///
/// Sign transaction. This api is used for sign any chain_type, you should build the right TxInput instance and
/// put it in the `input` field
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignParam {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(string, tag = "4")]
    pub chain_type: std::string::String,
    #[prost(string, tag = "5")]
    pub address: std::string::String,
    #[prost(message, optional, tag = "6")]
    pub input: ::std::option::Option<::prost_types::Any>,
    #[prost(oneof = "sign_param::Key", tags = "2, 3")]
    pub key: ::std::option::Option<sign_param::Key>,
}
pub mod sign_param {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Key {
        #[prost(string, tag = "2")]
        Password(std::string::String),
        #[prost(string, tag = "3")]
        DerivedKey(std::string::String),
    }
}
/// Other
// TODO: annotate following message usage

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
pub struct BtcForkDeriveExtraParam {
    #[prost(string, tag = "1")]
    pub network: std::string::String,
    #[prost(string, tag = "2")]
    pub seg_wit: std::string::String,
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
pub struct ImportSubstrateKeystoreParam {
    #[prost(string, tag = "1")]
    pub keystore: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: std::string::String,
    #[prost(bool, tag = "4")]
    pub r#override: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSubstrateKeystoreResult {
    #[prost(string, tag = "1")]
    pub keystore: std::string::String,
}
/// only support two types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Mnemonic = 0,
    PrivateKey = 1,
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
/// Only used in Android or iOS

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheDerivedKeyResult {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(bool, tag = "2")]
    pub enable_derived_key: bool,
    #[prost(string, tag = "3")]
    pub mode: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletId {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiometricModeResult {
    #[prost(string, tag = "1")]
    pub mode: std::string::String,
}
