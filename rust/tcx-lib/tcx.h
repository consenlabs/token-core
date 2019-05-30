#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void free_const_string(const char *s);

void free_string(char *s);

const char *read_file(const char *file_path);
