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
pub extern "C" fn adler32_z_c(adler: i32, buf: *const u8, buf_len: usize, len: i32) -> i32 {
    let buf = match view(buf, buf_len) { Ok(s) => s, Err(c) => return c };
    crate::adler32_z(adler, buf, len)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn adler32_c(adler: i32, buf: *const u8, buf_len: usize, len: i32) -> i32 {
    let buf = match view(buf, buf_len) { Ok(s) => s, Err(c) => return c };
    crate::adler32(adler, buf, len)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn adler32_combine__c(adler1: i32, adler2: i32, len2: i32) -> i32 {
    crate::adler32_combine_(adler1, adler2, len2)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn adler32_combine_c(adler1: i32, adler2: i32, len2: i32) -> i32 {
    crate::adler32_combine(adler1, adler2, len2)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn adler32_combine64_c(adler1: i32, adler2: i32, len2: i32) -> i32 {
    crate::adler32_combine64(adler1, adler2, len2)
}
