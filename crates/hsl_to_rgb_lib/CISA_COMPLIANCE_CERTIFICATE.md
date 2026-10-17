# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 hsl_to_rgb_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `e95d92d0c94b9de5` (52 lines)
- **Safe Rust Crate Digest (FNV-64):** `82578599a7c58365` (71 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `c93a25c13db84e12`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (233 ms, 12948 B .rlib, sha256:27054b240697f95c)

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

- **Leaf-to-Root Translation Order:** `hsl_to_rgb`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (27):**
  - `L9`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L10`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L11`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L14`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L15`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L16`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L23`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L24`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L25`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L27`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L28`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L29`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L31`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L32`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L33`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L35`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L36`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L37`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L39`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L40`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L41`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L43`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L44`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L45`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L47`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L48`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L49`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 8
- **Byte-Exact Functional Parity:** 8 / 8
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[0x00 0x00 0x00]` | `return 0` | `return 0` | **EXACT** |
| 2.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[0xb4 0x00 0.5]` | `return 0  dest=[0.5 0.5 0.5]  src=[0xb4 0x00 0.5]` | `return 0  dest=[0.5 0.5 0.5]  src=[0xb4 0x00 0.5]` | **EXACT** |
| 3.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[360 0x00 0x01]` | `return 0  dest=[0x01 0x01 0x01]  src=[360 0x00 0x01]` | `return 0  dest=[0x01 0x01 0x01]  src=[360 0x00 0x01]` | **EXACT** |
| 4.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[0x5a 0x00 0.25]` | `return 0  dest=[0.25 0.25 0.25]  src=[Z 0x00 0.25]` | `return 0  dest=[0.25 0.25 0.25]  src=[Z 0x00 0.25]` | **EXACT** |
| 5.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[270 0x00 0.75]` | `return 0  dest=[0.75 0.75 0.75]  src=[270 0x00 0.75]` | `return 0  dest=[0.75 0.75 0.75]  src=[270 0x00 0.75]` | **EXACT** |
| 6.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[0x00 0x01 0.5]` | `return 0  dest=[0x01 0x00 0x00]  src=[0x00 0x01 0.5]` | `return 0  dest=[0x01 0x00 0x00]  src=[0x00 0x01 0.5]` | **EXACT** |
| 7.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[0x1e 0x01 0.5]` | `return 0  dest=[0x01 0.5 0x00]  src=[0x1e 0x01 0.5]` | `return 0  dest=[0x01 0.5 0x00]  src=[0x1e 0x01 0.5]` | **EXACT** |
| 8.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[59.900001525878906 0x01 0.5]` | `return 0  dest=[0x01 0.9983333349227905 0x00]  src=[59.900001525878906 0x01 0.5]` | `return 0  dest=[0x01 0.9983333349227905 0x00]  src=[59.900001525878906 0x01 0.5]` | **EXACT** |

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

pub fn hsl_to_rgb(dest: &mut [f32], src: & [f32]) -> i32 {
    let mut h: f32 = (match src.get((0) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut s: f32 = (match src.get((1) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut l: f32 = (match src.get((2) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut c: f32 = (0.0 as f32);
    let mut m: f32 = (0.0 as f32);
    let mut x: f32 = (0.0 as f32);
    if s == (0 as f32) {
        put!(dest, 0, (l as f32));
        put!(dest, 1, (l as f32));
        put!(dest, 2, (l as f32));
        return 0;
    }
    c = ((1.0f32 - (((2.0f32 * l) - 1.0f32) as f32).abs()) * s);
    m = (1.0f32 * (l - (0.5f32 * c)));
    x = (c * (1.0f32 - (((((h / 60.0f32) as f32) % (2 as f32)) - 1.0f32) as f32).abs()));
    if (h >= 0.0f32) && (h < 60.0f32) {
        put!(dest, 0, ((c + m) as f32));
        put!(dest, 1, ((x + m) as f32));
        put!(dest, 2, (m as f32));
    } else if (h >= 60.0f32) && (h < 120.0f32) {
        put!(dest, 0, ((x + m) as f32));
        put!(dest, 1, ((c + m) as f32));
        put!(dest, 2, (m as f32));
    } else if (h < 120.0f32) && (h < 180.0f32) {
        put!(dest, 0, (m as f32));
        put!(dest, 1, ((c + m) as f32));
        put!(dest, 2, ((x + m) as f32));
    } else if (h >= 180.0f32) && (h < 240.0f32) {
        put!(dest, 0, (m as f32));
        put!(dest, 1, ((x + m) as f32));
        put!(dest, 2, ((c + m) as f32));
    } else if (h >= 240.0f32) && (h < 300.0f32) {
        put!(dest, 0, ((x + m) as f32));
        put!(dest, 1, (m as f32));
        put!(dest, 2, ((c + m) as f32));
    } else if (h >= 300.0f32) && (h < 360.0f32) {
        put!(dest, 0, ((c + m) as f32));
        put!(dest, 1, (m as f32));
        put!(dest, 2, ((x + m) as f32));
    } else {
        put!(dest, 0, (m as f32));
        put!(dest, 1, (m as f32));
        put!(dest, 2, (m as f32));
    }
    0
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
pub extern "C" fn hsl_to_rgb_c(dest: *mut f32, dest_len: usize, src: *const f32, src_len: usize) -> i32 {
    let dest = match view_mut(dest, dest_len) { Ok(s) => s, Err(c) => return c };
    let src = match view(src, src_len) { Ok(s) => s, Err(c) => return c };
    crate::hsl_to_rgb(dest, src)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t hsl_to_rgb_c(float * dest, size_t dest_len, const float * src, size_t src_len);
#endif
```
