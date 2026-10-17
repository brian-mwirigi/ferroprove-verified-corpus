#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t timingsafe_bcmp_c(const uint8_t * b1, size_t b1_len, const uint8_t * b2, size_t b2_len, int32_t n);
#endif