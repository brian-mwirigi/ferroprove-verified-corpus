// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{tflac_md5};

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
pub extern "C" fn md5_digest_c(m: *const tflac_md5, out: *mut u8, out_len: usize) -> i32 {
    if m.is_null() { return -1; }
    let m = unsafe { &*m };
    let out = match view_mut(out, out_len) { Ok(s) => s, Err(c) => return c };
    crate::md5_digest(m, out)
}
