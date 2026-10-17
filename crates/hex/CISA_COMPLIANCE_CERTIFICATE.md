# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** GitHub: git/git@v2.30.0:hex.c#L38-L48
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `7038d8a75c14fc41` (46 lines)
- **Safe Rust Crate Digest (FNV-64):** `8db58aeb47736498` (41 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `2b92f475c43c71d3`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (227 ms, 20588 B .rlib, sha256:a1634badcc2516d4)

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

- **Leaf-to-Root Translation Order:** `hex_to_bytes`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (3):**
  - `L39`: unchecked index hexval_table[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L39`: unchecked index hex[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L43`: unchecked index binary[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 6
- **Byte-Exact Functional Parity:** 3 / 3
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 3
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| ASCII "123456789" (len=9) | `binary[9]=[0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39]  hex[9]=[0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39]  len=9` | `undefined behavior — hex[9] past length 9` | `return -100  binary=[0x12 4 V x 5 6 7 8 9]  hex=[1 2 3 4 5 6 7 8 9]` | **CONTAINED** |
| ASCII "0571" (len=2) | `binary[4]=[0x30 0x35 0x37 0x31]  hex[4]=[0x30 0x35 0x37 0x31]  len=2` | `return 0  binary=[0x05 q 7 1]  hex=[0 5 7 1]` | `return 0  binary=[0x05 q 7 1]  hex=[0 5 7 1]` | **EXACT** |
| second buffer differs in byte 0 (len=2) | `binary[4]=[0x30 0x35 0x37 0x31]  hex[4]=[0x31 0x37 0x35 0x30]  len=2` | `return 0  binary=[0x17 P 7 1]  hex=[1 7 5 0]` | `return 0  binary=[0x17 P 7 1]  hex=[1 7 5 0]` | **EXACT** |
| empty buffer (len=0) | `binary[0]=[]  hex[0]=[]  len=0` | `return 0` | `return 0` | **EXACT** |
| len=4 on 2-byte buffer (over-read) | `binary[2]=[0x31 0x32]  hex[2]=[0x31 0x32]  len=4` | `undefined behavior — hex[2] past length 2` | `return -100  binary=[0x12 2]  hex=[1 2]` | **CONTAINED** |
| size_t length 1000000 on a 2-byte buffer | `binary[2]=[0x31 0x32]  hex[2]=[0x31 0x32]  len=1000000` | `undefined behavior — hex[2] past length 2` | `return -100  binary=[0x12 2]  hex=[1 2]` | **CONTAINED** |

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

pub const hexval_table: [i32; 256] = [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];

pub fn hex_to_bytes(binary: &mut [u8], hex: & [u8], mut len: i32) -> i32 {
    let mut __binary_off: i32 = 0;
    let mut __hex_off: i32 = 0;
    while len != 0 {
        let mut val: i32 = ((at!(hexval_table, (at!(hex, (__hex_off + 0)) & 255)) << 4) | at!(hexval_table, (at!(hex, (__hex_off + 1)) & 255)));
        if (val & (!255)) != 0 {
            return -1;
        }
        put!(binary, (__binary_off + 0), (val as u8));
        __binary_off = (__binary_off + 1);
        len = (len as i32).wrapping_sub(1);
        __hex_off = (__hex_off + 2);
    }
    return 0;
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
pub extern "C" fn hex_to_bytes_c(binary: *mut u8, binary_len: usize, hex: *const u8, hex_len: usize, len: i32) -> i32 {
    let binary = match view_mut(binary, binary_len) { Ok(s) => s, Err(c) => return c };
    let hex = match view(hex, hex_len) { Ok(s) => s, Err(c) => return c };
    crate::hex_to_bytes(binary, hex, len)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t hex_to_bytes_c(uint8_t * binary, size_t binary_len, const uint8_t * hex, size_t hex_len, int32_t len);
#endif
```
