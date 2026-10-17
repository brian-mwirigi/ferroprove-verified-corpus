# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 flac_validate_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `971efae4f7914712` (76 lines)
- **Safe Rust Crate Digest (FNV-64):** `d31954c03f36a03b` (101 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `75aefb43e2b3bf1c`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (237 ms, 22200 B .rlib, sha256:fe790c4fb4644594)

---

## 1. Static Memory-Safety Gate (`src/lib.rs`)

| Check | Requirement | Measured | Status |
| :--- | :--- | :--- | :--- |
| Crate-Level Directive | `#![forbid(unsafe_code)]` present | `true` | PASS |
| `unsafe` Blocks / Fn | `0` in safe core | `0` | PASS |
| Raw Pointer Derefs (`*mut` / `*const`) | `0` in safe core | `0` | PASS |
| Incomplete Stubs (`todo!()` / `unimplemented!()`) | `0` in safe core | `0` | PASS |
| Ambient Mutable Globals | Isolated into explicit `State` struct | `0 -> &mut State` | PASS |

---

## 2. Topological Call-Graph & Hazard Inventory

- **Leaf-to-Root Translation Order:** `tflac_size_memory -> flac_validate`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (0):**

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 6
- **Byte-Exact Functional Parity:** 6 / 6
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `(no args)` | `return -1  t.blocksize=0  t.samplerate=0  t.channels=0  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | `return -1  t.blocksize=0  t.samplerate=0  t.channels=0  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | **EXACT** |
| 2.json | `(no args)` | `return -1  t.blocksize=16  t.samplerate=0  t.channels=0  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | `return -1  t.blocksize=16  t.samplerate=0  t.channels=0  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | **EXACT** |
| 3.json | `(no args)` | `return -1  t.blocksize=16  t.samplerate=1  t.channels=0  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | `return -1  t.blocksize=16  t.samplerate=1  t.channels=0  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | **EXACT** |
| 4.json | `(no args)` | `return -1  t.blocksize=16  t.samplerate=1  t.channels=1  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | `return -1  t.blocksize=16  t.samplerate=1  t.channels=1  t.bitdepth=0  t.channel_mode=0  t.max_rice_value=0  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=0` | **EXACT** |
| 5.json | `(no args)` | `return 0  t.blocksize=16  t.samplerate=1  t.channels=1  t.bitdepth=1  t.channel_mode=0  t.max_rice_value=14  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=16` | `return 0  t.blocksize=16  t.samplerate=1  t.channels=1  t.bitdepth=1  t.channel_mode=0  t.max_rice_value=14  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=16` | **EXACT** |
| 6.json | `(no args)` | `return 0  t.blocksize=16  t.samplerate=1  t.channels=1  t.bitdepth=1  t.channel_mode=0  t.max_rice_value=1  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=16` | `return 0  t.blocksize=16  t.samplerate=1  t.channels=1  t.bitdepth=1  t.channel_mode=0  t.max_rice_value=1  t.min_partition_order=0  t.max_partition_order=0  t.partition_order=0  t.cur_blocksize=16` | **EXACT** |

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

pub const TFLAC_CHANNEL_INDEPENDENT: i32 = 0;
pub const TFLAC_CHANNEL_LEFT_SIDE: i32 = 1;
pub const TFLAC_CHANNEL_SIDE_RIGHT: i32 = 2;
pub const TFLAC_CHANNEL_MID_SIDE: i32 = 3;
pub const TFLAC_CHANNEL_MODE_COUNT: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct tflac {
    pub blocksize: i32,
    pub samplerate: i32,
    pub channels: i32,
    pub bitdepth: i32,
    pub channel_mode: i32,
    pub max_rice_value: i32,
    pub min_partition_order: i32,
    pub max_partition_order: i32,
    pub partition_order: i32,
    pub cur_blocksize: i32,
}

pub fn tflac_size_memory(mut blocksize: i32) -> i32 {
    return (15 as i32).wrapping_add((5 as i32).wrapping_mul(((15 as i32).wrapping_add((blocksize as i32).wrapping_mul(4)) & -16)));
}

pub fn flac_validate(t: &mut tflac) -> i32 {
    if t.blocksize < 16 {
        return -1;
    }
    if t.blocksize > 65535 {
        return -1;
    }
    if t.samplerate == 0 {
        return -1;
    }
    if t.samplerate > 655350 {
        return -1;
    }
    if t.channels == 0 {
        return -1;
    }
    if t.channels > 8 {
        return -1;
    }
    if t.bitdepth == 0 {
        return -1;
    }
    if t.bitdepth > 32 {
        return -1;
    }
    if t.channel_mode != TFLAC_CHANNEL_INDEPENDENT {
        if (t.channels != 2) || (t.bitdepth == 32) {
            t.channel_mode = TFLAC_CHANNEL_INDEPENDENT;
        }
    }
    if t.max_rice_value == 0 {
        if t.bitdepth <= 16 {
            t.max_rice_value = 14;
        } else {
            t.max_rice_value = 30;
        }
    } else if t.max_rice_value > 30 {
        return -1;
    }
    if t.max_partition_order > 15 {
        return -1;
    }
    if t.min_partition_order > t.max_partition_order {
        return -1;
    }
    t.partition_order = t.min_partition_order;
    while ((t.blocksize % (1 << (t.partition_order + 1))) == 0) && (t.partition_order < t.max_partition_order) {
        t.partition_order = (t.partition_order + 1);
    }
    t.cur_blocksize = t.blocksize;
    return 0;
}
```

## 5. Isolated C-ABI Drop-In Shim (`src/ffi.rs`)

```rust
// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{tflac};

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
#[allow(non_snake_case)]
pub extern "C" fn tflac_size_memory_c(blocksize: i32) -> i32 {
    crate::tflac_size_memory(blocksize)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn flac_validate_c(t: *mut tflac) -> i32 {
    if t.is_null() { return -1; }
    let t = unsafe { &mut *t };
    crate::flac_validate(t)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
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
```
