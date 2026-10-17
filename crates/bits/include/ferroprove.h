#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
State *ferro_state_new(void);
void ferro_state_free(State *state);
int32_t bit_from_byte_c(State *state, int32_t b, int32_t bit);
int32_t pull_bits_c(State *state, const uint8_t * buf, size_t nbytes, int32_t nbits, int32_t * out);
#endif