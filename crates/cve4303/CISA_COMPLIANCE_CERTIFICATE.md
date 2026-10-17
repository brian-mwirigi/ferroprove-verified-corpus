# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** GitHub CVE: cJSON v1.0.0 parse_hex4
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `9d262c6023bd6c89` (89 lines)
- **Safe Rust Crate Digest (FNV-64):** `13a1cf2e4e6cef84` (71 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `c8e5fb44ac59a9c6`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (273 ms, 9780 B .rlib, sha256:9fa4b589786be623)

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

- **Leaf-to-Root Translation Order:** `parse_hex4`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (24):**
  - `L9`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L11`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L13`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L15`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L17`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L19`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L30`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L32`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L34`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L36`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L38`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L40`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L50`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L52`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L54`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L56`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L58`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L60`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L70`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L72`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L74`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L76`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L78`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L80`: unchecked deref *str -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 4
- **Byte-Exact Functional Parity:** 3 / 3
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 1
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| "0571" -> 0x0571 (1393) | `str[4]=[0x30 0x35 0x37 0x31]` | `return 1393  str=[0 5 7 1]` | `return 1393  str=[0 5 7 1]` | **EXACT** |
| "BEEF" -> 0xBEEF (48879) | `str[4]=[0x42 0x45 0x45 0x46]` | `return 48879  str=[B E E F]` | `return 48879  str=[B E E F]` | **EXACT** |
| "zzzz" -> 0 (invalid) | `str[4]=[0x7a 0x7a 0x7a 0x7a]` | `return 0  str=[z z z z]` | `return 0  str=[z z z z]` | **EXACT** |
| CVE-2016-4303 truncated "0" | `str[1]=[0x30]` | `undefined behavior — str[1] past length 1` | `return -100  str=[0]` | **CONTAINED** |

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

pub fn parse_hex4(str: & [u8]) -> i32 {
    let mut __str_off: i32 = 0;
    let mut h: i32 = 0;
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    h = (h as i32).wrapping_shl((4) as u32);
    __str_off = (__str_off + 1);
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    h = (h as i32).wrapping_shl((4) as u32);
    __str_off = (__str_off + 1);
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    h = (h as i32).wrapping_shl((4) as u32);
    __str_off = (__str_off + 1);
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    return h;
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
pub extern "C" fn parse_hex4_c(str: *const u8, str_len: usize) -> i32 {
    let str = match view(str, str_len) { Ok(s) => s, Err(c) => return c };
    crate::parse_hex4(str)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t parse_hex4_c(const uint8_t * str, size_t str_len);
#endif
```
