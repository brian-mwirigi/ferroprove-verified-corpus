#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t btac1c2_fakesqrt_c(int32_t val);
int32_t stereo_samples_c(int16_t * ibuf0, size_t ibuf0_len, int16_t * ibuf1, size_t ibuf1_len, int32_t len);
#endif