#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
enum TFLAC_CHANNEL_MODE {
    TFLAC_CHANNEL_INDEPENDENT = 0,
    TFLAC_CHANNEL_LEFT_SIDE = 1,
    TFLAC_CHANNEL_SIDE_RIGHT = 2,
    TFLAC_CHANNEL_MID_SIDE = 3,
    TFLAC_CHANNEL_MODE_COUNT = 4,
};
typedef struct tflac {
    int32_t blocksize;
    int32_t samplerate;
    int32_t channels;
    int32_t bitdepth;
    int32_t channel_mode;
    int32_t max_rice_value;
    int32_t min_partition_order;
    int32_t max_partition_order;
    int32_t partition_order;
    int32_t cur_blocksize;
} tflac;
typedef struct State State;
int32_t tflac_size_memory_c(int32_t blocksize);
int32_t flac_validate_c(tflac * t);
#endif