// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{spritebatch_sprite_t};

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
pub extern "C" fn spritebatch_internal_sprite_less_than_or_equal_c(a: *mut spritebatch_sprite_t, b: *mut spritebatch_sprite_t) -> i32 {
    if a.is_null() { return -1; }
    let a = unsafe { &mut *a };
    if b.is_null() { return -1; }
    let b = unsafe { &mut *b };
    crate::spritebatch_internal_sprite_less_than_or_equal(a, b)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn spritebatch_internal_merge_sort_iteration_c(a: *mut spritebatch_sprite_t, a_len: usize, lo: i32, split: i32, hi: i32, b: *mut spritebatch_sprite_t, b_len: usize) -> i32 {
    let a = match view_mut(a, a_len) { Ok(s) => s, Err(c) => return c };
    let b = match view_mut(b, b_len) { Ok(s) => s, Err(c) => return c };
    crate::spritebatch_internal_merge_sort_iteration(a, lo, split, hi, b)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn spritebatch_internal_merge_sort_recurse_c(b: *mut spritebatch_sprite_t, b_len: usize, lo: i32, hi: i32, a: *mut spritebatch_sprite_t, a_len: usize) -> i32 {
    let b = match view_mut(b, b_len) { Ok(s) => s, Err(c) => return c };
    let a = match view_mut(a, a_len) { Ok(s) => s, Err(c) => return c };
    crate::spritebatch_internal_merge_sort_recurse(b, lo, hi, a)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn merge_sort_c(a: *mut spritebatch_sprite_t, a_len: usize, b: *mut spritebatch_sprite_t, b_len: usize, size: i32) -> i32 {
    let a = match view_mut(a, a_len) { Ok(s) => s, Err(c) => return c };
    let b = match view_mut(b, b_len) { Ok(s) => s, Err(c) => return c };
    crate::merge_sort(a, b, size)
}
