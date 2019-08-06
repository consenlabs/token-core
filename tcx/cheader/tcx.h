#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "TWBase.h"

typedef const char TWString;

TW_EXTERN_C_BEGIN

/// Represents a key stored as an encrypted file.
TW_EXPORT_CLASS
struct TWApi;

TW_EXPORT_STATIC_METHOD
void TWApiClear_err(void);

TW_EXPORT_STATIC_METHOD
void TWApiFree_const_string(TWString *_Nonnull s);

TW_EXPORT_STATIC_METHOD
void TWApiFree_string(TWString *_Nonnull s);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiGet_last_err_message(void);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiImport_bch_wallet_from_mnemonic(TWString *_Nonnull mnemonic, TWString *_Nonnull password);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiImport_wallet_from_mnemonic(TWString *_Nonnull json_str);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiImport_wallet_from_private_key(TWString *_Nonnull json_str);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiRead_file(TWString *_Nonnull file_path);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiRead_file_error(void);

TW_EXPORT_STATIC_METHOD
void TWApiScan_wallets(TWString *_Nonnull json_str);

TW_EXPORT_STATIC_METHOD
TWString *_Nonnull TWApiSign_transaction(TWString *_Nonnull json_str);


TW_EXTERN_C_END