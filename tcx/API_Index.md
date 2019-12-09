# API Index

## Action
There is a `call_tcx_api` method in tcx which act as a endpoint like RPC. It accepts a `TcxAction` param which method field is
the real action and param field is the real param of that method.
When an error occurred, the `call_tcx_api` will return a `Response` which isSuccess field be false and error field is the reason 
which cause the error.
Below shows all method and it's param definition. 

## Hd Store
### hd_store_create
create a new hd keystore

__param__: `HdStoreCreateParam`  
__return__: `WalletResult`

### hd_store_import
create a new hd keystore by mnemonic

__param__: `HdStoreImportParam`
__return__: `WalletResult`

### hd_store_derive
derive new accounts from a hd keystore

__param__: `HdStoreDeriveParam`
__return__: `AccountsResponse`


### hd_store_export
export the mnemonic from a hd keystore

__param__: `WalletKeyParam`
__return__: `KeystoreCommonExportResult`

### hd_store_derive
derive new accounts from a hd keystore

__param__: `HdStoreDeriveParam`
__return__: `AccountsResponse`

## Private Key Store
### private_key_store_import
create a new hd keystore by a private key

__param__: `PrivateKeyStoreImportParam`
__return__: `WalletResult`

### private_key_store_export
export the private key from a hd keystore

__param__: `PrivateKeyStoreExportParam`
__return__: `KeystoreCommonExportResult`


## Keystore Common

### keystore_common_verify
verify the password of the keystore

__param__: `WalletKeyParam`
__return__: `Response`

### keystore_common_delete
delete the keystore

__param__: `WalletKeyParam`
__return__: `Response`

### keystore_common_exists
Check is there a keystore was generate by the special privateKey or mnemonic

__param__: `KeystoreCommonExistsParam`
__return__: `KeystoreCommonExistsResult`


### keystore_common_accounts
List all accounts from the keystore

__param__: `KeystoreCommonAccountsParam`
__return__: `AccountsResponse`


## Sign
### sign_tx
Sign transaction.
This api is used for sign any chain_type, you should build the right TxInput instance and put it in the `input` field 

__param__: `SignParam`
__return__: `_TxOut_ define by chain`

#### definition of Tx

##### bch & ltc
__input__: `BtcForkTxInput`
__output__: `BtcForkSignedTxOutput`

##### tron
__input__: `TronTxInput`
__output__: `TronTxOutput`



### tron_sign_message
Sign tron message
This api use the a common struct named `SignParam`, you should build the `TronMessageInput` and put it in the `input` field 

__param__: `SignParam`
__return__: `TronMessageOutput`

