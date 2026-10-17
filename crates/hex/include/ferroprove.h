#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t hex_to_bytes_c(uint8_t * binary, size_t binary_len, const uint8_t * hex, size_t hex_len, int32_t len);
#endif