# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 stereo_samples_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `c1d57bf573fa62eb` (56 lines)
- **Safe Rust Crate Digest (FNV-64):** `8f7ed22511471c4e` (100 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `6b85612a2f4d70bc`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (271 ms, 14784 B .rlib, sha256:83b6e99206366231)

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

- **Leaf-to-Root Translation Order:** `btac1c2_fakesqrt -> stereo_samples`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (4):**
  - `L37`: unchecked index ibuf0[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L38`: unchecked index ibuf0[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L39`: unchecked index ibuf1[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L40`: unchecked index ibuf1[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 2
- **Byte-Exact Functional Parity:** 2 / 2
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `ibuf0[32]=[-246 24929 -1 25087 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -159 -1 -1 -1 -1 -1 -1 -1 24929 24929 24929 24929 24929]  ibuf1[32]=[24929 1285 24837 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -1 -1 -1 -1 -1 -1 -1 -1 -1 -9 -1 32767 -1 -1 -1 -1 -1537 -1 -1]  len=-1` | `return 0  ibuf0=[-246 24929 -1 25087 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -159 -1 -1 -1 -1 -1 -1 -1 24929 24929 24929 24929 24929]  ibuf1=[24929 1285 24837 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -1 -1 -1 -1 -1 -1 -1 -1 -1 -9 -1 32767 -1 -1 -1 -1 -1537 -1 -1]` | `return 0  ibuf0=[-246 24929 -1 25087 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -159 -1 -1 -1 -1 -1 -1 -1 24929 24929 24929 24929 24929]  ibuf1=[24929 1285 24837 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -1 -1 -1 -1 -1 -1 -1 -1 -1 -9 -1 32767 -1 -1 -1 -1 -1537 -1 -1]` | **EXACT** |
| 2.json | `ibuf0[32]=[-246 -1 25087 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929]  ibuf1[32]=[24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -159 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -129 -1 -1 -1 -1 -1 -1]  len=-1` | `return 0  ibuf0=[-246 -1 25087 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929]  ibuf1=[24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -159 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -129 -1 -1 -1 -1 -1 -1]` | `return 0  ibuf0=[-246 -1 25087 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929]  ibuf1=[24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 24929 -159 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -129 -1 -1 -1 -1 -1 -1]` | **EXACT** |

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

pub fn btac1c2_fakesqrt(mut val: i32) -> i32 {
    let mut v: i32 = 0;
    let mut v1: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if val < 0 {
        return -btac1c2_fakesqrt((-val));
    }
    v = val;
    while v > (1 << 30) {
        v = (v >> 1);
    }
    while (v as i32).wrapping_mul(v) > val {
        v = (v >> 1);
    }
    i = 1;
    while i < 17 {
        v1 = (v + (v >> i));
        j = 16;
        if v1 == v {
            break;
        }
        while ((v1 as i32).wrapping_mul(v1) <= val) && (j != 0) {
            j = (j - 1);
            v = v1;
            v1 = (v + (v >> i));
        }
        j = (j - 1);
        i = (i + 1);
    }
    return v;
}

pub fn stereo_samples(ibuf0: &mut [i16], ibuf1: &mut [i16], mut len: i32) -> i32 {
    len = (len % 16);
    let mut p0: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    let mut pc0: i32 = 0;
    let mut ps0: i32 = 0;
    let mut pc1: i32 = 0;
    let mut ps1: i32 = 0;
    let mut e: i32 = 0;
    let mut d: i32 = 0;
    let mut dc: i32 = 0;
    let mut ds: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    e = 0;
    p0 = 0;
    p1 = 0;
    i = 0;
    while i < len {
        p0 = at!(ibuf0, ((i as i32).wrapping_mul(2) + 0));
        p1 = at!(ibuf0, ((i as i32).wrapping_mul(2) + 1));
        p2 = at!(ibuf1, ((i as i32).wrapping_mul(2) + 0));
        p3 = at!(ibuf1, ((i as i32).wrapping_mul(2) + 1));
        pc0 = ((p0 + p1) >> 1);
        ps0 = (p0 - p1);
        pc1 = ((p2 + p3) >> 1);
        ps1 = (p2 - p3);
        dc = (pc0 - pc1);
        ds = (ps0 - ps1);
        ds = (ds >> 2);
        d = ((dc as i32).wrapping_mul(dc) + (ds as i32).wrapping_mul(ds));
        e = (e + d);
        i = (i + 1);
    }
    e = btac1c2_fakesqrt(e);
    if (e < 0) || (e > (1 << 30)) {
        e = (1 << 30);
    }
    return e;
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
pub extern "C" fn btac1c2_fakesqrt_c(val: i32) -> i32 {
    crate::btac1c2_fakesqrt(val)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn stereo_samples_c(ibuf0: *mut i16, ibuf0_len: usize, ibuf1: *mut i16, ibuf1_len: usize, len: i32) -> i32 {
    let ibuf0 = match view_mut(ibuf0, ibuf0_len) { Ok(s) => s, Err(c) => return c };
    let ibuf1 = match view_mut(ibuf1, ibuf1_len) { Ok(s) => s, Err(c) => return c };
    crate::stereo_samples(ibuf0, ibuf1, len)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t btac1c2_fakesqrt_c(int32_t val);
int32_t stereo_samples_c(int16_t * ibuf0, size_t ibuf0_len, int16_t * ibuf1, size_t ibuf1_len, int32_t len);
#endif
```
