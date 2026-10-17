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
