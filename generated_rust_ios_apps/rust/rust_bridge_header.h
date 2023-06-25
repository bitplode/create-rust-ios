#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * # Safety
 *
 * This function should not be called too early!
 */
char *rust_greet(const char *to);

/**
 * # Safety
 *
 * This function should not be called too early!
 */
void rust_greet_free(char *s);
