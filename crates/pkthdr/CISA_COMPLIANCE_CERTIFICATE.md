# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Struct: Packet header & histogram
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `9e883699db7cfb52` (48 lines)
- **Safe Rust Crate Digest (FNV-64):** `481ee981e25bd02f` (80 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `9a984f6d584d13a3`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (333 ms, 24748 B .rlib, sha256:c8a1a34d8f1e6fa1)

---

## 1. Static Memory-Safety Gate (`src/lib.rs`)

| Check | Requirement | Measured | Status |
| :--- | :--- | :--- | :--- |
| Crate-Level Directive | `#![forbid(unsafe_code)]` present | `true` | PASS |
| `unsafe` Blocks / Fn | `0` in safe core | `0` | PASS |
| Raw Pointer Derefs (`*mut` / `*const`) | `0` in safe core | `0` | PASS |
| Incomplete Stubs (`todo!()` / `unimplemented!()`) | `0` in safe core | `0` | PASS |
| Ambient Mutable Globals | Isolated into explicit `State` struct | `1 -> &mut State` | PASS |

---

## 2. Topological Call-Graph & Hazard Inventory

- **Leaf-to-Root Translation Order:** `fold_checksum -> inspect_frame`
- **Mutable Globals Threaded into `State`:** `g_frames` (L20)
- **Legacy Memory Hazards Eliminated (7):**
  - `L20`: mutable global g_frames -> collapsed into `State` struct & threaded as `&mut State`
  - `L33`: unchecked index pkt[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L34`: unchecked index pkt[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L35`: unchecked index pkt[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L38`: unchecked index pkt[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L41`: unchecked index hist[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L44`: unchecked index hist[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 4
- **Byte-Exact Functional Parity:** 3 / 3
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 1
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| FP v1 low-ascii payload | `pkt[7]=[0x46 0x50 0x01 0x03 0x20 0x41 0x42]` | `return 0  out_meta.version=1  out_meta.flags=3  out_meta.checksum=65  out_meta.peak_bucket=1  pkt=[F P 0x01 0x03   A B]  g_frames=1` | `return 0  out_meta.version=1  out_meta.flags=3  out_meta.checksum=65  out_meta.peak_bucket=1  pkt=[F P 0x01 0x03   A B]  g_frames=1` | **EXACT** |
| truncated 2-byte frame | `pkt[2]=[0x46 0x50]` | `return -1  out_meta.version=0  out_meta.flags=0  out_meta.checksum=0  out_meta.peak_bucket=0  pkt=[F P]` | `return -1  out_meta.version=0  out_meta.flags=0  out_meta.checksum=0  out_meta.peak_bucket=0  pkt=[F P]` | **EXACT** |
| bad magic header | `pkt[4]=[0x00 0x50 0x01 0x00]` | `return -2  out_meta.version=0  out_meta.flags=0  out_meta.checksum=0  out_meta.peak_bucket=0  pkt=[0x00 P 0x01 0x00]` | `return -2  out_meta.version=0  out_meta.flags=0  out_meta.checksum=0  out_meta.peak_bucket=0  pkt=[0x00 P 0x01 0x00]` | **EXACT** |
| high byte 0xE0 overflows hist[4] | `pkt[5]=[0x46 0x50 0x01 0x00 0xe0]` | `undefined behavior — hist[7] past length 4` | `return -100  out_meta.version=1  out_meta.flags=0  out_meta.checksum=0  out_meta.peak_bucket=0  pkt=[F P 0x01 0x00 0xe0]` | **CONTAINED** |

---

## 4. Verified Safe Rust Core (`src/lib.rs`)

```rust
//! FerroProve safe crate. Indexes become slice gets that return -100.
#![forbid(unsafe_code)]
#![allow(unused_variables, unused_mut, unused_assignments, unused_parens, unused_macros, dead_code, unreachable_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]

macro_rules! at {
    ($buf:expr, $i:expr) => {
        match $buf.get($i as usize) {
            Some(v) => *v as i32,
            None => return -100,
        }
    };
}
macro_rules! put {
    ($buf:expr, $i:expr, $v:expr) => {{
        let __idx = ($i) as usize;
        let __val = $v;
        match $buf.get_mut(__idx) {
            Some(slot) => *slot = __val,
            None => return -100,
        }
    }};
}

pub const FRAME_OK: i32 = 0;
pub const ERR_SHORT: i32 = -1;
pub const ERR_MAGIC: i32 = -2;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameSummary {
    pub version: i32,
    pub flags: i32,
    pub checksum: i32,
    pub peak_bucket: i32,
}

pub struct State {
    pub g_frames: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_frames: 0,
        }
    }
}

pub fn fold_checksum(state: &mut State, mut acc: i32, mut byte_val: i32) -> i32 {
    return (((acc as i32).wrapping_mul(31) + byte_val) & 255);
}

pub fn inspect_frame(state: &mut State, pkt: & [u8], out_meta: &mut FrameSummary) -> i32 {
    let mut hist: [u8; 4] = [0u8; 4];
    let mut i: i32 = 0;
    let mut csum: i32 = 0;
    let mut b: i32 = 0;
    let mut bucket: i32 = 0;
    if (pkt.len() as i32) < 4 {
        return ERR_SHORT;
    }
    if (at!(pkt, 0) != 70) || (at!(pkt, 1) != 80) {
        return ERR_MAGIC;
    }
    out_meta.version = at!(pkt, 2);
    out_meta.flags = at!(pkt, 3);
    csum = 0;
    i = 4;
    while i < (pkt.len() as i32) {
        b = at!(pkt, i);
        csum = fold_checksum(state, csum, b);
        bucket = (b >> 5);
        put!(hist, bucket, ((at!(hist, bucket) + 1) as u8));
        i = (i as i32).wrapping_add(1);
    }
    out_meta.checksum = csum;
    out_meta.peak_bucket = (at!(hist, 0) + at!(hist, 1));
    state.g_frames = (state.g_frames + 1);
    return FRAME_OK;
}
```

## 5. Isolated C-ABI Drop-In Shim (`src/ffi.rs`)

```rust
// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{State, FrameSummary};

#[allow(dead_code)]
fn view<'a, T>(ptr: *const T, n: usize) -> Result<&'a [T], i32> {
    if n == 0 { return Ok(unsafe { std::slice::from_raw_parts(std::ptr::NonNull::<T>::dangling().as_ptr(), 0) }); }
    if ptr.is_null() { return Err(-1); }
    Ok(unsafe { std::slice::from_raw_parts(ptr, n) })
}
#[allow(dead_code)]
fn view_mut<'a, T>(ptr: *mut T, n: usize) -> Result<&'a mut [T], i32> {
    if n == 0 { return Ok(unsafe { std::slice::from_raw_parts_mut(std::ptr::NonNull::<T>::dangling().as_ptr(), 0) }); }
    if ptr.is_null() { return Err(-1); }
    Ok(unsafe { std::slice::from_raw_parts_mut(ptr, n) })
}

#[no_mangle]
pub extern "C" fn ferro_state_new() -> *mut State {
    Box::into_raw(Box::new(State::new()))
}
#[no_mangle]
pub extern "C" fn ferro_state_free(state: *mut State) {
    if !state.is_null() { unsafe { drop(Box::from_raw(state)); } }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fold_checksum_c(state: *mut State, acc: i32, byte_val: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::fold_checksum(state, acc, byte_val)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn inspect_frame_c(state: *mut State, pkt: *const u8, n: usize, out_meta: *mut FrameSummary) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let pkt = match view(pkt, n) { Ok(s) => s, Err(c) => return c };
    if out_meta.is_null() { return -1; }
    let out_meta = unsafe { &mut *out_meta };
    crate::inspect_frame(state, pkt, out_meta)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
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
```
