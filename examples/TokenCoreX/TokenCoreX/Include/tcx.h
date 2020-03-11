#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * dispatch protobuf rpc call
 */
const char *call_tcx_api(const char *hex_str);

void clear_err(void);

void free_const_string(const char *s);

const char *get_last_err_message(void);

void init_token_core_x(const char *json_str);
