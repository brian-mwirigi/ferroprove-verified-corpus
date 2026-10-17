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

pub fn adler32_z(mut adler: i32, buf: & [u8], mut len: i32) -> i32 {
    let mut __buf_off: i32 = 0;
    let mut sum2: i32 = 0;
    let mut n: i32 = 0;
    sum2 = ((((adler) as u32).wrapping_shr((16) as u32) as i32) & 65535);
    adler = (adler & 65535);
    if len == 1 {
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 0)));
        if adler >= 65521 {
            adler = (adler as i32).wrapping_sub(65521);
        }
        sum2 = (sum2 as i32).wrapping_add(adler);
        if sum2 >= 65521 {
            sum2 = (sum2 as i32).wrapping_sub(65521);
        }
        return (adler | (sum2 as i32).wrapping_shl((16) as u32));
    }
    if false {
        return 1;
    }
    if len < 16 {
        while len != 0 {
            len = (len as i32).wrapping_sub(1);
            adler = (adler as i32).wrapping_add(at!(buf, __buf_off));
            __buf_off = (__buf_off + 1);
            sum2 = (sum2 as i32).wrapping_add(adler);
        }
        if adler >= 65521 {
            adler = (adler as i32).wrapping_sub(65521);
        }
        sum2 = (sum2 % 65521);
        return (adler | (sum2 as i32).wrapping_shl((16) as u32));
    }
    while len >= 5552 {
        len = (len as i32).wrapping_sub(5552);
        n = (5552 / 16);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 0)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 1)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 2)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 3)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 4)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 5)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 6)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 7)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 8)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 9)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 10)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 11)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 12)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 13)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 14)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 15)));
        sum2 = (sum2 as i32).wrapping_add(adler);
        __buf_off = (__buf_off + 16);
        n = (n as i32).wrapping_sub(1);
        while n != 0 {
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 0)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 1)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 2)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 3)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 4)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 5)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 6)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 7)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 8)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 9)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 10)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 11)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 12)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 13)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 14)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 15)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            __buf_off = (__buf_off + 16);
            n = (n as i32).wrapping_sub(1);
        }
        adler = (adler % 65521);
        sum2 = (sum2 % 65521);
    }
    if len != 0 {
        while len >= 16 {
            len = (len as i32).wrapping_sub(16);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 0)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 1)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 2)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 3)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 4)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 5)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 6)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 7)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 8)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 9)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 10)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 11)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 12)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 13)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 14)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            adler = (adler as i32).wrapping_add(at!(buf, (__buf_off + 15)));
            sum2 = (sum2 as i32).wrapping_add(adler);
            __buf_off = (__buf_off + 16);
        }
        while len != 0 {
            len = (len as i32).wrapping_sub(1);
            adler = (adler as i32).wrapping_add(at!(buf, __buf_off));
            __buf_off = (__buf_off + 1);
            sum2 = (sum2 as i32).wrapping_add(adler);
        }
        adler = (adler % 65521);
        sum2 = (sum2 % 65521);
    }
    return (adler | (sum2 as i32).wrapping_shl((16) as u32));
}

pub fn adler32(mut adler: i32, buf: & [u8], mut len: i32) -> i32 {
    return adler32_z(adler, buf, len);
}

pub fn adler32_combine_(mut adler1: i32, mut adler2: i32, mut len2: i32) -> i32 {
    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    let mut rem: i32 = 0;
    if len2 < 0 {
        return -1;
    }
    len2 = (len2 % 65521);
    rem = len2;
    sum1 = (adler1 & 65535);
    sum2 = (rem as i32).wrapping_mul(sum1);
    sum2 = (sum2 % 65521);
    sum1 = (sum1 as i32).wrapping_add((((adler2 & 65535) as i32).wrapping_add(65521) as i32).wrapping_sub(1));
    sum2 = (sum2 as i32).wrapping_add((((((((adler1) as u32).wrapping_shr((16) as u32) as i32) & 65535) as i32).wrapping_add(((((adler2) as u32).wrapping_shr((16) as u32) as i32) & 65535)) as i32).wrapping_add(65521) as i32).wrapping_sub(rem));
    if sum1 >= 65521 {
        sum1 = (sum1 as i32).wrapping_sub(65521);
    }
    if sum1 >= 65521 {
        sum1 = (sum1 as i32).wrapping_sub(65521);
    }
    if sum2 >= (65521 << 1) {
        sum2 = (sum2 as i32).wrapping_sub((65521 << 1));
    }
    if sum2 >= 65521 {
        sum2 = (sum2 as i32).wrapping_sub(65521);
    }
    return (sum1 | (sum2 as i32).wrapping_shl((16) as u32));
}

pub fn adler32_combine(mut adler1: i32, mut adler2: i32, mut len2: i32) -> i32 {
    return adler32_combine_(adler1, adler2, len2);
}

pub fn adler32_combine64(mut adler1: i32, mut adler2: i32, mut len2: i32) -> i32 {
    return adler32_combine_(adler1, adler2, len2);
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1_std_123456789() {
        let adler: i32 = 1;
        let mut buf: Vec<u8> = vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8];
        let len: i32 = 9;
        let rc = super::adler32(adler, &buf, len);
        assert_eq!(rc, 152961502);
        assert_eq!(buf, vec![49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8]);
    }

    #[test]
    fn oracle_2_ascii_0571() {
        let adler: i32 = 1;
        let mut buf: Vec<u8> = vec![48u8, 53u8, 55u8, 49u8];
        let len: i32 = 2;
        let rc = super::adler32(adler, &buf, len);
        assert_eq!(rc, 9896038);
        assert_eq!(buf, vec![48u8, 53u8, 55u8, 49u8]);
    }

    #[test]
    fn oracle_3_empty_zero() {
        let adler: i32 = 1;
        let mut buf: Vec<u8> = vec![];
        let len: i32 = 0;
        let rc = super::adler32(adler, &buf, len);
        assert_eq!(rc, 1);
        assert_eq!(buf, vec![]);
    }

    #[test]
    fn oracle_4_oob_len_overread() {
        let adler: i32 = 1;
        let mut buf: Vec<u8> = vec![49u8, 50u8];
        let len: i32 = 4;
        let rc = super::adler32(adler, &buf, len);
        assert_eq!(rc, -100);
    }

    #[test]
    fn oracle_5_unsigned_huge_len() {
        let adler: i32 = 1;
        let mut buf: Vec<u8> = vec![49u8, 50u8];
        let len: i32 = 1000000;
        let rc = super::adler32(adler, &buf, len);
        assert_eq!(rc, -100);
    }

}
