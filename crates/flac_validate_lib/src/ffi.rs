// ABI shim. The only unsafe in the lift lives here, turning C pointers into slices.
use crate::{tflac};

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
pub extern "C" fn tflac_size_memory_c(blocksize: i32) -> i32 {
    crate::tflac_size_memory(blocksize)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn flac_validate_c(t: *mut tflac) -> i32 {
    if t.is_null() { return -1; }
    let t = unsafe { &mut *t };
    crate::flac_validate(t)
}
