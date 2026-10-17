# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** GitHub: openbsd/src@master:lib/libc/string/timingsafe_bcmp.c
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `14051f711eb24a49` (31 lines)
- **Safe Rust Crate Digest (FNV-64):** `e4be322c245d587a` (36 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `7e82fa652134c4b6`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (238 ms, 11544 B .rlib, sha256:c0ab26e9c52a8231)

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

- **Leaf-to-Root Translation Order:** `timingsafe_bcmp`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (2):**
  - `L27`: unchecked deref *p1 -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L27`: unchecked deref *p2 -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 6
- **Byte-Exact Functional Parity:** 4 / 4
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 2
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| ASCII "123456789" (len=9) | `b1[9]=[0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39]  b2[9]=[0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39]  n=9` | `return 0  b1=[1 2 3 4 5 6 7 8 9]  b2=[1 2 3 4 5 6 7 8 9]  p1=[1 2 3 4 5 6 7 8 9]  p2=[1 2 3 4 5 6 7 8 9]` | `return 0  b1=[1 2 3 4 5 6 7 8 9]  b2=[1 2 3 4 5 6 7 8 9]  p1=[1 2 3 4 5 6 7 8 9]  p2=[1 2 3 4 5 6 7 8 9]` | **EXACT** |
| ASCII "0571" (len=2) | `b1[4]=[0x30 0x35 0x37 0x31]  b2[4]=[0x30 0x35 0x37 0x31]  n=2` | `return 0  b1=[0 5 7 1]  b2=[0 5 7 1]  p1=[0 5 7 1]  p2=[0 5 7 1]` | `return 0  b1=[0 5 7 1]  b2=[0 5 7 1]  p1=[0 5 7 1]  p2=[0 5 7 1]` | **EXACT** |
| second buffer differs in byte 0 (len=2) | `b1[4]=[0x30 0x35 0x37 0x31]  b2[4]=[0x31 0x37 0x35 0x30]  n=2` | `return 1  b1=[0 5 7 1]  b2=[1 7 5 0]  p1=[0 5 7 1]  p2=[1 7 5 0]` | `return 1  b1=[0 5 7 1]  b2=[1 7 5 0]  p1=[0 5 7 1]  p2=[1 7 5 0]` | **EXACT** |
| empty buffer (len=0) | `b1[0]=[]  b2[0]=[]  n=0` | `return 0` | `return 0` | **EXACT** |
| len=4 on 2-byte buffer (over-read) | `b1[2]=[0x31 0x32]  b2[2]=[0x31 0x32]  n=4` | `undefined behavior — p1[2] past length 2` | `return -100  b1=[1 2]  b2=[1 2]  p1=[1 2]  p2=[1 2]` | **CONTAINED** |
| size_t length 1000000 on a 2-byte buffer | `b1[2]=[0x31 0x32]  b2[2]=[0x31 0x32]  n=1000000` | `undefined behavior — p1[2] past length 2` | `return -100  b1=[1 2]  b2=[1 2]  p1=[1 2]  p2=[1 2]` | **CONTAINED** |

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

pub fn timingsafe_bcmp(b1: & [u8], b2: & [u8], mut n: i32) -> i32 {
    let mut __p1_off: i32 = 0;
    let mut __p2_off: i32 = 0;
    let mut ret: i32 = 0;
    while n > 0 {
        ret = (ret | (at!(b1, __p1_off) ^ at!(b2, __p2_off)));
        __p1_off = (__p1_off + 1);
        __p2_off = (__p2_off + 1);
        n = (n as i32).wrapping_sub(1);
    }
    return (if ret != 0 { 1 } else { 0 });
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
pub extern "C" fn timingsafe_bcmp_c(b1: *const u8, b1_len: usize, b2: *const u8, b2_len: usize, n: i32) -> i32 {
    let b1 = match view(b1, b1_len) { Ok(s) => s, Err(c) => return c };
    let b2 = match view(b2, b2_len) { Ok(s) => s, Err(c) => return c };
    crate::timingsafe_bcmp(b1, b2, n)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t timingsafe_bcmp_c(const uint8_t * b1, size_t b1_len, const uint8_t * b2, size_t b2_len, int32_t n);
#endif
```
