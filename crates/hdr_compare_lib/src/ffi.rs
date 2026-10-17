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
pub extern "C" fn hdr_valid_c(h: *const u8, h_len: usize) -> i32 {
    let h = match view(h, h_len) { Ok(s) => s, Err(c) => return c };
    crate::hdr_valid(h)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn hdr_compare_c(h1: *const u8, h1_len: usize, h2: *const u8, h2_len: usize) -> i32 {
    let h1 = match view(h1, h1_len) { Ok(s) => s, Err(c) => return c };
    let h2 = match view(h2, h2_len) { Ok(s) => s, Err(c) => return c };
    crate::hdr_compare(h1, h2)
}
