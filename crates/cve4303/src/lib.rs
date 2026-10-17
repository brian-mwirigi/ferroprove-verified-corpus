//! FerroProve safe crate. Indexes become slice gets that return -100.
#![forbid(unsafe_code)]
#![allow(unused_variables, unused_mut, unused_assignments, unused_parens, unused_macros, dead_code, unreachable_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]

macro_rules! at {
    ($buf:expr, $i:expr) => {
        match $buf.get($i as usize) {
            Some(v) => *v as i32,
            None => return -100,
        }
    };
}
macro_rules! put {
    ($buf:expr, $i:expr, $v:expr) => {{
        let __idx = ($i) as usize;
        let __val = $v;
        match $buf.get_mut(__idx) {
            Some(slot) => *slot = __val,
            None => return -100,
        }
    }};
}

pub fn parse_hex4(str: & [u8]) -> i32 {
    let mut __str_off: i32 = 0;
    let mut h: i32 = 0;
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    h = (h as i32).wrapping_shl((4) as u32);
    __str_off = (__str_off + 1);
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    h = (h as i32).wrapping_shl((4) as u32);
    __str_off = (__str_off + 1);
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    h = (h as i32).wrapping_shl((4) as u32);
    __str_off = (__str_off + 1);
    if (at!(str, __str_off) >= (b'0' as i32)) && (at!(str, __str_off) <= (b'9' as i32)) {
        h = (h as i32).wrapping_add((at!(str, __str_off) - (b'0' as i32)));
    } else if (at!(str, __str_off) >= (b'A' as i32)) && (at!(str, __str_off) <= (b'F' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'A' as i32)));
    } else if (at!(str, __str_off) >= (b'a' as i32)) && (at!(str, __str_off) <= (b'f' as i32)) {
        h = (h as i32).wrapping_add(((10 + at!(str, __str_off)) - (b'a' as i32)));
    } else {
        return 0;
    }
    return h;
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1_u0571() {
        let mut str: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let rc = super::parse_hex4(&str);
        assert_eq!(rc, 1393);
        assert_eq!(str, vec![48u8, 53u8, 55u8, 49u8]);
    }

    #[test]
    fn oracle_2_beef() {
        let mut str: Vec<u8> = vec![66u8, 69u8, 69u8, 70u8];
        let rc = super::parse_hex4(&str);
        assert_eq!(rc, 48879);
        assert_eq!(str, vec![66u8, 69u8, 69u8, 70u8]);
    }

    #[test]
    fn oracle_3_badhex() {
        let mut str: Vec<u8> = vec![122u8, 122u8, 122u8, 122u8];
        let rc = super::parse_hex4(&str);
        assert_eq!(rc, 0);
        assert_eq!(str, vec![122u8, 122u8, 122u8, 122u8]);
    }

    #[test]
    fn oracle_4_cve_trunc() {
        let mut str: Vec<u8> = vec![48u8];
        let rc = super::parse_hex4(&str);
        assert_eq!(rc, -100);
    }

}
