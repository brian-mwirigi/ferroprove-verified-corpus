#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
State *ferro_state_new(void);
void ferro_state_free(State *state);
int32_t commit_c(State *state, int32_t v);
int32_t eval_expr_c(State *state, const uint8_t * s, size_t n, int32_t * out);
#endif