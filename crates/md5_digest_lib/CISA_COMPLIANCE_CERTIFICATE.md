# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 md5_digest_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `7331dabbaeccfcd4` (33 lines)
- **Safe Rust Crate Digest (FNV-64):** `7f947f6e45e5e5cd` (52 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `3f7da7a3ef29eb9b`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (218 ms, 15932 B .rlib, sha256:88ab9e3ec2805573)

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

- **Leaf-to-Root Translation Order:** `md5_digest`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (16):**
  - `L15`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L16`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L17`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L18`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L19`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L20`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L21`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L22`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L23`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L24`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L25`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L26`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L27`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L28`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L29`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L30`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 5
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `out[16]=[0x98 0x79 0xa9 0xc6 0x8c 0xe5 0x15 0x41 0x08 0x14 0xb4 0x26 0x00 0x40 0xf0 0xa2]` | `return 0  m.a=61539269  m.b=2369996940  m.c=2883785692  m.d=3215706183  out=[0xc5 0x03 0xab 0x03 0x8c H C 0x8d 0xdc 0x13 0xe3 0xab G 0xc8 0xab 0xbf]` | `return 0  m.a=61539269  m.b=2369996940  m.c=2883785692  m.d=3215706183  out=[0xc5 0x03 0xab 0x03 0x8c H C 0x8d 0xdc 0x13 0xe3 0xab G 0xc8 0xab 0xbf]` | **EXACT** |
| 2.json | `out[16]=[0x86 0x53 0xcf 0xd2 0xe8 0xc0 0x7e 0xb2 0xca 0xd8 0xb0 0x3f 0xc7 0x35 0x4a 0xe8]` | `return 0  m.a=274051987  m.b=2147196113  m.c=1360955111  m.d=1053418827  out=[0x93 0xb3 U 0x10 0xd1 0x9c 0xfb 0x7f 0xe7 0x86 0x1e Q K 0xe5 0xc9 >]` | `return 0  m.a=274051987  m.b=2147196113  m.c=1360955111  m.d=1053418827  out=[0x93 0xb3 U 0x10 0xd1 0x9c 0xfb 0x7f 0xe7 0x86 0x1e Q K 0xe5 0xc9 >]` | **EXACT** |
| 3.json | `out[16]=[0x9e 0x8a 0x38 0x78 0x04 0x04 0x27 0xc6 0x61 0x72 0xaf 0x7b 0xff 0x74 0xc0 0xec]` | `return 0  m.a=705554095  m.b=1101994371  m.c=4200502587  m.d=1880582026  out=[0xaf 0xe6 0x0d * 0x83 0x19 0xaf A ; 0x95 ^ 0xfa 0x8a g 0x17 p]` | `return 0  m.a=705554095  m.b=1101994371  m.c=4200502587  m.d=1880582026  out=[0xaf 0xe6 0x0d * 0x83 0x19 0xaf A ; 0x95 ^ 0xfa 0x8a g 0x17 p]` | **EXACT** |
| 4.json | `out[16]=[0x07 0xe0 0x0e 0xd8 0xe9 0x64 0xf4 0x97 0xb5 0xf2 0x0e 0x58 0x98 0x71 0xc2 0xea]` | `return 0  m.a=3377357174  m.b=3650895433  m.c=1053678897  m.d=1476143889  out=[v a N 0xc9 I > 0x9c 0xd9 1 0xdd 0xcd > 0x11 + 0xfc W]` | `return 0  m.a=3377357174  m.b=3650895433  m.c=1053678897  m.d=1476143889  out=[v a N 0xc9 I > 0x9c 0xd9 1 0xdd 0xcd > 0x11 + 0xfc W]` | **EXACT** |
| 5.json | `out[16]=[0x3f 0x45 0x41 0x1c 0x66 0xc1 0x9a 0x22 0xfb 0xc2 0x7b 0x2d 0xaf 0xb1 0xe1 0x92]` | `return 0  m.a=4235143695  m.b=2850509030  m.c=2230942739  m.d=3461974248  out=[0x0f * o 0xfc 0xe6 P 0xe7 0xa9 0x13 | 0xf9 0x84 0xe8 0x88 Y 0xce]` | `return 0  m.a=4235143695  m.b=2850509030  m.c=2230942739  m.d=3461974248  out=[0x0f * o 0xfc 0xe6 P 0xe7 0xa9 0x13 | 0xf9 0x84 0xe8 0x88 Y 0xce]` | **EXACT** |

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
pub struct tflac_md5 {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

pub fn md5_digest(m: & tflac_md5, out: &mut [u8]) -> i32 {
    put!(out, 0, (m.a as u8));
    put!(out, 1, ((m.a >> 8) as u8));
    put!(out, 2, ((m.a >> 16) as u8));
    put!(out, 3, ((m.a >> 24) as u8));
    put!(out, 4, (m.b as u8));
    put!(out, 5, ((m.b >> 8) as u8));
    put!(out, 6, ((m.b >> 16) as u8));
    put!(out, 7, ((m.b >> 24) as u8));
    put!(out, 8, (m.c as u8));
    put!(out, 9, ((m.c >> 8) as u8));
    put!(out, 10, ((m.c >> 16) as u8));
    put!(out, 11, ((m.c >> 24) as u8));
    put!(out, 12, (m.d as u8));
    put!(out, 13, ((m.d >> 8) as u8));
    put!(out, 14, ((m.d >> 16) as u8));
    put!(out, 15, ((m.d >> 24) as u8));
    0
}
```

## 5. Isolated C-ABI Drop-In Shim (`src/ffi.rs`)

```rust
// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{tflac_md5};

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
pub extern "C" fn md5_digest_c(m: *const tflac_md5, out: *mut u8, out_len: usize) -> i32 {
    if m.is_null() { return -1; }
    let m = unsafe { &*m };
    let out = match view_mut(out, out_len) { Ok(s) => s, Err(c) => return c };
    crate::md5_digest(m, out)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct tflac_md5 {
    int32_t a;
    int32_t b;
    int32_t c;
    int32_t d;
} tflac_md5;
typedef struct State State;
int32_t md5_digest_c(const tflac_md5 * m, uint8_t * out, size_t out_len);
#endif
```
