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
    pub g_oversize: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_oversize: 0,
        }
    }
}

pub fn u8_word(state: &mut State, mut hi: i32, mut lo: i32) -> i32 {
    return ((hi << 8) + lo);
}

pub fn read_be16(state: &mut State, p: & [u8], mut off: i32) -> i32 {
    let mut hi: i32 = 0;
    let mut lo: i32 = 0;
    hi = at!(p, off);
    lo = at!(p, (off as i32).wrapping_add(1));
    return u8_word(state, hi, lo);
}

pub fn tlv_decode(state: &mut State, buf: & [u8], dst: &mut [u8], written: &mut i32) -> i32 {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut tag: i32 = 0;
    if (buf.len() as i32) < 3 {
        return -1;
    }
    tag = at!(buf, 0);
    len = read_be16(state, buf, 1);
    if len > (dst.len() as i32) {
        state.g_oversize = (state.g_oversize + 1);
        return -3;
    }
    i = 0;
    while i < len {
        put!(dst, i, (at!(buf, (i as i32).wrapping_add(3)) as u8));
        i = (i as i32).wrapping_add(1);
    }
    *written = len;
    return tag;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1_ok() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![7u8, 0u8, 2u8, 170u8, 187u8];
        let mut dst: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::tlv_decode(&mut state, &buf, &mut dst, &mut written);
        assert_eq!(rc, 7);
        assert_eq!(written, 2);
        assert_eq!(buf, vec![7u8, 0u8, 2u8, 170u8, 187u8]);
        assert_eq!(dst, vec![170u8, 187u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_oversize, 0);
    }

    #[test]
    fn oracle_2_short() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![1u8, 2u8];
        let mut dst: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::tlv_decode(&mut state, &buf, &mut dst, &mut written);
        assert_eq!(rc, -1);
        assert_eq!(written, 0);
        assert_eq!(buf, vec![1u8, 2u8]);
        assert_eq!(dst, vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(state.g_oversize, 0);
    }

    #[test]
    fn oracle_3_cap() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![1u8, 0u8, 4u8, 9u8, 9u8];
        let mut dst: Vec<u8> = vec![0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::tlv_decode(&mut state, &buf, &mut dst, &mut written);
        assert_eq!(rc, -3);
        assert_eq!(written, 0);
        assert_eq!(buf, vec![1u8, 0u8, 4u8, 9u8, 9u8]);
        assert_eq!(dst, vec![0u8, 0u8]);
        assert_eq!(state.g_oversize, 1);
    }

    #[test]
    fn oracle_4_oob() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![7u8, 0u8, 4u8];
        let mut dst: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let mut written: i32 = 0;
        let rc = super::tlv_decode(&mut state, &buf, &mut dst, &mut written);
        assert_eq!(rc, -100);
    }

}
