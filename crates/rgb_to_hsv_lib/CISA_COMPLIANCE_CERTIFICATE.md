# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 rgb_to_hsv_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `af9daf14202b777b` (42 lines)
- **Safe Rust Crate Digest (FNV-64):** `2dba88dfb5191ecf` (63 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `4628499d98330aca`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (222 ms, 11804 B .rlib, sha256:b76e843c302f0738)

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

- **Leaf-to-Root Translation Order:** `rgb_to_hsv`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (9):**
  - `L7`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L8`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L9`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L23`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L24`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L25`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L38`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L39`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L40`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 5
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[2.5580184e+32 1.0345711e-20 28193286000000000000]` | `return 0  dest=[360 0x01 2.5580183673180012e+32]  src=[2.5580184e+32 1.0345711e-20 28193286000000000000]` | `return 0  dest=[360 0x01 2.5580183673180012e+32]  src=[2.5580184e+32 1.0345711e-20 28193286000000000000]` | **EXACT** |
| 2.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[-3.4547846e-12 -0.000044706365 41262783000000]` | `return 0  dest=[0xf0 0x01 41262782611456]  src=[-3.4547846e-12 -0.000044706365 41262783000000]` | `return 0  dest=[0xf0 0x01 41262782611456]  src=[-3.4547846e-12 -0.000044706365 41262783000000]` | **EXACT** |
| 3.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[-620678550000 -3.8153195 9.080249e-21]` | `return 0  dest=[0xb4 6.8354793550453655e+31 9.080249158504418e-21]  src=[-620678550000 -3.8153195 9.080249e-21]` | `return 0  dest=[0xb4 6.8354793550453655e+31 9.080249158504418e-21]  src=[-620678550000 -3.8153195 9.080249e-21]` | **EXACT** |
| 4.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[-13417.771 -1.5067105e-8 -0.15898927]` | `return 0  dest=[179.99928283691406 -890534166528 -1.506710489707075e-8]  src=[-13417.771 -1.5067105e-8 -0.15898927]` | `return 0  dest=[179.99928283691406 -890534166528 -1.506710489707075e-8]  src=[-13417.771 -1.5067105e-8 -0.15898927]` | **EXACT** |
| 5.json | `dest[3]=[0x00 0x00 0x00]  src[3]=[2.9476963e-36 8.3269427e+24 -1.0019444e-29]` | `return 0  dest=[x 0x01 8.326942708760071e+24]  src=[2.9476963e-36 8.3269427e+24 -1.0019444e-29]` | `return 0  dest=[x 0x01 8.326942708760071e+24]  src=[2.9476963e-36 8.3269427e+24 -1.0019444e-29]` | **EXACT** |

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

pub fn rgb_to_hsv(dest: &mut [f32], src: & [f32]) -> i32 {
    let mut r: f32 = (match src.get((0) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut g: f32 = (match src.get((1) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut b: f32 = (match src.get((2) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut h: f32 = (0 as f32);
    let mut s: f32 = (0 as f32);
    let mut v: f32 = (0 as f32);
    let mut min: f32 = (r as f32);
    let mut max: f32 = (r as f32);
    let mut delta: f32 = (0.0 as f32);
    min = (if min < g { min } else { g });
    min = (if min < b { min } else { b });
    max = (if max > g { max } else { g });
    max = (if max > b { max } else { b });
    delta = (max - min);
    v = max;
    if (delta == (0 as f32)) || (max == (0 as f32)) {
        put!(dest, 0, (h as f32));
        put!(dest, 1, (s as f32));
        put!(dest, 2, (v as f32));
        return 0;
    }
    s = (delta / max);
    if r == max {
        h = ((g - b) / delta);
    } else if g == max {
        h = ((2 as f32) + ((b - r) / delta));
    } else {
        h = ((4 as f32) + ((r - g) / delta));
    }
    h = (h * (60 as f32));
    if h < (0 as f32) {
        h = (h + (360 as f32));
    }
    put!(dest, 0, (h as f32));
    put!(dest, 1, (s as f32));
    put!(dest, 2, (v as f32));
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
pub extern "C" fn rgb_to_hsv_c(dest: *mut f32, dest_len: usize, src: *const f32, src_len: usize) -> i32 {
    let dest = match view_mut(dest, dest_len) { Ok(s) => s, Err(c) => return c };
    let src = match view(src, src_len) { Ok(s) => s, Err(c) => return c };
    crate::rgb_to_hsv(dest, src)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t rgb_to_hsv_c(float * dest, size_t dest_len, const float * src, size_t src_len);
#endif
```
