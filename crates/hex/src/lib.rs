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

pub const hexval_table: [i32; 256] = [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];

pub fn hex_to_bytes(binary: &mut [u8], hex: & [u8], mut len: i32) -> i32 {
    let mut __binary_off: i32 = 0;
    let mut __hex_off: i32 = 0;
    while len != 0 {
        let mut val: i32 = ((at!(hexval_table, (at!(hex, (__hex_off + 0)) & 255)) << 4) | at!(hexval_table, (at!(hex, (__hex_off + 1)) & 255)));
        if (val & (!255)) != 0 {
            return -1;
        }
        put!(binary, (__binary_off + 0), (val as u8));
        __binary_off = (__binary_off + 1);
        len = (len as i32).wrapping_sub(1);
        __hex_off = (__hex_off + 2);
    }
    return 0;
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1_std_123456789() {
        let mut binary: Vec<u8> = vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8];
        let mut hex: Vec<u8> = vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8];
        let len: i32 = 9;
        let rc = super::hex_to_bytes(&mut binary, &hex, len);
        assert_eq!(rc, -100);
    }

    #[test]
    fn oracle_2_ascii_0571() {
        let mut binary: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let mut hex: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let len: i32 = 2;
        let rc = super::hex_to_bytes(&mut binary, &hex, len);
        assert_eq!(rc, 0);
        assert_eq!(binary, vec![5u8, 113u8, 55u8, 49u8]);
        assert_eq!(hex, vec![48u8, 53u8, 55u8, 49u8]);
    }

    #[test]
    fn oracle_3_slices_differ() {
        let mut binary: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let mut hex: Vec<u8> = vec![49u8, 55u8, 53u8, 48u8];
        let len: i32 = 2;
        let rc = super::hex_to_bytes(&mut binary, &hex, len);
        assert_eq!(rc, 0);
        assert_eq!(binary, vec![23u8, 80u8, 55u8, 49u8]);
        assert_eq!(hex, vec![49u8, 55u8, 53u8, 48u8]);
    }

    #[test]
    fn oracle_4_empty_zero() {
        let mut binary: Vec<u8> = vec![];
        let mut hex: Vec<u8> = vec![];
        let len: i32 = 0;
        let rc = super::hex_to_bytes(&mut binary, &hex, len);
        assert_eq!(rc, 0);
        assert_eq!(binary, vec![]);
        assert_eq!(hex, vec![]);
    }

    #[test]
    fn oracle_5_oob_len_overread() {
        let mut binary: Vec<u8> = vec![49u8, 50u8];
        let mut hex: Vec<u8> = vec![49u8, 50u8];
        let len: i32 = 4;
        let rc = super::hex_to_bytes(&mut binary, &hex, len);
        assert_eq!(rc, -100);
    }

    #[test]
    fn oracle_6_unsigned_huge_len() {
        let mut binary: Vec<u8> = vec![49u8, 50u8];
        let mut hex: Vec<u8> = vec![49u8, 50u8];
        let len: i32 = 1000000;
        let rc = super::hex_to_bytes(&mut binary, &hex, len);
        assert_eq!(rc, -100);
    }

}
