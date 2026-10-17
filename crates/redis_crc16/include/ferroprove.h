#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t crc16_c(const uint8_t * buf, size_t buf_len, int32_t len);
#endif