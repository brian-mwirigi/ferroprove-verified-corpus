// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{State, FrameSummary};

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
pub extern "C" fn fold_checksum_c(state: *mut State, acc: i32, byte_val: i32) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    crate::fold_checksum(state, acc, byte_val)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn inspect_frame_c(state: *mut State, pkt: *const u8, n: usize, out_meta: *mut FrameSummary) -> i32 {
    if state.is_null() { return -1; }
    let state = unsafe { &mut *state };
    let pkt = match view(pkt, n) { Ok(s) => s, Err(c) => return c };
    if out_meta.is_null() { return -1; }
    let out_meta = unsafe { &mut *out_meta };
    crate::inspect_frame(state, pkt, out_meta)
}
