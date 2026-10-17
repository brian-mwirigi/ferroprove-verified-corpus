# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** GitHub: jedisct1/libsodium@1.0.19:src/libsodium/sodium/utils.c
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `dcaa65c85b7fdfa5` (23 lines)
- **Safe Rust Crate Digest (FNV-64):** `5d3ed3b3e203e310` (34 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `4d501799e55abe7e`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (202 ms, 11800 B .rlib, sha256:8d6d58e2a48c6492)

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

- **Leaf-to-Root Translation Order:** `sodium_memcmp`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (2):**
  - `L20`: unchecked index b1[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L20`: unchecked index b2[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 6
- **Byte-Exact Functional Parity:** 4 / 4
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 2
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| ASCII "123456789" (len=9) | `b1_[9]=[0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39]  b2_[9]=[0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39]  len=9` | `return 0  b1_=[1 2 3 4 5 6 7 8 9]  b2_=[1 2 3 4 5 6 7 8 9]  b1=[1 2 3 4 5 6 7 8 9]  b2=[1 2 3 4 5 6 7 8 9]` | `return 0  b1_=[1 2 3 4 5 6 7 8 9]  b2_=[1 2 3 4 5 6 7 8 9]  b1=[1 2 3 4 5 6 7 8 9]  b2=[1 2 3 4 5 6 7 8 9]` | **EXACT** |
| ASCII "0571" (len=2) | `b1_[4]=[0x30 0x35 0x37 0x31]  b2_[4]=[0x30 0x35 0x37 0x31]  len=2` | `return 0  b1_=[0 5 7 1]  b2_=[0 5 7 1]  b1=[0 5 7 1]  b2=[0 5 7 1]` | `return 0  b1_=[0 5 7 1]  b2_=[0 5 7 1]  b1=[0 5 7 1]  b2=[0 5 7 1]` | **EXACT** |
| second buffer differs in byte 0 (len=2) | `b1_[4]=[0x30 0x35 0x37 0x31]  b2_[4]=[0x31 0x37 0x35 0x30]  len=2` | `return -1  b1_=[0 5 7 1]  b2_=[1 7 5 0]  b1=[0 5 7 1]  b2=[1 7 5 0]` | `return -1  b1_=[0 5 7 1]  b2_=[1 7 5 0]  b1=[0 5 7 1]  b2=[1 7 5 0]` | **EXACT** |
| empty buffer (len=0) | `b1_[0]=[]  b2_[0]=[]  len=0` | `return 0` | `return 0` | **EXACT** |
| len=4 on 2-byte buffer (over-read) | `b1_[2]=[0x31 0x32]  b2_[2]=[0x31 0x32]  len=4` | `undefined behavior — b1[2] past length 2` | `return -100  b1_=[1 2]  b2_=[1 2]  b1=[1 2]  b2=[1 2]` | **CONTAINED** |
| size_t length 1000000 on a 2-byte buffer | `b1_[2]=[0x31 0x32]  b2_[2]=[0x31 0x32]  len=1000000` | `undefined behavior — b1[2] past length 2` | `return -100  b1_=[1 2]  b2_=[1 2]  b1=[1 2]  b2=[1 2]` | **CONTAINED** |

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

pub fn sodium_memcmp(b1_: & [u8], b2_: & [u8], mut len: i32) -> i32 {
    let mut i: i32 = 0;
    let mut d: i32 = 0;
    i = 0;
    while i < len {
        d = (((d | (at!(b1_, i) ^ at!(b2_, i)))) & 255);
        i = (i as i32).wrapping_add(1);
    }
    return ((1 & ((((d as i32).wrapping_sub(1)) as u32).wrapping_shr((8) as u32) as i32)) as i32).wrapping_sub(1);
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
pub extern "C" fn sodium_memcmp_c(b1_: *const u8, b1_len: usize, b2_: *const u8, b2_len: usize, len: i32) -> i32 {
    let b1_ = match view(b1_, b1_len) { Ok(s) => s, Err(c) => return c };
    let b2_ = match view(b2_, b2_len) { Ok(s) => s, Err(c) => return c };
    crate::sodium_memcmp(b1_, b2_, len)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t sodium_memcmp_c(const uint8_t * b1_, size_t b1_len, const uint8_t * b2_, size_t b2_len, int32_t len);
#endif
```
