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
pub extern "C" fn crc8_populate_msb_c(table: *mut u8, table_len: usize, polynomial: i32) -> i32 {
    let table = match view_mut(table, table_len) { Ok(s) => s, Err(c) => return c };
    crate::crc8_populate_msb(table, polynomial)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn crc8_populate_lsb_c(table: *mut u8, table_len: usize, polynomial: i32) -> i32 {
    let table = match view_mut(table, table_len) { Ok(s) => s, Err(c) => return c };
    crate::crc8_populate_lsb(table, polynomial)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn crc8_c(table: *const u8, table_len: usize, pdata: *const u8, pdata_len: usize, nbytes: i32, crc: i32) -> i32 {
    let table = match view(table, table_len) { Ok(s) => s, Err(c) => return c };
    let pdata = match view(pdata, pdata_len) { Ok(s) => s, Err(c) => return c };
    crate::crc8(table, pdata, nbytes, crc)
}
