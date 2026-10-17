#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
State *ferro_state_new(void);
void ferro_state_free(State *state);
int32_t u8_word_c(State *state, int32_t hi, int32_t lo);
int32_t read_be16_c(State *state, const uint8_t * p, size_t n, int32_t off);
int32_t tlv_decode_c(State *state, const uint8_t * buf, size_t n, uint8_t * dst, size_t dstcap, int32_t * written);
#endif