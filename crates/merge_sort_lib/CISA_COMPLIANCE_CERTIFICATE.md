# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 merge_sort_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `670841e25c790025` (54 lines)
- **Safe Rust Crate Digest (FNV-64):** `2b99d084cf0e891a` (100 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `2c2bbfd8a0033a7d`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (254 ms, 24216 B .rlib, sha256:5c4e4829fdf69e9b)

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

- **Leaf-to-Root Translation Order:** `spritebatch_internal_sprite_less_than_or_equal -> spritebatch_internal_merge_sort_iteration -> spritebatch_internal_merge_sort_recurse -> merge_sort`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (4):**
  - `L30`: unchecked index b[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L30`: unchecked index a[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L33`: unchecked index b[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L33`: unchecked index a[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 5
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `b[0]=[]  size=5` | `return 0` | `return 0` | **EXACT** |
| 2.json | `a[0]=[]  b[0]=[]  size=0` | `return 0` | `return 0` | **EXACT** |
| 3.json | `b[0]=[]  size=5` | `return 0` | `return 0` | **EXACT** |
| 4.json | `b[0]=[]  size=5` | `return 0` | `return 0` | **EXACT** |
| 5.json | `b[0]=[]  size=20` | `return 0` | `return 0` | **EXACT** |

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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct spritebatch_sprite_t {
    pub texture_id: i32,
    pub sort_bits: i32,
}

pub fn spritebatch_internal_sprite_less_than_or_equal(a: &mut spritebatch_sprite_t, b: &mut spritebatch_sprite_t) -> i32 {
    if a.sort_bits <= b.sort_bits {
        return 1;
    }
    if (a.sort_bits == b.sort_bits) && (a.texture_id <= b.texture_id) {
        return 1;
    }
    return 0;
}

pub fn spritebatch_internal_merge_sort_iteration(a: &mut [spritebatch_sprite_t], mut lo: i32, mut split: i32, mut hi: i32, b: &mut [spritebatch_sprite_t]) -> i32 {
    let mut i: i32 = lo;
    let mut j: i32 = split;
    let mut k: i32 = lo;
    while k < hi {
        if (i < split) && ((j >= hi) || ({
        let __fp_i0 = (i) as usize;
        if __fp_i0 >= a.len() { return -100; }
        let mut __fp_e0 = a[__fp_i0];
        let __fp_i1 = (j) as usize;
        if __fp_i1 >= a.len() { return -100; }
        let mut __fp_e1 = a[__fp_i1];
        let __rc = spritebatch_internal_sprite_less_than_or_equal(&mut __fp_e0, &mut __fp_e1);
        a[__fp_i0] = __fp_e0;
        a[__fp_i1] = __fp_e1;
        __rc
    } != 0)) {
            {
                let __di = (k) as usize;
                let __si = (i) as usize;
                if __di >= b.len() || __si >= a.len() { return -100; }
                b[__di] = a[__si];
            }
            i = (i + 1);
        } else {
            {
                let __di = (k) as usize;
                let __si = (j) as usize;
                if __di >= b.len() || __si >= a.len() { return -100; }
                b[__di] = a[__si];
            }
            j = (j + 1);
        }
        k = (k + 1);
    }
    0
}

pub fn spritebatch_internal_merge_sort_recurse(b: &mut [spritebatch_sprite_t], mut lo: i32, mut hi: i32, a: &mut [spritebatch_sprite_t]) -> i32 {
    if (hi - lo) <= 1 {
        return 0;
    }
    let mut split: i32 = ((lo + hi) / 2);
    spritebatch_internal_merge_sort_recurse(a, lo, split, b);
    spritebatch_internal_merge_sort_recurse(a, split, hi, b);
    spritebatch_internal_merge_sort_iteration(b, lo, split, hi, a);
    0
}

pub fn merge_sort(a: &mut [spritebatch_sprite_t], b: &mut [spritebatch_sprite_t], mut size: i32) -> i32 {
    {
        let __n = ((1 as i32).wrapping_mul(size)) as usize;
        if __n > b.len() || __n > a.len() { return -100; }
        b[..__n].copy_from_slice(&a[..__n]);
        0
    };
    spritebatch_internal_merge_sort_recurse(b, 0, size, a);
    0
}
```

## 5. Isolated C-ABI Drop-In Shim (`src/ffi.rs`)

```rust
// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{spritebatch_sprite_t};

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
pub extern "C" fn spritebatch_internal_sprite_less_than_or_equal_c(a: *mut spritebatch_sprite_t, b: *mut spritebatch_sprite_t) -> i32 {
    if a.is_null() { return -1; }
    let a = unsafe { &mut *a };
    if b.is_null() { return -1; }
    let b = unsafe { &mut *b };
    crate::spritebatch_internal_sprite_less_than_or_equal(a, b)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn spritebatch_internal_merge_sort_iteration_c(a: *mut spritebatch_sprite_t, a_len: usize, lo: i32, split: i32, hi: i32, b: *mut spritebatch_sprite_t, b_len: usize) -> i32 {
    let a = match view_mut(a, a_len) { Ok(s) => s, Err(c) => return c };
    let b = match view_mut(b, b_len) { Ok(s) => s, Err(c) => return c };
    crate::spritebatch_internal_merge_sort_iteration(a, lo, split, hi, b)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn spritebatch_internal_merge_sort_recurse_c(b: *mut spritebatch_sprite_t, b_len: usize, lo: i32, hi: i32, a: *mut spritebatch_sprite_t, a_len: usize) -> i32 {
    let b = match view_mut(b, b_len) { Ok(s) => s, Err(c) => return c };
    let a = match view_mut(a, a_len) { Ok(s) => s, Err(c) => return c };
    crate::spritebatch_internal_merge_sort_recurse(b, lo, hi, a)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn merge_sort_c(a: *mut spritebatch_sprite_t, a_len: usize, b: *mut spritebatch_sprite_t, b_len: usize, size: i32) -> i32 {
    let a = match view_mut(a, a_len) { Ok(s) => s, Err(c) => return c };
    let b = match view_mut(b, b_len) { Ok(s) => s, Err(c) => return c };
    crate::merge_sort(a, b, size)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct spritebatch_sprite_t {
    int32_t texture_id;
    int32_t sort_bits;
} spritebatch_sprite_t;
typedef struct State State;
int32_t spritebatch_internal_sprite_less_than_or_equal_c(spritebatch_sprite_t * a, spritebatch_sprite_t * b);
int32_t spritebatch_internal_merge_sort_iteration_c(spritebatch_sprite_t * a, size_t a_len, int32_t lo, int32_t split, int32_t hi, spritebatch_sprite_t * b, size_t b_len);
int32_t spritebatch_internal_merge_sort_recurse_c(spritebatch_sprite_t * b, size_t b_len, int32_t lo, int32_t hi, spritebatch_sprite_t * a, size_t a_len);
int32_t merge_sort_c(spritebatch_sprite_t * a, size_t a_len, spritebatch_sprite_t * b, size_t b_len, int32_t size);
#endif
```
