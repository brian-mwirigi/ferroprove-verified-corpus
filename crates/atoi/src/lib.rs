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

pub fn atoi(s: & [u8]) -> i32 {
    let mut __s_off: i32 = 0;
    let mut n: i32 = 0;
    let mut neg: i32 = 0;
    while (at!(s, __s_off) == (b' ' as i32)) || ((at!(s, __s_off) >= (b'\t' as i32)) && (at!(s, __s_off) <= (b'\r' as i32))) {
        __s_off = (__s_off + 1);
    }
    if at!(s, __s_off) == (b'-' as i32) {
        neg = 1;
        __s_off = (__s_off + 1);
    } else if at!(s, __s_off) == (b'+' as i32) {
        __s_off = (__s_off + 1);
    }
    while (at!(s, __s_off) >= (b'0' as i32)) && (at!(s, __s_off) <= (b'9' as i32)) {
        n = ((10 as i32).wrapping_mul(n) - (at!(s, __s_off) - (b'0' as i32)));
        __s_off = (__s_off + 1);
    }
    return (if neg != 0 { n } else { (-n) });
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1_valid_0571_nul() {
        let mut s: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8, 0u8];
        let rc = super::atoi(&s);
        assert_eq!(rc, 571);
        assert_eq!(s, vec![48u8, 53u8, 55u8, 49u8, 0u8]);
    }

    #[test]
    fn oracle_2_valid_neg42_nul() {
        let mut s: Vec<u8> = vec![45u8, 52u8, 50u8, 0u8];
        let rc = super::atoi(&s);
        assert_eq!(rc, -42);
        assert_eq!(s, vec![45u8, 52u8, 50u8, 0u8]);
    }

    #[test]
    fn oracle_3_truncated_1byte() {
        let mut s: Vec<u8> = vec![48u8];
        let rc = super::atoi(&s);
        assert_eq!(rc, -100);
    }

    #[test]
    fn oracle_4_empty_slice() {
        let mut s: Vec<u8> = vec![];
        let rc = super::atoi(&s);
        assert_eq!(rc, -100);
    }

}
