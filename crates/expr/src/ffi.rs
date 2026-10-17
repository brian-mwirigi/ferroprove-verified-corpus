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
