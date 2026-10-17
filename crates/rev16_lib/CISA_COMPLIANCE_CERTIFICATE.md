# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 rev16_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `7e1c80323d245977` (15 lines)
- **Safe Rust Crate Digest (FNV-64):** `ed333c964971f730` (31 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `90151791e86ec9ca`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (193 ms, 10618 B .rlib, sha256:45d6da0f0bf843bb)

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

- **Leaf-to-Root Translation Order:** `rev16`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (0):**

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 5
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `a=580244498` | `return 18475` | `return 18475` | **EXACT** |
| 2.json | `a=1089851626` | `return 22283` | `return 22283` | **EXACT** |
| 3.json | `a=1015771191` | `return 60430` | `return 60430` | **EXACT** |
| 4.json | `a=2163831009` | `return 34574` | `return 34574` | **EXACT** |
| 5.json | `a=3758842904` | `return 6182` | `return 6182` | **EXACT** |

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

pub fn rev16(mut a: i32) -> i32 {
    a = (((((a & 43690)) as u32).wrapping_shr((1) as u32) as i32) | ((a & 21845) as i32).wrapping_shl((1) as u32));
    a = (((((a & 52428)) as u32).wrapping_shr((2) as u32) as i32) | ((a & 13107) as i32).wrapping_shl((2) as u32));
    a = (((((a & 61680)) as u32).wrapping_shr((4) as u32) as i32) | ((a & 3855) as i32).wrapping_shl((4) as u32));
    a = (((((a & 65280)) as u32).wrapping_shr((8) as u32) as i32) | ((a & 255) as i32).wrapping_shl((8) as u32));
    return a;
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
pub extern "C" fn rev16_c(a: i32) -> i32 {
    crate::rev16(a)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t rev16_c(int32_t a);
#endif
```
