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
    pub g_bitpos: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_bitpos: 0,
        }
    }
}

pub fn bit_from_byte(state: &mut State, mut b: i32, mut bit: i32) -> i32 {
    return ((b >> bit) & 1);
}

pub fn pull_bits(state: &mut State, buf: & [u8], mut nbits: i32, out: &mut i32) -> i32 {
    let mut acc: i32 = 0;
    let mut k: i32 = 0;
    let mut bit: i32 = 0;
    let mut byte_i: i32 = 0;
    let mut b: i32 = 0;
    acc = 0;
    k = 0;
    while k < nbits {
        byte_i = (state.g_bitpos / 8);
        bit = (7 - (state.g_bitpos % 8));
        b = at!(buf, byte_i);
        acc = ((acc << 1) + bit_from_byte(state, b, bit));
        state.g_bitpos = (state.g_bitpos + 1);
        k = (k + 1);
    }
    *out = acc;
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1_three() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![176u8];
        let nbits: i32 = 3;
        let mut out: i32 = 0;
        let rc = super::pull_bits(&mut state, &buf, nbits, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(out, 5);
        assert_eq!(buf, vec![176u8]);
        assert_eq!(state.g_bitpos, 3);
    }

    #[test]
    fn oracle_2_eight() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![176u8];
        let nbits: i32 = 8;
        let mut out: i32 = 0;
        let rc = super::pull_bits(&mut state, &buf, nbits, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(out, 176);
        assert_eq!(buf, vec![176u8]);
        assert_eq!(state.g_bitpos, 8);
    }

    #[test]
    fn oracle_3_nine() {
        let mut state = State::new();
        let mut buf: Vec<u8> = vec![176u8];
        let nbits: i32 = 9;
        let mut out: i32 = 0;
        let rc = super::pull_bits(&mut state, &buf, nbits, &mut out);
        assert_eq!(rc, -100);
    }

}
