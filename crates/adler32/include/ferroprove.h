#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t adler32_z_c(int32_t adler, const uint8_t * buf, size_t buf_len, int32_t len);
int32_t adler32_c(int32_t adler, const uint8_t * buf, size_t buf_len, int32_t len);
int32_t adler32_combine__c(int32_t adler1, int32_t adler2, int32_t len2);
int32_t adler32_combine_c(int32_t adler1, int32_t adler2, int32_t len2);
int32_t adler32_combine64_c(int32_t adler1, int32_t adler2, int32_t len2);
#endif