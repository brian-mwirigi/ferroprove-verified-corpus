#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
enum FrameStatus {
    FRAME_OK = 0,
    ERR_SHORT = -1,
    ERR_MAGIC = -2,
};
typedef struct FrameSummary {
    int32_t version;
    int32_t flags;
    int32_t checksum;
    int32_t peak_bucket;
} FrameSummary;
typedef struct State State;
State *ferro_state_new(void);
void ferro_state_free(State *state);
int32_t fold_checksum_c(State *state, int32_t acc, int32_t byte_val);
int32_t inspect_frame_c(State *state, const uint8_t * pkt, size_t n, FrameSummary * out_meta);
#endif