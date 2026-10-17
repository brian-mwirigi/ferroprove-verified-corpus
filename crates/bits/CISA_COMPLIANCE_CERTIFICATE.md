# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Crypto: Bitstream unpack
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `144ea93f662c0164` (31 lines)
- **Safe Rust Crate Digest (FNV-64):** `ec3fde276296d8f5` (58 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `dee15bf9e3d4d146`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (348 ms, 17310 B .rlib, sha256:c9ea8bf02bdb69fc)

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

- **Leaf-to-Root Translation Order:** `bit_from_byte -> pull_bits`
- **Mutable Globals Threaded into `State`:** `g_bitpos` (L6)
- **Legacy Memory Hazards Eliminated (2):**
  - `L6`: mutable global g_bitpos -> collapsed into `State` struct & threaded as `&mut State`
  - `L23`: unchecked index buf[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 3
- **Byte-Exact Functional Parity:** 2 / 2
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 1
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| top 3 bits of 0xB0 | `buf[1]=[0xb0]  nbits=3` | `return 0  out=5  buf=[0xb0]  g_bitpos=3` | `return 0  out=5  buf=[0xb0]  g_bitpos=3` | **EXACT** |
| full byte 0xB0 | `buf[1]=[0xb0]  nbits=8` | `return 0  out=176  buf=[0xb0]  g_bitpos=8` | `return 0  out=176  buf=[0xb0]  g_bitpos=8` | **EXACT** |
| 9 bits from 1 byte | `buf[1]=[0xb0]  nbits=9` | `undefined behavior — buf[1] past length 1` | `return -100  out=0  buf=[0xb0]  g_bitpos=8` | **CONTAINED** |

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
    pub g_bitpos: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_bitpos: 0,
        }
    }
}

pub fn bit_from_byte(state: &mut State, mut b: i32, mut bit: i32) -> i32 {
    return ((b >> bit) & 1);
}

pub fn pull_bits(state: &mut State, buf: & [u8], mut nbits: i32, out: &mut i32) -> i32 {
    let mut acc: i32 = 0;
    let mut k: i32 = 0;
    let mut bit: i32 = 0;
    let mut byte_i: i32 = 0;
    let mut b: i32 = 0;
    acc = 0;
    k = 0;
    while k < nbits {
        byte_i = (state.g_bitpos / 8);
        bit = (7 - (state.g_bitpos % 8));
        b = at!(buf, byte_i);
        acc = ((acc << 1) + bit_from_byte(state, b, bit));
        state.g_bitpos = (state.g_bitpos + 1);
        k = (k + 1);
    }
    *out = acc;
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
pub extern "C" fn bit_from_byte_c(state: *mut State, b: i32, bit: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::bit_from_byte(state, b, bit)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn pull_bits_c(state: *mut State, buf: *const u8, nbytes: usize, nbits: i32, out: *mut i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let buf = match view(buf, nbytes) { Ok(s) => s, Err(c) => return c };
    if out.is_null() { return -1; }
    let out = unsafe { &mut *out };
    crate::pull_bits(state, buf, nbits, out)
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
int32_t bit_from_byte_c(State *state, int32_t b, int32_t bit);
int32_t pull_bits_c(State *state, const uint8_t * buf, size_t nbytes, int32_t nbits, int32_t * out);
#endif
```
