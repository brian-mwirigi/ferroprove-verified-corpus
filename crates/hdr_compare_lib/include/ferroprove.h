#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t hdr_valid_c(const uint8_t * h, size_t h_len);
int32_t hdr_compare_c(const uint8_t * h1, size_t h1_len, const uint8_t * h2, size_t h2_len);
#endif