# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Network: TLV length decode
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `9d61120fa37c9c83` (39 lines)
- **Safe Rust Crate Digest (FNV-64):** `ca15a694e13f57ee` (68 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `2c9960975df71e6d`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (301 ms, 19698 B .rlib, sha256:d25f88e2db0a3961)

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

- **Leaf-to-Root Translation Order:** `u8_word -> read_be16 -> tlv_decode`
- **Mutable Globals Threaded into `State`:** `g_oversize` (L6)
- **Legacy Memory Hazards Eliminated (6):**
  - `L6`: mutable global g_oversize -> collapsed into `State` struct & threaded as `&mut State`
  - `L15`: unchecked index p[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L16`: unchecked index p[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L25`: unchecked index buf[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L33`: unchecked index dst[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L33`: unchecked index buf[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 4
- **Byte-Exact Functional Parity:** 3 / 3
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 1
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| tag 7, len 2 | `buf[5]=[0x07 0x00 0x02 0xaa 0xbb]  dst[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 7  written=2  buf=[0x07 0x00 0x02 0xaa 0xbb]  dst=[0xaa 0xbb 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 7  written=2  buf=[0x07 0x00 0x02 0xaa 0xbb]  dst=[0xaa 0xbb 0x00 0x00 0x00 0x00 0x00 0x00]` | **EXACT** |
| truncated header | `buf[2]=[0x01 0x02]  dst[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return -1  written=0  buf=[0x01 0x02]` | `return -1  written=0  buf=[0x01 0x02]` | **EXACT** |
| length exceeds dst | `buf[5]=[0x01 0x00 0x04 0x09 0x09]  dst[2]=[0x00 0x00]` | `return -3  written=0  buf=[0x01 0x00 0x04 0x09 0x09]  g_oversize=1` | `return -3  written=0  buf=[0x01 0x00 0x04 0x09 0x09]  g_oversize=1` | **EXACT** |
| length past datagram | `buf[3]=[0x07 0x00 0x04]  dst[8]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `undefined behavior — buf[3] past length 3` | `return -100  written=0  buf=[0x07 0x00 0x04]` | **CONTAINED** |

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
    pub g_oversize: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_oversize: 0,
        }
    }
}

pub fn u8_word(state: &mut State, mut hi: i32, mut lo: i32) -> i32 {
    return ((hi << 8) + lo);
}

pub fn read_be16(state: &mut State, p: & [u8], mut off: i32) -> i32 {
    let mut hi: i32 = 0;
    let mut lo: i32 = 0;
    hi = at!(p, off);
    lo = at!(p, (off as i32).wrapping_add(1));
    return u8_word(state, hi, lo);
}

pub fn tlv_decode(state: &mut State, buf: & [u8], dst: &mut [u8], written: &mut i32) -> i32 {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut tag: i32 = 0;
    if (buf.len() as i32) < 3 {
        return -1;
    }
    tag = at!(buf, 0);
    len = read_be16(state, buf, 1);
    if len > (dst.len() as i32) {
        state.g_oversize = (state.g_oversize + 1);
        return -3;
    }
    i = 0;
    while i < len {
        put!(dst, i, (at!(buf, (i as i32).wrapping_add(3)) as u8));
        i = (i as i32).wrapping_add(1);
    }
    *written = len;
    return tag;
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
pub extern "C" fn u8_word_c(state: *mut State, hi: i32, lo: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::u8_word(state, hi, lo)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn read_be16_c(state: *mut State, p: *const u8, n: usize, off: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let p = match view(p, n) { Ok(s) => s, Err(c) => return c };
    crate::read_be16(state, p, off)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn tlv_decode_c(state: *mut State, buf: *const u8, n: usize, dst: *mut u8, dstcap: usize, written: *mut i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let buf = match view(buf, n) { Ok(s) => s, Err(c) => return c };
    let dst = match view_mut(dst, dstcap) { Ok(s) => s, Err(c) => return c };
    if written.is_null() { return -1; }
    let written = unsafe { &mut *written };
    crate::tlv_decode(state, buf, dst, written)
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
int32_t u8_word_c(State *state, int32_t hi, int32_t lo);
int32_t read_be16_c(State *state, const uint8_t * p, size_t n, int32_t off);
int32_t tlv_decode_c(State *state, const uint8_t * buf, size_t n, uint8_t * dst, size_t dstcap, int32_t * written);
#endif
```
