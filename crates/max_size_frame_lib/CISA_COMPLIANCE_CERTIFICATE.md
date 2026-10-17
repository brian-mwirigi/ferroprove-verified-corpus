# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 max_size_frame_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `17e02b8cafb4fdb6` (12 lines)
- **Safe Rust Crate Digest (FNV-64):** `5a9b14e4262ddc39` (27 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `db6adbdddaa199ca`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (193 ms, 10082 B .rlib, sha256:10d0f36e3f42ba91)

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

- **Leaf-to-Root Translation Order:** `max_size_frame`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (0):**

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 8
- **Byte-Exact Functional Parity:** 8 / 8
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `blocksize=1024  channels=2  bitdepth=16` | `return 4244` | `return 4244` | **EXACT** |
| 2.json | `blocksize=512  channels=2  bitdepth=24` | `return 3156` | `return 3156` | **EXACT** |
| 3.json | `blocksize=2048  channels=2  bitdepth=8` | `return 4372` | `return 4372` | **EXACT** |
| 4.json | `blocksize=4096  channels=2  bitdepth=32` | `return 32788` | `return 32788` | **EXACT** |
| 5.json | `blocksize=256  channels=2  bitdepth=20` | `return 1332` | `return 1332` | **EXACT** |
| 6.json | `blocksize=1024  channels=1  bitdepth=16` | `return 2067` | `return 2067` | **EXACT** |
| 7.json | `blocksize=1024  channels=3  bitdepth=16` | `return 6165` | `return 6165` | **EXACT** |
| 8.json | `blocksize=1024  channels=4  bitdepth=16` | `return 8214` | `return 8214` | **EXACT** |

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

pub fn max_size_frame(mut blocksize: i32, mut channels: i32, mut bitdepth: i32) -> i32 {
    return ((18 as i32).wrapping_add(channels) as i32).wrapping_add(((((((blocksize as i32).wrapping_mul(bitdepth) as i32).wrapping_mul((channels as i32).wrapping_mul((if channels != 2 { 1 } else { 0 }))) as i32).wrapping_add(((blocksize as i32).wrapping_mul(bitdepth) as i32).wrapping_mul((if channels == 2 { 1 } else { 0 }))) as i32).wrapping_add(((blocksize as i32).wrapping_mul((bitdepth as i32).wrapping_add((if bitdepth != 32 { 1 } else { 0 }))) as i32).wrapping_mul((if channels == 2 { 1 } else { 0 }))) as i32).wrapping_add(7) / 8));
}
```

## 5. Isolated C-ABI Drop-In Shim (`src/ffi.rs`)

```rust
// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.

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
pub extern "C" fn max_size_frame_c(blocksize: i32, channels: i32, bitdepth: i32) -> i32 {
    crate::max_size_frame(blocksize, channels, bitdepth)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t max_size_frame_c(int32_t blocksize, int32_t channels, int32_t bitdepth);
#endif
```
