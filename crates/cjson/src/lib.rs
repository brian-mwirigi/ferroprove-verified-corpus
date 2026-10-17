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

pub struct State {
    pub g_trunc: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_trunc: 0,
        }
    }
}

pub fn hex_digit(state: &mut State, mut c: i32) -> i32 {
    if (c >= (b'0' as i32)) && (c <= (b'9' as i32)) {
        return (c - (b'0' as i32));
    }
    if (c >= (b'a' as i32)) && (c <= (b'f' as i32)) {
        return ((c - (b'a' as i32)) + 10);
    }
    if (c >= (b'A' as i32)) && (c <= (b'F' as i32)) {
        return ((c - (b'A' as i32)) + 10);
    }
    return -1;
}

pub fn hex_byte(state: &mut State, mut hi: i32, mut lo: i32) -> i32 {
    let mut h1: i32 = 0;
    let mut h2: i32 = 0;
    h1 = hex_digit(state, hi);
    h2 = hex_digit(state, lo);
    if (h1 < 0) || (h2 < 0) {
        return -1;
    }
    return ((h1 as i32).wrapping_mul(16) + h2);
}

pub fn scan_string(state: &mut State, src: & [u8], out: &mut [u8], written: &mut i32) -> i32 {
    let mut i: i32 = 0;
    let mut o: i32 = 0;
    let mut c: i32 = 0;
    let mut e: i32 = 0;
    let mut hb: i32 = 0;
    i = 0;
    o = 0;
    if (src.len() as i32) == 0 {
        return -1;
    }
    if at!(src, 0) != (b'"' as i32) {
        return -1;
    }
    i = 1;
    while (i < (src.len() as i32)) && (at!(src, i) != (b'"' as i32)) {
        c = at!(src, i);
        if c == 92 {
            e = at!(src, (i as i32).wrapping_add(1));
            if e == (b'n' as i32) {
                c = 10;
                i = (i as i32).wrapping_add(2);
            } else if e == (b't' as i32) {
                c = 9;
                i = (i as i32).wrapping_add(2);
            } else if e == 92 {
                c = e;
                i = (i as i32).wrapping_add(2);
            } else if e == (b'"' as i32) {
                c = e;
                i = (i as i32).wrapping_add(2);
            } else if e == (b'x' as i32) {
                hb = hex_byte(state, at!(src, (i as i32).wrapping_add(2)), at!(src, (i as i32).wrapping_add(3)));
                if hb < 0 {
                    state.g_trunc = (state.g_trunc + 1);
                    return -2;
                }
                c = hb;
                i = (i as i32).wrapping_add(4);
            } else {
                state.g_trunc = (state.g_trunc + 1);
                return -2;
            }
        } else {
            i = (i as i32).wrapping_add(1);
        }
        if o >= (out.len() as i32) {
            return -3;
        }
        put!(out, o, (c as u8));
        o = (o as i32).wrapping_add(1);
    }
    if i >= (src.len() as i32) {
        return -4;
    }
    if at!(src, i) != (b'"' as i32) {
        return -4;
    }
    *written = o;
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1_plain() {
        let mut state = State::new();
        let mut src: Vec<u8> = vec![34u8, 104u8, 105u8, 34u8];
        let mut out: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::scan_string(&mut state, &src, &mut out, &mut written);
        assert_eq!(rc, 0);
        assert_eq!(written, 2);
        assert_eq!(src, vec![34u8, 104u8, 105u8, 34u8]);
        assert_eq!(out, vec![104u8, 105u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_trunc, 0);
    }

    #[test]
    fn oracle_2_nl() {
        let mut state = State::new();
        let mut src: Vec<u8> = vec![34u8, 97u8, 92u8, 110u8, 34u8];
        let mut out: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::scan_string(&mut state, &src, &mut out, &mut written);
        assert_eq!(rc, 0);
        assert_eq!(written, 2);
        assert_eq!(src, vec![34u8, 97u8, 92u8, 110u8, 34u8]);
        assert_eq!(out, vec![97u8, 10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_trunc, 0);
    }

    #[test]
    fn oracle_3_hex() {
        let mut state = State::new();
        let mut src: Vec<u8> = vec![34u8, 92u8, 120u8, 52u8, 49u8, 34u8];
        let mut out: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::scan_string(&mut state, &src, &mut out, &mut written);
        assert_eq!(rc, 0);
        assert_eq!(written, 1);
        assert_eq!(src, vec![34u8, 92u8, 120u8, 52u8, 49u8, 34u8]);
        assert_eq!(out, vec![65u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_trunc, 0);
    }

    #[test]
    fn oracle_4_bad() {
        let mut state = State::new();
        let mut src: Vec<u8> = vec![34u8, 92u8, 113u8, 34u8];
        let mut out: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::scan_string(&mut state, &src, &mut out, &mut written);
        assert_eq!(rc, -2);
        assert_eq!(written, 0);
        assert_eq!(src, vec![34u8, 92u8, 113u8, 34u8]);
        assert_eq!(out, vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_trunc, 1);
    }

    #[test]
    fn oracle_5_empty() {
        let mut state = State::new();
        let mut src: Vec<u8> = vec![];
        let mut out: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::scan_string(&mut state, &src, &mut out, &mut written);
        assert_eq!(rc, -1);
        assert_eq!(written, 0);
        assert_eq!(src, vec![]);
        assert_eq!(out, vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_trunc, 0);
    }

    #[test]
    fn oracle_6_trunc() {
        let mut state = State::new();
        let mut src: Vec<u8> = vec![34u8, 92u8];
        let mut out: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::scan_string(&mut state, &src, &mut out, &mut written);
        assert_eq!(rc, -100);
    }

}
