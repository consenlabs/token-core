/// Action Wrapper
/// There is a `call_tcx_api` method in tcx which act as a endpoint like RPC. It accepts a `TcxAction` param which method field is
/// the real action and param field is the real param of that method.
/// When an error occurred, the `call_tcx_api` will return a `Response` which isSuccess field be false and error field is the reason
/// which cause the error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcxAction {
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub param: ::core::option::Option<::prost_types::Any>,
}
/// A common response when error occurred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub is_success: bool,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
/// FUNCTION: init_token_core_x(InitTokenCoreXParam)
///
/// initialize tcx by passing keystore folder and xpub encryption params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitTokenCoreXParam {
    #[prost(string, tag = "1")]
    pub file_dir: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub xpub_common_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub xpub_common_iv: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_debug: bool,
}
///
/// // FUNCTION: export_private_key(ExportPrivateKeyParam): KeystoreCommonExportResult
/// //
/// // export the private key from a private key keystore or a hd keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportPrivateKeyParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub network: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub main_address: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub path: ::prost::alloc::string::String,
}
///
/// /// Keystore Common
///
/// // FUNCTION: keystore_common_verify(WalletKeyParam) -> Response
/// //
/// // verify the password of the keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletKeyParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// FUNCTION: hd_store_create(HdStoreCreateParam): WalletResult
///
/// create a new hd keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreCreateParam {
    #[prost(string, tag = "1")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password_hint: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub accounts: ::prost::alloc::vec::Vec<AccountResponse>,
    #[prost(int64, tag = "5")]
    pub created_at: i64,
}
/// FUNCTION: hd_store_import(HdStoreImportParam): WalletResult
///
/// create a new hd keystore by mnemonic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreImportParam {
    #[prost(string, tag = "1")]
    pub mnemonic: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub password_hint: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub overwrite: bool,
}
/// FUNCTION: hd_store_derive(HdStoreDeriveParam): AccountsResponse
///
/// derive new accounts from a hd keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonDeriveParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub derivations: ::prost::alloc::vec::Vec<keystore_common_derive_param::Derivation>,
}
/// Nested message and enum types in `KeystoreCommonDeriveParam`.
pub mod keystore_common_derive_param {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Derivation {
        #[prost(string, tag = "1")]
        pub chain_type: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub network: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub seg_wit: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub chain_id: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub curve: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountResponse {
    #[prost(string, tag = "1")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub extended_xpub_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccountResponse>,
}
/// FUNCTION: hd_store_export(KeystoreCommonExportResult): KeystoreCommonExistsResult
///
/// export the mnemonic from a hd keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExportResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "KeyType", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
/// FUNCTION: private_key_store_import(PrivateKeyStoreImportParam): WalletResult
///
/// create a new private key keystore by a private key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreImportParam {
    #[prost(string, tag = "1")]
    pub private_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub password_hint: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub overwrite: bool,
    #[prost(string, tag = "6")]
    pub encoding: ::prost::alloc::string::String,
}
/// FUNCTION: private_key_store_export(PrivateKeyStoreExportParam): KeystoreCommonExportResult
///
/// export the private key from a private key keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreExportParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub network: ::prost::alloc::string::String,
}
/// FUNCTION: keystore_common_exists(KeystoreCommonExistsParam): KeystoreCommonExistsResult
///
/// Check is there a keystore was generate by the special privateKey or mnemonic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExistsParam {
    #[prost(enumeration = "KeyType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encoding: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonExistsResult {
    #[prost(bool, tag = "1")]
    pub is_exists: bool,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// FUNCTION: keystore_common_accounts(KeystoreCommonAccountsParam): AccountsResponse
///
/// List all accounts from the keystore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeystoreCommonAccountsParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// FUNCTION: sign_tx(SignParam)
///
/// Sign transaction. This api is used for sign any chain_type, you should build the right TxInput instance and
/// put it in the `input` field
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub input: ::core::option::Option<::prost_types::Any>,
    #[prost(oneof = "sign_param::Key", tags = "2, 3")]
    pub key: ::core::option::Option<sign_param::Key>,
}
/// Nested message and enum types in `SignParam`.
pub mod sign_param {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Key {
        #[prost(string, tag = "2")]
        Password(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        DerivedKey(::prost::alloc::string::String),
    }
}
/// btc-fork
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub external_idx: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressResult {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub derived_path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddressExtra {
    #[prost(string, tag = "1")]
    pub enc_xpub: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub external_address: ::core::option::Option<external_address_extra::ExternalAddress>,
}
/// Nested message and enum types in `ExternalAddressExtra`.
pub mod external_address_extra {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExternalAddress {
        #[prost(string, tag = "1")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub derived_path: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub r#type: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcForkDeriveExtraParam {
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub seg_wit: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreExtendedPublicKeyParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HdStoreExtendedPublicKeyResponse {
    #[prost(string, tag = "1")]
    pub extended_public_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub chain_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub public_key: ::prost::alloc::string::String,
}
/// only support two types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Mnemonic = 0,
    PrivateKey = 1,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::Mnemonic => "MNEMONIC",
            KeyType::PrivateKey => "PRIVATE_KEY",
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyDerivedKeyParam {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub derived_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivedKeyResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub derived_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheDerivedKeyResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enable_derived_key: bool,
    #[prost(string, tag = "3")]
    pub mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletId {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiometricModeResult {
    #[prost(string, tag = "1")]
    pub mode: ::prost::alloc::string::String,
}
