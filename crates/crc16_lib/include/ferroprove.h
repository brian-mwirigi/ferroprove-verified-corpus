#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t crc16_c(const uint8_t * d, size_t d_len, int32_t len, int32_t crc16);
#endif