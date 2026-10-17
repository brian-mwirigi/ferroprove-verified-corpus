# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** State: expr_eval globals
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `58a90d25de8ae4ab` (59 lines)
- **Safe Rust Crate Digest (FNV-64):** `172cd412bb3f959c` (93 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `f31df89f55448297`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (320 ms, 17344 B .rlib, sha256:04139604f7445ea2)

---

## 1. Static Memory-Safety Gate (`src/lib.rs`)

| Check | Requirement | Measured | Status |
| :--- | :--- | :--- | :--- |
| Crate-Level Directive | `#![forbid(unsafe_code)]` present | `true` | PASS |
| `unsafe` Blocks / Fn | `0` in safe core | `0` | PASS |
| Raw Pointer Derefs (`*mut` / `*const`) | `0` in safe core | `0` | PASS |
| Incomplete Stubs (`todo!()` / `unimplemented!()`) | `0` in safe core | `0` | PASS |
| Ambient Mutable Globals | Isolated into explicit `State` struct | `2 -> &mut State` | PASS |

---

## 2. Topological Call-Graph & Hazard Inventory

- **Leaf-to-Root Translation Order:** `commit -> eval_expr`
- **Mutable Globals Threaded into `State`:** `g_acc` (L6), `g_err` (L7)
- **Legacy Memory Hazards Eliminated (8):**
  - `L6`: mutable global g_acc -> collapsed into `State` struct & threaded as `&mut State`
  - `L7`: mutable global g_err -> collapsed into `State` struct & threaded as `&mut State`
  - `L29`: unchecked index s[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L33`: unchecked index s[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L34`: unchecked index s[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L44`: unchecked index s[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L45`: unchecked index s[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L53`: unchecked index s[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 6
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 1
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 12+3 | `s[4]=[0x31 0x32 0x2b 0x33]` | `return 0  out=15  s=[1 2 + 3]  g_acc=15` | `return 0  out=15  s=[1 2 + 3]  g_acc=15` | **EXACT** |
| 2*4 | `s[3]=[0x32 0x2a 0x34]` | `return 0  out=8  s=[2 * 4]  g_acc=8` | `return 0  out=8  s=[2 * 4]  g_acc=8` | **EXACT** |
| 3+4*2 left-assoc | `s[5]=[0x33 0x2b 0x34 0x2a 0x32]` | `return 0  out=14  s=[3 + 4 * 2]  g_acc=14` | `return 0  out=14  s=[3 + 4 * 2]  g_acc=14` | **EXACT** |
| empty | `s[0]=[]` | `return -1  out=0  g_err=1` | `return -1  out=0  g_err=1` | **EXACT** |
| x | `s[1]=[0x78]` | `return -2  out=0  s=[x]  g_err=1` | `return -2  out=0  s=[x]  g_err=1` | **EXACT** |
| 12+ | `s[3]=[0x31 0x32 0x2b]` | `undefined behavior — s[3] past length 3` | `return -100  out=0  s=[1 2 +]  g_err=1` | **CONTAINED** |

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
    pub g_acc: i32,
    pub g_err: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_acc: 0,
            g_err: 0,
        }
    }
}

pub fn commit(state: &mut State, mut v: i32) -> i32 {
    state.g_acc = v;
    return state.g_acc;
}

pub fn eval_expr(state: &mut State, s: & [u8], out: &mut i32) -> i32 {
    let mut i: i32 = 0;
    let mut acc: i32 = 0;
    let mut op: i32 = 0;
    let mut v: i32 = 0;
    i = 0;
    acc = 0;
    op = 0;
    state.g_err = 0;
    if (s.len() as i32) == 0 {
        state.g_err = (state.g_err + 1);
        return -1;
    }
    while i < (s.len() as i32) {
        v = 0;
        if (at!(s, i) < (b'0' as i32)) || (at!(s, i) > (b'9' as i32)) {
            state.g_err = (state.g_err + 1);
            return -2;
        }
        while ((i < (s.len() as i32)) && (at!(s, i) >= (b'0' as i32))) && (at!(s, i) <= (b'9' as i32)) {
            v = ((v as i32).wrapping_mul(10) + (at!(s, i) - (b'0' as i32)));
            i = (i as i32).wrapping_add(1);
        }
        if op == 0 {
            acc = v;
        } else if op == 1 {
            acc = (acc + v);
        } else {
            acc = (acc as i32).wrapping_mul(v);
        }
        if i >= (s.len() as i32) {
            *out = commit(state, acc);
            return 0;
        }
        if at!(s, i) == (b'+' as i32) {
            op = 1;
        } else if at!(s, i) == (b'*' as i32) {
            op = 2;
        } else {
            state.g_err = (state.g_err + 1);
            return -3;
        }
        i = (i as i32).wrapping_add(1);
        if i >= (s.len() as i32) {
            state.g_err = (state.g_err + 1);
            return at!(s, i);
        }
    }
    *out = commit(state, acc);
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
pub extern "C" fn commit_c(state: *mut State, v: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::commit(state, v)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn eval_expr_c(state: *mut State, s: *const u8, n: usize, out: *mut i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let s = match view(s, n) { Ok(s) => s, Err(c) => return c };
    if out.is_null() { return -1; }
    let out = unsafe { &mut *out };
    crate::eval_expr(state, s, out)
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
int32_t commit_c(State *state, int32_t v);
int32_t eval_expr_c(State *state, const uint8_t * s, size_t n, int32_t * out);
#endif
```
