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

pub const FRAME_OK: i32 = 0;
pub const ERR_SHORT: i32 = -1;
pub const ERR_MAGIC: i32 = -2;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameSummary {
    pub version: i32,
    pub flags: i32,
    pub checksum: i32,
    pub peak_bucket: i32,
}

pub struct State {
    pub g_frames: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_frames: 0,
        }
    }
}

pub fn fold_checksum(state: &mut State, mut acc: i32, mut byte_val: i32) -> i32 {
    return (((acc as i32).wrapping_mul(31) + byte_val) & 255);
}

pub fn inspect_frame(state: &mut State, pkt: & [u8], out_meta: &mut FrameSummary) -> i32 {
    let mut hist: [u8; 4] = [0u8; 4];
    let mut i: i32 = 0;
    let mut csum: i32 = 0;
    let mut b: i32 = 0;
    let mut bucket: i32 = 0;
    if (pkt.len() as i32) < 4 {
        return ERR_SHORT;
    }
    if (at!(pkt, 0) != 70) || (at!(pkt, 1) != 80) {
        return ERR_MAGIC;
    }
    out_meta.version = at!(pkt, 2);
    out_meta.flags = at!(pkt, 3);
    csum = 0;
    i = 4;
    while i < (pkt.len() as i32) {
        b = at!(pkt, i);
        csum = fold_checksum(state, csum, b);
        bucket = (b >> 5);
        put!(hist, bucket, ((at!(hist, bucket) + 1) as u8));
        i = (i as i32).wrapping_add(1);
    }
    out_meta.checksum = csum;
    out_meta.peak_bucket = (at!(hist, 0) + at!(hist, 1));
    state.g_frames = (state.g_frames + 1);
    return FRAME_OK;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1_valid() {
        let mut state = State::new();
        let mut pkt: Vec<u8> = vec![70u8, 80u8, 1u8, 3u8, 32u8, 65u8, 66u8];
        let mut out_meta = FrameSummary::default();
        let rc = super::inspect_frame(&mut state, &pkt, &mut out_meta);
        assert_eq!(rc, 0);
        assert_eq!(out_meta.version, 1);
        assert_eq!(out_meta.flags, 3);
        assert_eq!(out_meta.checksum, 65);
        assert_eq!(out_meta.peak_bucket, 1);
        assert_eq!(pkt, vec![70u8, 80u8, 1u8, 3u8, 32u8, 65u8, 66u8]);
        assert_eq!(state.g_frames, 1);
    }

    #[test]
    fn oracle_2_short() {
        let mut state = State::new();
        let mut pkt: Vec<u8> = vec![70u8, 80u8];
        let mut out_meta = FrameSummary::default();
        let rc = super::inspect_frame(&mut state, &pkt, &mut out_meta);
        assert_eq!(rc, -1);
        assert_eq!(out_meta.version, 0);
        assert_eq!(out_meta.flags, 0);
        assert_eq!(out_meta.checksum, 0);
        assert_eq!(out_meta.peak_bucket, 0);
        assert_eq!(pkt, vec![70u8, 80u8]);
        assert_eq!(state.g_frames, 0);
    }

    #[test]
    fn oracle_3_magic() {
        let mut state = State::new();
        let mut pkt: Vec<u8> = vec![0u8, 80u8, 1u8, 0u8];
        let mut out_meta = FrameSummary::default();
        let rc = super::inspect_frame(&mut state, &pkt, &mut out_meta);
        assert_eq!(rc, -2);
        assert_eq!(out_meta.version, 0);
        assert_eq!(out_meta.flags, 0);
        assert_eq!(out_meta.checksum, 0);
        assert_eq!(out_meta.peak_bucket, 0);
        assert_eq!(pkt, vec![0u8, 80u8, 1u8, 0u8]);
        assert_eq!(state.g_frames, 0);
    }

    #[test]
    fn oracle_4_stack_oob() {
        let mut state = State::new();
        let mut pkt: Vec<u8> = vec![70u8, 80u8, 1u8, 0u8, 224u8];
        let mut out_meta = FrameSummary::default();
        let rc = super::inspect_frame(&mut state, &pkt, &mut out_meta);
        assert_eq!(rc, -100);
    }

}
