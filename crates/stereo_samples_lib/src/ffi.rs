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
pub extern "C" fn btac1c2_fakesqrt_c(val: i32) -> i32 {
    crate::btac1c2_fakesqrt(val)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn stereo_samples_c(ibuf0: *mut i16, ibuf0_len: usize, ibuf1: *mut i16, ibuf1_len: usize, len: i32) -> i32 {
    let ibuf0 = match view_mut(ibuf0, ibuf0_len) { Ok(s) => s, Err(c) => return c };
    let ibuf1 = match view_mut(ibuf1, ibuf1_len) { Ok(s) => s, Err(c) => return c };
    crate::stereo_samples(ibuf0, ibuf1, len)
}
