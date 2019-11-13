#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

const char *cache_derived_key(const char *json_str);

const char *calc_external_address(const char *json_str);

const char *clear_derived_key(void);

void clear_err(void);

const char *create_wallet(const char *json_str);

const char *export_mnemonic(const char *json_str);

const char *find_wallet_by_mnemonic(const char *json_str);

const char *export_private_key(const char *json_str);

const char *find_wallet_by_mnemonic(const char *json_str);

const char *find_wallet_by_private_key(const char *json_str);

void free_const_string(const char *s);

void free_string(char *s);

const char *get_derived_key(const char *json_str);

const char *get_last_err_message(void);

const char *import_wallet_from_mnemonic(const char *json_str);

const char *import_wallet_from_private_key(const char *json_str);

void init_token_core_x(const char *json_str);

const char *remove_wallet(const char *json_str);

const char *sign_message(const char *json_str);

const char *sign_transaction(const char *json_str);

const char *verify_derived_key(const char *json_str);

const char *verify_password(const char *json_str);
