#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t crc8_populate_msb_c(uint8_t * table, size_t table_len, int32_t polynomial);
int32_t crc8_populate_lsb_c(uint8_t * table, size_t table_len, int32_t polynomial);
int32_t crc8_c(const uint8_t * table, size_t table_len, const uint8_t * pdata, size_t pdata_len, int32_t nbytes, int32_t crc);
#endif