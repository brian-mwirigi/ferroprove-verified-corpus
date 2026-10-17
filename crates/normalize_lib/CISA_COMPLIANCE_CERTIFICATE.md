# CISA MEMORY SAFETY ROADMAP — ARTIFACT COMPLIANCE CERTIFICATE

- **Module:** Battery 01 normalize_lib
- **Policy Alignment:** CISA / NSA / ONCD "Product Security Bad Practices" (Jan 1, 2026 Memory Safety Mandate)
- **Legacy C Digest (FNV-64):** `9f490020db95ddee` (22 lines)
- **Safe Rust Crate Digest (FNV-64):** `dc9df8b77fa786b5` (49 lines)
- **ABI Drop-In Shim Digest (FNV-64):** `bfdb70e3d5ec9718`
- **Compiler Verification:** `rustc 1.96.0 (ac68faa20 2026-05-25)` — PASS (279 ms, 11948 B .rlib, sha256:5d60be95dec55aaa)

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

- **Leaf-to-Root Translation Order:** `normalize`
- **Mutable Globals Threaded into `State`:** None
- **Legacy Memory Hazards Eliminated (3):**
  - `L13`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L17`: unchecked index dest[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)
  - `L17`: unchecked index src[] -> lifted to checked slice `at! / put!` (returns `-100` on OOB)

---

## 3. Differential Equivalence & Exploit Containment Oracle

- **Total Vectors Executed:** 5
- **Byte-Exact Functional Parity:** 5 / 5
- **Legacy Out-of-Bounds / Memory Corruption Contained:** 0
- **Behavioral Regressions / Mismatches:** 0

| Vector | Input | Legacy C Outcome | Safe Rust Outcome | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| 1.json | `dest[0]=[]  src[2]=[0.1 0.2]  size=2` | `return 0  dest=[0.4472135901451111 0.8944271802902222]  src=[0.1 0.2]` | `return 0  dest=[0.4472135901451111 0.8944271802902222]  src=[0.1 0.2]` | **EXACT** |
| 2.json | `dest[0]=[]  src[0]=[]  size=0` | `return 0` | `return 0` | **EXACT** |
| 3.json | `dest[0]=[]  src[5]=[0.1 0.2 1000 100000 1000000000]  size=5` | `return 0  dest=[9.999999439624929e-11 1.9999998879249858e-10 9.999999974752427e-7 0.00009999999747378752 0x01]  src=[0.1 0.2 1000 100000 1000000000]` | `return 0  dest=[9.999999439624929e-11 1.9999998879249858e-10 9.999999974752427e-7 0.00009999999747378752 0x01]  src=[0.1 0.2 1000 100000 1000000000]` | **EXACT** |
| 4.json | `dest[0]=[]  src[14]=[-0.00021050152 -70.1147 -0.00028400574 -17160019000000 1370915294 0.44721356 0.8944271 -0.00021050152 -70.1147 -0.00028400574 -17160019000000 1370915294 0.44721356 0.8944271]  size=14` | `return 0  dest=[-8.674060748692364e-18 -2.8891912733780556e-12 -1.1702923182259409e-17 -0.7071067690849304 0.000056490815040888265 1.8428169079143068e-14 3.6856338158286137e-14 -8.674060748692364e-18 -2.8891912733780556e-12 -1.1702923182259409e-17 -0.7071067690849304 0.000056490815040888265 1.8428169079143068e-14 3.6856338158286137e-14]  src=[-0.00021050152 -70.1147 -0.00028400574 -17160019000000 1370915294 0.44721356 0.8944271 -0.00021050152 -70.1147 -0.00028400574 -17160019000000 1370915294 0.44721356 0.8944271]` | `return 0  dest=[-8.674060748692364e-18 -2.8891912733780556e-12 -1.1702923182259409e-17 -0.7071067690849304 0.000056490815040888265 1.8428169079143068e-14 3.6856338158286137e-14 -8.674060748692364e-18 -2.8891912733780556e-12 -1.1702923182259409e-17 -0.7071067690849304 0.000056490815040888265 1.8428169079143068e-14 3.6856338158286137e-14]  src=[-0.00021050152 -70.1147 -0.00028400574 -17160019000000 1370915294 0.44721356 0.8944271 -0.00021050152 -70.1147 -0.00028400574 -17160019000000 1370915294 0.44721356 0.8944271]` | **EXACT** |
| 5.json | `dest[0]=[]  src[25]=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 1000000000000 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]  size=25` | `return 0  dest=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]  src=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 1000000000000 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | `return 0  dest=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]  src=[0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 1000000000000 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00]` | **EXACT** |

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

pub fn normalize(dest: &mut [f32], src: & [f32], mut size: i32) -> i32 {
    let mut sum: f32 = (0.0f32 as f32);
    let mut i: i32 = 0;
    i = 0;
    while i < size {
        sum = (sum + (match src.get((i) as usize) { Some(__v) => *__v, None => return -100 as _ } * match src.get((i) as usize) { Some(__v) => *__v, None => return -100 as _ }));
        i = (i + 1);
    }
    if sum > 0.0f32 {
        sum = (1.0f32 / (sum as f32).sqrt());
        i = 0;
        while i < size {
            put!(dest, i, ((match src.get((i) as usize) { Some(__v) => *__v, None => return -100 as _ } * sum) as f32));
            i = (i + 1);
        }
    } else if dest != src {
        {
        let __n = ((size as i32).wrapping_mul(1)) as usize;
        if __n > dest.len() { return -100; }
        for __slot in &mut dest[..__n] { *__slot = (0) as f32; }
        0
    };
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
pub extern "C" fn normalize_c(dest: *mut f32, dest_len: usize, src: *const f32, src_len: usize, size: i32) -> i32 {
    let dest = match view_mut(dest, dest_len) { Ok(s) => s, Err(c) => return c };
    let src = match view(src, src_len) { Ok(s) => s, Err(c) => return c };
    crate::normalize(dest, src, size)
}
```

## 6. C Header (`include/ferroprove.h`)

```c
#ifndef FERROPROVE_LIFT_H
#define FERROPROVE_LIFT_H
#include <stdint.h>
#include <stddef.h>
typedef struct State State;
int32_t normalize_c(float * dest, size_t dest_len, const float * src, size_t src_len, int32_t size);
#endif
```
