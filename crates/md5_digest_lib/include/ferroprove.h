#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct tflac_md5 {
    int32_t a;
    int32_t b;
    int32_t c;
    int32_t d;
} tflac_md5;
typedef struct State State;
int32_t md5_digest_c(const tflac_md5 * m, uint8_t * out, size_t out_len);
#endif