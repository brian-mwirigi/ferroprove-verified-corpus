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

pub fn hdr_valid(h: & [u8]) -> i32 {
    return (if ((((at!(h, 0) == 255) && (((at!(h, 1) & 240) == 240) || ((at!(h, 1) & 254) == 226))) && (((at!(h, 1) >> 1) & 3) != 0)) && ((at!(h, 2) >> 4) != 15)) && (((at!(h, 2) >> 2) & 3) != 3) { 1 } else { 0 });
}

pub fn hdr_compare(h1: & [u8], h2: & [u8]) -> i32 {
    return (if (((hdr_valid(h2) != 0) && (((at!(h1, 1) ^ at!(h2, 1)) & 254) == 0)) && (((at!(h1, 2) ^ at!(h2, 2)) & 12) == 0)) && (!(((if (at!(h1, 2) & 240) == 0 { 1 } else { 0 }) ^ (if (at!(h2, 2) & 240) == 0 { 1 } else { 0 })) != 0)) { 1 } else { 0 });
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1__1_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![254u8, 242u8, 16u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 0);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![254u8, 242u8, 16u8]);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 162u8, 16u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 0);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 162u8, 16u8]);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 240u8, 16u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 0);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 240u8, 16u8]);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 242u8, 240u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 0);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 242u8, 240u8]);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 242u8, 28u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 0);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 242u8, 28u8]);
    }

    #[test]
    fn oracle_6__6_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 242u8, 16u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 1);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 242u8, 16u8]);
    }

    #[test]
    fn oracle_7__7_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 244u8, 16u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 0);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 244u8, 16u8]);
    }

    #[test]
    fn oracle_8__8_json() {
        let mut h1: Vec<u8> = vec![255u8, 242u8, 16u8];
        let mut h2: Vec<u8> = vec![255u8, 242u8, 16u8];
        let rc = super::hdr_compare(&h1, &h2);
        assert_eq!(rc, 1);
        assert_eq!(h1, vec![255u8, 242u8, 16u8]);
        assert_eq!(h2, vec![255u8, 242u8, 16u8]);
    }

}
