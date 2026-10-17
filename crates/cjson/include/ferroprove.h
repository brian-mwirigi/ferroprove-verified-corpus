#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
State *ferro_state_new(void);
void ferro_state_free(State *state);
int32_t hex_digit_c(State *state, int32_t c);
int32_t hex_byte_c(State *state, int32_t hi, int32_t lo);
int32_t scan_string_c(State *state, const uint8_t * src, size_t n, uint8_t * out, size_t outcap, int32_t * written);
#endif