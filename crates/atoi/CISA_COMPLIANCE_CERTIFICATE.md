# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** GitHub: esmil/musl@master:src/stdlib/atoi.c
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `cfe28fef24b26ad9` (17 lines)
- **Safe Rust Crate Digest (FNV-64):** `f2ad2327a5aabc58` (43 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `c5d42fd821a7f042`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (267 ms, 9868 B .rlib, sha256:ec3927919236dd76)

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

- **Leaf-to-Root Translation Order:** `atoi`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (4):**
  - `L7`: unchecked deref *s -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L8`: unchecked deref *s -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L13`: unchecked deref *s -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L14`: unchecked deref *s -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 4
- **Byte-Exact Functional Parity:** 2 / 2
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 2
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| NUL-terminated "0571\0" | `s[5]=[0x30 0x35 0x37 0x31 0x00]` | `return 571  s=[0 5 7 1 0x00]` | `return 571  s=[0 5 7 1 0x00]` | **EXACT** |
| NUL-terminated "-42\0" | `s[4]=[0x2d 0x34 0x32 0x00]` | `return -42  s=[- 4 2 0x00]` | `return -42  s=[- 4 2 0x00]` | **EXACT** |
| unterminated 1-byte "0" (missing NUL) | `s[1]=[0x30]` | `undefined behavior — s[1] past length 1` | `return -100  s=[0]` | **CONTAINED** |
| empty 0-byte buffer | `s[0]=[]` | `undefined behavior — s[0] past length 0` | `return -100` | **CONTAINED** |

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

pub fn atoi(s: & [u8]) -> i32 {
    let mut __s_off: i32 = 0;
    let mut n: i32 = 0;
    let mut neg: i32 = 0;
    while (at!(s, __s_off) == (b' ' as i32)) || ((at!(s, __s_off) >= (b'\t' as i32)) && (at!(s, __s_off) <= (b'\r' as i32))) {
        __s_off = (__s_off + 1);
    }
    if at!(s, __s_off) == (b'-' as i32) {
        neg = 1;
        __s_off = (__s_off + 1);
    } else if at!(s, __s_off) == (b'+' as i32) {
        __s_off = (__s_off + 1);
    }
    while (at!(s, __s_off) >= (b'0' as i32)) && (at!(s, __s_off) <= (b'9' as i32)) {
        n = ((10 as i32).wrapping_mul(n) - (at!(s, __s_off) - (b'0' as i32)));
        __s_off = (__s_off + 1);
    }
    return (if neg != 0 { n } else { (-n) });
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
pub extern "C" fn atoi_c(s: *const u8, s_len: usize) -> i32 {
    let s = match view(s, s_len) { Ok(s) => s, Err(c) => return c };
    crate::atoi(s)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t atoi_c(const uint8_t * s, size_t s_len);
#endif
```
