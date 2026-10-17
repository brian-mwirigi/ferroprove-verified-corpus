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

pub fn btac1c2_fakesqrt(mut val: i32) -> i32 {
    let mut v: i32 = 0;
    let mut v1: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if val < 0 {
        return -btac1c2_fakesqrt((-val));
    }
    v = val;
    while v > (1 << 30) {
        v = (v >> 1);
    }
    while (v as i32).wrapping_mul(v) > val {
        v = (v >> 1);
    }
    i = 1;
    while i < 17 {
        v1 = (v + (v >> i));
        j = 16;
        if v1 == v {
            break;
        }
        while ((v1 as i32).wrapping_mul(v1) <= val) && (j != 0) {
            j = (j - 1);
            v = v1;
            v1 = (v + (v >> i));
        }
        j = (j - 1);
        i = (i + 1);
    }
    return v;
}

pub fn stereo_samples(ibuf0: &mut [i16], ibuf1: &mut [i16], mut len: i32) -> i32 {
    len = (len % 16);
    let mut p0: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    let mut pc0: i32 = 0;
    let mut ps0: i32 = 0;
    let mut pc1: i32 = 0;
    let mut ps1: i32 = 0;
    let mut e: i32 = 0;
    let mut d: i32 = 0;
    let mut dc: i32 = 0;
    let mut ds: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    e = 0;
    p0 = 0;
    p1 = 0;
    i = 0;
    while i < len {
        p0 = at!(ibuf0, ((i as i32).wrapping_mul(2) + 0));
        p1 = at!(ibuf0, ((i as i32).wrapping_mul(2) + 1));
        p2 = at!(ibuf1, ((i as i32).wrapping_mul(2) + 0));
        p3 = at!(ibuf1, ((i as i32).wrapping_mul(2) + 1));
        pc0 = ((p0 + p1) >> 1);
        ps0 = (p0 - p1);
        pc1 = ((p2 + p3) >> 1);
        ps1 = (p2 - p3);
        dc = (pc0 - pc1);
        ds = (ps0 - ps1);
        ds = (ds >> 2);
        d = ((dc as i32).wrapping_mul(dc) + (ds as i32).wrapping_mul(ds));
        e = (e + d);
        i = (i + 1);
    }
    e = btac1c2_fakesqrt(e);
    if (e < 0) || (e > (1 << 30)) {
        e = (1 << 30);
    }
    return e;
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1__1_json() {
        let mut ibuf0: Vec<i16> = vec![-246i16, 24929i16, -1i16, 25087i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, -159i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16];
        let mut ibuf1: Vec<i16> = vec![24929i16, 1285i16, 24837i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -9i16, -1i16, 32767i16, -1i16, -1i16, -1i16, -1i16, -1537i16, -1i16, -1i16];
        let len: i32 = -1;
        let rc = super::stereo_samples(&mut ibuf0, &mut ibuf1, len);
        assert_eq!(rc, 0);
        assert_eq!(ibuf0, vec![-246i16, 24929i16, -1i16, 25087i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, -159i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16]);
        assert_eq!(ibuf1, vec![24929i16, 1285i16, 24837i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -9i16, -1i16, 32767i16, -1i16, -1i16, -1i16, -1i16, -1537i16, -1i16, -1i16]);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut ibuf0: Vec<i16> = vec![-246i16, -1i16, 25087i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16];
        let mut ibuf1: Vec<i16> = vec![24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, -159i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -129i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16];
        let len: i32 = -1;
        let rc = super::stereo_samples(&mut ibuf0, &mut ibuf1, len);
        assert_eq!(rc, 0);
        assert_eq!(ibuf0, vec![-246i16, -1i16, 25087i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16]);
        assert_eq!(ibuf1, vec![24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, 24929i16, -159i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16, -129i16, -1i16, -1i16, -1i16, -1i16, -1i16, -1i16]);
    }

}
