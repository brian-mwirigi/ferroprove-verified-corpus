# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Parser: cJSON string scan
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `c7415c7d5f36f612` (75 lines)
- **Safe Rust Crate Digest (FNV-64):** `ce9454029f811800` (120 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `c078060ed3180c34`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (464 ms, 21186 B .rlib, sha256:23d3921a40c28312)

---

## 1. Static Memory-Safety Gate (`src/lib.rs`)

| Check | Requirement | Measured | Status |
| :--- | :--- | :--- | :--- |
| Crate-Level Directive | `#![forbid(unsafe_code)]` present | `true` | PASS |
| `unsafe` Blocks / Fn | `0` in safe core | `0` | PASS |
| Raw Pointer Derefs (`*mut` / `*const`) | `0` in safe core | `0` | PASS |
| Incomplete Stubs (`todo!()` / `unimplemented!()`) | `0` in safe core | `0` | PASS |
| Ambient Mutable Globals | Isolated into explicit `State` struct | `1 -> &mut State` | PASS |

---

## 2. Topological Call-Graph & Hazard Inventory

- **Leaf-to-Root Translation Order:** `hex_digit -> hex_byte -> scan_string`
- **Mutable Globals Threaded into `State`:** `g_trunc` (L6)
- **Legacy Memory Hazards Eliminated (8):**
  - `L6`: mutable global g_trunc -> collapsed into `State` struct & threaded as `&mut State`
  - `L33`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L35`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L36`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L38`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L52`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L67`: unchecked index out[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L71`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 6
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 1
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| "hi" | `src[4]=[0x22 0x68 0x69 0x22]  out[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  written=2  src=[" h i "]  out=[h i 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  written=2  src=[" h i "]  out=[h i 0x00 0x00 0x00 0x00 0x00 0x00]` | **EXACT** |
| quote a \\ n quote | `src[5]=[0x22 0x61 0x5c 0x6e 0x22]  out[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  written=2  src=[" a \ n "]  out=[a 0x0a 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  written=2  src=[" a \ n "]  out=[a 0x0a 0x00 0x00 0x00 0x00 0x00 0x00]` | **EXACT** |
| \\x41 | `src[6]=[0x22 0x5c 0x78 0x34 0x31 0x22]  out[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  written=1  src=[" \ x 4 1 "]  out=[A 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  written=1  src=[" \ x 4 1 "]  out=[A 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | **EXACT** |
| bad escape \\q | `src[4]=[0x22 0x5c 0x71 0x22]  out[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return -2  written=0  src=[" \ q "]  g_trunc=1` | `return -2  written=0  src=[" \ q "]  g_trunc=1` | **EXACT** |
| empty input | `src[0]=[]  out[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return -1  written=0` | `return -1  written=0` | **EXACT** |
| trailing backslash | `src[2]=[0x22 0x5c]  out[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `undefined behavior — src[2] past length 2` | `return -100  written=0  src=[" \]` | **CONTAINED** |

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

pub struct State {
    pub g_trunc: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_trunc: 0,
        }
    }
}

pub fn hex_digit(state: &mut State, mut c: i32) -> i32 {
    if (c >= (b'0' as i32)) && (c <= (b'9' as i32)) {
        return (c - (b'0' as i32));
    }
    if (c >= (b'a' as i32)) && (c <= (b'f' as i32)) {
        return ((c - (b'a' as i32)) + 10);
    }
    if (c >= (b'A' as i32)) && (c <= (b'F' as i32)) {
        return ((c - (b'A' as i32)) + 10);
    }
    return -1;
}

pub fn hex_byte(state: &mut State, mut hi: i32, mut lo: i32) -> i32 {
    let mut h1: i32 = 0;
    let mut h2: i32 = 0;
    h1 = hex_digit(state, hi);
    h2 = hex_digit(state, lo);
    if (h1 < 0) || (h2 < 0) {
        return -1;
    }
    return ((h1 as i32).wrapping_mul(16) + h2);
}

pub fn scan_string(state: &mut State, src: & [u8], out: &mut [u8], written: &mut i32) -> i32 {
    let mut i: i32 = 0;
    let mut o: i32 = 0;
    let mut c: i32 = 0;
    let mut e: i32 = 0;
    let mut hb: i32 = 0;
    i = 0;
    o = 0;
    if (src.len() as i32) == 0 {
        return -1;
    }
    if at!(src, 0) != (b'"' as i32) {
        return -1;
    }
    i = 1;
    while (i < (src.len() as i32)) && (at!(src, i) != (b'"' as i32)) {
        c = at!(src, i);
        if c == 92 {
            e = at!(src, (i as i32).wrapping_add(1));
            if e == (b'n' as i32) {
                c = 10;
                i = (i as i32).wrapping_add(2);
            } else if e == (b't' as i32) {
                c = 9;
                i = (i as i32).wrapping_add(2);
            } else if e == 92 {
                c = e;
                i = (i as i32).wrapping_add(2);
            } else if e == (b'"' as i32) {
                c = e;
                i = (i as i32).wrapping_add(2);
            } else if e == (b'x' as i32) {
                hb = hex_byte(state, at!(src, (i as i32).wrapping_add(2)), at!(src, (i as i32).wrapping_add(3)));
                if hb < 0 {
                    state.g_trunc = (state.g_trunc + 1);
                    return -2;
                }
                c = hb;
                i = (i as i32).wrapping_add(4);
            } else {
                state.g_trunc = (state.g_trunc + 1);
                return -2;
            }
        } else {
            i = (i as i32).wrapping_add(1);
        }
        if o >= (out.len() as i32) {
            return -3;
        }
        put!(out, o, (c as u8));
        o = (o as i32).wrapping_add(1);
    }
    if i >= (src.len() as i32) {
        return -4;
    }
    if at!(src, i) != (b'"' as i32) {
        return -4;
    }
    *written = o;
    return 0;
}
```

## 5. Isolated C-ABI Drop-In Shim (`src/ffi.rs`)

```rust
// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{State};

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
pub extern "C" fn ferro_state_new() -> *mut State {
    Box::into_raw(Box::new(State::new()))
}
#[no_mangle]
pub extern "C" fn ferro_state_free(state: *mut State) {
    if !state.is_null() { unsafe { drop(Box::from_raw(state)); } }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn hex_digit_c(state: *mut State, c: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::hex_digit(state, c)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn hex_byte_c(state: *mut State, hi: i32, lo: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::hex_byte(state, hi, lo)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn scan_string_c(state: *mut State, src: *const u8, n: usize, out: *mut u8, outcap: usize, written: *mut i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let src = match view(src, n) { Ok(s) => s, Err(c) => return c };
    let out = match view_mut(out, outcap) { Ok(s) => s, Err(c) => return c };
    if written.is_null() { return -1; }
    let written = unsafe { &mut *written };
    crate::scan_string(state, src, out, written)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
State *ferro_state_new(void);
void ferro_state_free(State *state);
int32_t hex_digit_c(State *state, int32_t c);
int32_t hex_byte_c(State *state, int32_t hi, int32_t lo);
int32_t scan_string_c(State *state, const uint8_t * src, size_t n, uint8_t * out, size_t outcap, int32_t * written);
#endif
```
