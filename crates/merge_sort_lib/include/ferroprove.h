#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct spritebatch_sprite_t {
    int32_t texture_id;
    int32_t sort_bits;
} spritebatch_sprite_t;
typedef struct State State;
int32_t spritebatch_internal_sprite_less_than_or_equal_c(spritebatch_sprite_t * a, spritebatch_sprite_t * b);
int32_t spritebatch_internal_merge_sort_iteration_c(spritebatch_sprite_t * a, size_t a_len, int32_t lo, int32_t split, int32_t hi, spritebatch_sprite_t * b, size_t b_len);
int32_t spritebatch_internal_merge_sort_recurse_c(spritebatch_sprite_t * b, size_t b_len, int32_t lo, int32_t hi, spritebatch_sprite_t * a, size_t a_len);
int32_t merge_sort_c(spritebatch_sprite_t * a, size_t a_len, spritebatch_sprite_t * b, size_t b_len, int32_t size);
#endif