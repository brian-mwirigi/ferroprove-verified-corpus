# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 hdr_compare_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `e82a8b2e71ba14ed` (19 lines)
- **Safe Rust Crate Digest (FNV-64):** `12a05e12380df119` (31 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `91995625d8addb6b`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (260 ms, 15262 B .rlib, sha256:2f6af4e4ef5f0066)

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

- **Leaf-to-Root Translation Order:** `hdr_valid -> hdr_compare`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (9):**
  - `L9`: unchecked index h[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L10`: unchecked index h[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L11`: unchecked index h[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L15`: unchecked index h1[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L15`: unchecked index h2[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L16`: unchecked index h1[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L16`: unchecked index h2[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L17`: unchecked index h1[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L17`: unchecked index h2[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 8
- **Byte-Exact Functional Parity:** 8 / 8
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xfe 0xf2 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xfe 0xf2 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xfe 0xf2 0x10]` | **EXACT** |
| 2.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xa2 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xa2 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xa2 0x10]` | **EXACT** |
| 3.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xf0 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf0 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf0 0x10]` | **EXACT** |
| 4.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xf2 0xf0]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0xf0]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0xf0]` | **EXACT** |
| 5.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xf2 0x1c]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0x1c]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0x1c]` | **EXACT** |
| 6.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xf2 0x10]` | `return 1  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0x10]` | `return 1  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0x10]` | **EXACT** |
| 7.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xf4 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf4 0x10]` | `return 0  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf4 0x10]` | **EXACT** |
| 8.json | `h1[3]=[0xff 0xf2 0x10]  h2[3]=[0xff 0xf2 0x10]` | `return 1  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0x10]` | `return 1  h1=[0xff 0xf2 0x10]  h2=[0xff 0xf2 0x10]` | **EXACT** |

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

pub fn hdr_valid(h: & [u8]) -> i32 {
    return (if ((((at!(h, 0) == 255) && (((at!(h, 1) & 240) == 240) || ((at!(h, 1) & 254) == 226))) && (((at!(h, 1) >> 1) & 3) != 0)) && ((at!(h, 2) >> 4) != 15)) && (((at!(h, 2) >> 2) & 3) != 3) { 1 } else { 0 });
}

pub fn hdr_compare(h1: & [u8], h2: & [u8]) -> i32 {
    return (if (((hdr_valid(h2) != 0) && (((at!(h1, 1) ^ at!(h2, 1)) & 254) == 0)) && (((at!(h1, 2) ^ at!(h2, 2)) & 12) == 0)) && (!(((if (at!(h1, 2) & 240) == 0 { 1 } else { 0 }) ^ (if (at!(h2, 2) & 240) == 0 { 1 } else { 0 })) != 0)) { 1 } else { 0 });
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
pub extern "C" fn hdr_valid_c(h: *const u8, h_len: usize) -> i32 {
    let h = match view(h, h_len) { Ok(s) => s, Err(c) => return c };
    crate::hdr_valid(h)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn hdr_compare_c(h1: *const u8, h1_len: usize, h2: *const u8, h2_len: usize) -> i32 {
    let h1 = match view(h1, h1_len) { Ok(s) => s, Err(c) => return c };
    let h2 = match view(h2, h2_len) { Ok(s) => s, Err(c) => return c };
    crate::hdr_compare(h1, h2)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t hdr_valid_c(const uint8_t * h, size_t h_len);
int32_t hdr_compare_c(const uint8_t * h1, size_t h1_len, const uint8_t * h2, size_t h2_len);
#endif
```
