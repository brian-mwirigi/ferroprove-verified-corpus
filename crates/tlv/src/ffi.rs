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
