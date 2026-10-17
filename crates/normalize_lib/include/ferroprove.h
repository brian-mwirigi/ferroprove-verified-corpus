#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t normalize_c(float * dest, size_t dest_len, const float * src, size_t src_len, int32_t size);
#endif