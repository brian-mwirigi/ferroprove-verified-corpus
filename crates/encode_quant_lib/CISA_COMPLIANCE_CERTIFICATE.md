# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 encode_quant_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `0a846e9b198ddfa5` (66 lines)
- **Safe Rust Crate Digest (FNV-64):** `8f31a5c4033b5b7b` (100 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `52b8eed5e4b49721`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (210 ms, 10824 B .rlib, sha256:aaef30489dd3087b)

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

- **Leaf-to-Root Translation Order:** `encode_quant`
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
| 1.json | `uni=7  step=10  pred=100  tgt=50  tgt2=60  lsbit=0` | `return 6` | `return 6` | **EXACT** |
| 2.json | `uni=5  step=10  pred=100  tgt=50  tgt2=60  lsbit=0` | `return 4` | `return 4` | **EXACT** |
| 3.json | `uni=8  step=10  pred=100  tgt=50  tgt2=60  lsbit=0` | `return 9` | `return 9` | **EXACT** |
| 4.json | `uni=6  step=10  pred=100  tgt=50  tgt2=60  lsbit=0` | `return 5` | `return 5` | **EXACT** |
| 5.json | `uni=10  step=15  pred=200  tgt=180  tgt2=190  lsbit=0` | `return 11` | `return 11` | **EXACT** |
| 6.json | `uni=10  step=15  pred=200  tgt=180  tgt2=190  lsbit=4` | `return 10` | `return 10` | **EXACT** |
| 7.json | `uni=10  step=15  pred=200  tgt=180  tgt2=190  lsbit=1` | `return 11` | `return 11` | **EXACT** |
| 8.json | `uni=10  step=15  pred=200  tgt=180  tgt2=190  lsbit=2` | `return 10` | `return 10` | **EXACT** |

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

pub fn encode_quant(mut uni: i32, mut step: i32, mut pred: i32, mut tgt: i32, mut tgt2: i32, mut lsbit: i32) -> i32 {
    let mut uni1: i32 = 0;
    let mut uni2: i32 = 0;
    let mut diff: i32 = 0;
    let mut p0: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    let mut d0: i32 = 0;
    let mut d1: i32 = 0;
    let mut d2: i32 = 0;
    let mut d3: i32 = 0;
    uni1 = (uni + 1);
    uni2 = (uni - 1);
    if ((uni ^ uni1) & (!7)) != 0 {
        uni1 = uni;
    }
    if ((uni ^ uni2) & (!7)) != 0 {
        uni2 = uni;
    }
    if lsbit != 0 {
        if lsbit == 4 {
            uni = (uni & (!1));
            uni1 = (uni1 & (!1));
            uni2 = (uni2 & (!1));
            uni = (uni | (((uni >> 1) & (uni >> 2)) & 1));
            uni1 = (uni1 | (((uni1 >> 1) & (uni1 >> 2)) & 1));
            uni2 = (uni2 | (((uni2 >> 1) & (uni2 >> 2)) & 1));
        } else if (lsbit & 1) != 0 {
            uni = (uni | 1);
            uni1 = (uni1 | 1);
            uni2 = (uni2 | 1);
        } else {
            uni = (uni & (!1));
            uni1 = (uni1 & (!1));
            uni2 = (uni2 & (!1));
        }
    }
    diff = ((((2 as i32).wrapping_mul((uni & 7)) + 1) as i32).wrapping_mul(step) / 8);
    if (uni & 8) != 0 {
        diff = -diff;
    }
    p0 = (pred + diff);
    d0 = (tgt - p0);
    d0 = (d0 ^ (d0 >> 31));
    diff = ((((2 as i32).wrapping_mul((uni1 & 7)) + 1) as i32).wrapping_mul(step) / 8);
    if (uni1 & 8) != 0 {
        diff = -diff;
    }
    p1 = (pred + diff);
    d1 = (tgt - p1);
    d1 = (d1 ^ (d1 >> 31));
    diff = ((((2 as i32).wrapping_mul((uni2 & 7)) + 1) as i32).wrapping_mul(step) / 8);
    if (uni2 & 8) != 0 {
        diff = -diff;
    }
    p2 = (pred + diff);
    d2 = (tgt - p2);
    d2 = (d2 ^ (d2 >> 31));
    d3 = (tgt2 - p0);
    d3 = (d3 ^ (d3 >> 31));
    d0 = (d0 + (d3 >> 5));
    d3 = (tgt2 - p1);
    d3 = (d3 ^ (d3 >> 31));
    d1 = (d1 + (d3 >> 5));
    d3 = (tgt2 - p2);
    d3 = (d3 ^ (d3 >> 31));
    d2 = (d2 + (d3 >> 5));
    if d1 < d0 {
        uni = uni1;
    }
    if d2 < d0 {
        uni = uni2;
    }
    return uni;
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
pub extern "C" fn encode_quant_c(uni: i32, step: i32, pred: i32, tgt: i32, tgt2: i32, lsbit: i32) -> i32 {
    crate::encode_quant(uni, step, pred, tgt, tgt2, lsbit)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t encode_quant_c(int32_t uni, int32_t step, int32_t pred, int32_t tgt, int32_t tgt2, int32_t lsbit);
#endif
```
