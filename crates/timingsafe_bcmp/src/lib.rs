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

pub fn timingsafe_bcmp(b1: & [u8], b2: & [u8], mut n: i32) -> i32 {
    let mut __p1_off: i32 = 0;
    let mut __p2_off: i32 = 0;
    let mut ret: i32 = 0;
    while n > 0 {
        ret = (ret | (at!(b1, __p1_off) ^ at!(b2, __p2_off)));
        __p1_off = (__p1_off + 1);
        __p2_off = (__p2_off + 1);
        n = (n as i32).wrapping_sub(1);
    }
    return (if ret != 0 { 1 } else { 0 });
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1_std_123456789() {
        let mut b1: Vec<u8> = vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8];
        let mut b2: Vec<u8> = vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8];
        let n: i32 = 9;
        let rc = super::timingsafe_bcmp(&b1, &b2, n);
        assert_eq!(rc, 0);
        assert_eq!(b1, vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8]);
        assert_eq!(b2, vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8]);
    }

    #[test]
    fn oracle_2_ascii_0571() {
        let mut b1: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let mut b2: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let n: i32 = 2;
        let rc = super::timingsafe_bcmp(&b1, &b2, n);
        assert_eq!(rc, 0);
        assert_eq!(b1, vec![48u8, 53u8, 55u8, 49u8]);
        assert_eq!(b2, vec![48u8, 53u8, 55u8, 49u8]);
    }

    #[test]
    fn oracle_3_slices_differ() {
        let mut b1: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let mut b2: Vec<u8> = vec![49u8, 55u8, 53u8, 48u8];
        let n: i32 = 2;
        let rc = super::timingsafe_bcmp(&b1, &b2, n);
        assert_eq!(rc, 1);
        assert_eq!(b1, vec![48u8, 53u8, 55u8, 49u8]);
        assert_eq!(b2, vec![49u8, 55u8, 53u8, 48u8]);
    }

    #[test]
    fn oracle_4_empty_zero() {
        let mut b1: Vec<u8> = vec![];
        let mut b2: Vec<u8> = vec![];
        let n: i32 = 0;
        let rc = super::timingsafe_bcmp(&b1, &b2, n);
        assert_eq!(rc, 0);
        assert_eq!(b1, vec![]);
        assert_eq!(b2, vec![]);
    }

    #[test]
    fn oracle_5_oob_len_overread() {
        let mut b1: Vec<u8> = vec![49u8, 50u8];
        let mut b2: Vec<u8> = vec![49u8, 50u8];
        let n: i32 = 4;
        let rc = super::timingsafe_bcmp(&b1, &b2, n);
        assert_eq!(rc, -100);
    }

    #[test]
    fn oracle_6_unsigned_huge_len() {
        let mut b1: Vec<u8> = vec![49u8, 50u8];
        let mut b2: Vec<u8> = vec![49u8, 50u8];
        let n: i32 = 1000000;
        let rc = super::timingsafe_bcmp(&b1, &b2, n);
        assert_eq!(rc, -100);
    }

}
