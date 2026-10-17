#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t encode_quant_c(int32_t uni, int32_t step, int32_t pred, int32_t tgt, int32_t tgt2, int32_t lsbit);
#endif