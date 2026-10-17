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

pub const TFLAC_CHANNEL_INDEPENDENT: i32 = 0;
pub const TFLAC_CHANNEL_LEFT_SIDE: i32 = 1;
pub const TFLAC_CHANNEL_SIDE_RIGHT: i32 = 2;
pub const TFLAC_CHANNEL_MID_SIDE: i32 = 3;
pub const TFLAC_CHANNEL_MODE_COUNT: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct tflac {
    pub blocksize: i32,
    pub samplerate: i32,
    pub channels: i32,
    pub bitdepth: i32,
    pub channel_mode: i32,
    pub max_rice_value: i32,
    pub min_partition_order: i32,
    pub max_partition_order: i32,
    pub partition_order: i32,
    pub cur_blocksize: i32,
}

pub fn tflac_size_memory(mut blocksize: i32) -> i32 {
    return (15 as i32).wrapping_add((5 as i32).wrapping_mul(((15 as i32).wrapping_add((blocksize as i32).wrapping_mul(4)) & -16)));
}

pub fn flac_validate(t: &mut tflac) -> i32 {
    if t.blocksize < 16 {
        return -1;
    }
    if t.blocksize > 65535 {
        return -1;
    }
    if t.samplerate == 0 {
        return -1;
    }
    if t.samplerate > 655350 {
        return -1;
    }
    if t.channels == 0 {
        return -1;
    }
    if t.channels > 8 {
        return -1;
    }
    if t.bitdepth == 0 {
        return -1;
    }
    if t.bitdepth > 32 {
        return -1;
    }
    if t.channel_mode != TFLAC_CHANNEL_INDEPENDENT {
        if (t.channels != 2) || (t.bitdepth == 32) {
            t.channel_mode = TFLAC_CHANNEL_INDEPENDENT;
        }
    }
    if t.max_rice_value == 0 {
        if t.bitdepth <= 16 {
            t.max_rice_value = 14;
        } else {
            t.max_rice_value = 30;
        }
    } else if t.max_rice_value > 30 {
        return -1;
    }
    if t.max_partition_order > 15 {
        return -1;
    }
    if t.min_partition_order > t.max_partition_order {
        return -1;
    }
    t.partition_order = t.min_partition_order;
    while ((t.blocksize % (1 << (t.partition_order + 1))) == 0) && (t.partition_order < t.max_partition_order) {
        t.partition_order = (t.partition_order + 1);
    }
    t.cur_blocksize = t.blocksize;
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1__1_json() {
        let mut t = tflac::default();
        t.blocksize = 0;
        t.samplerate = 0;
        t.channels = 0;
        t.bitdepth = 0;
        t.channel_mode = 0;
        t.max_rice_value = 0;
        t.min_partition_order = 0;
        t.max_partition_order = 0;
        t.partition_order = 0;
        t.cur_blocksize = 0;
        let rc = super::flac_validate(&mut t);
        assert_eq!(rc, -1);
        assert_eq!(t.blocksize, 0);
        assert_eq!(t.samplerate, 0);
        assert_eq!(t.channels, 0);
        assert_eq!(t.bitdepth, 0);
        assert_eq!(t.channel_mode, 0);
        assert_eq!(t.max_rice_value, 0);
        assert_eq!(t.min_partition_order, 0);
        assert_eq!(t.max_partition_order, 0);
        assert_eq!(t.partition_order, 0);
        assert_eq!(t.cur_blocksize, 0);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut t = tflac::default();
        t.blocksize = 16;
        t.samplerate = 0;
        t.channels = 0;
        t.bitdepth = 0;
        t.channel_mode = 0;
        t.max_rice_value = 0;
        t.min_partition_order = 0;
        t.max_partition_order = 0;
        t.partition_order = 0;
        t.cur_blocksize = 0;
        let rc = super::flac_validate(&mut t);
        assert_eq!(rc, -1);
        assert_eq!(t.blocksize, 16);
        assert_eq!(t.samplerate, 0);
        assert_eq!(t.channels, 0);
        assert_eq!(t.bitdepth, 0);
        assert_eq!(t.channel_mode, 0);
        assert_eq!(t.max_rice_value, 0);
        assert_eq!(t.min_partition_order, 0);
        assert_eq!(t.max_partition_order, 0);
        assert_eq!(t.partition_order, 0);
        assert_eq!(t.cur_blocksize, 0);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut t = tflac::default();
        t.blocksize = 16;
        t.samplerate = 1;
        t.channels = 0;
        t.bitdepth = 0;
        t.channel_mode = 0;
        t.max_rice_value = 0;
        t.min_partition_order = 0;
        t.max_partition_order = 0;
        t.partition_order = 0;
        t.cur_blocksize = 0;
        let rc = super::flac_validate(&mut t);
        assert_eq!(rc, -1);
        assert_eq!(t.blocksize, 16);
        assert_eq!(t.samplerate, 1);
        assert_eq!(t.channels, 0);
        assert_eq!(t.bitdepth, 0);
        assert_eq!(t.channel_mode, 0);
        assert_eq!(t.max_rice_value, 0);
        assert_eq!(t.min_partition_order, 0);
        assert_eq!(t.max_partition_order, 0);
        assert_eq!(t.partition_order, 0);
        assert_eq!(t.cur_blocksize, 0);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut t = tflac::default();
        t.blocksize = 16;
        t.samplerate = 1;
        t.channels = 1;
        t.bitdepth = 0;
        t.channel_mode = 0;
        t.max_rice_value = 0;
        t.min_partition_order = 0;
        t.max_partition_order = 0;
        t.partition_order = 0;
        t.cur_blocksize = 0;
        let rc = super::flac_validate(&mut t);
        assert_eq!(rc, -1);
        assert_eq!(t.blocksize, 16);
        assert_eq!(t.samplerate, 1);
        assert_eq!(t.channels, 1);
        assert_eq!(t.bitdepth, 0);
        assert_eq!(t.channel_mode, 0);
        assert_eq!(t.max_rice_value, 0);
        assert_eq!(t.min_partition_order, 0);
        assert_eq!(t.max_partition_order, 0);
        assert_eq!(t.partition_order, 0);
        assert_eq!(t.cur_blocksize, 0);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut t = tflac::default();
        t.blocksize = 16;
        t.samplerate = 1;
        t.channels = 1;
        t.bitdepth = 1;
        t.channel_mode = 0;
        t.max_rice_value = 0;
        t.min_partition_order = 0;
        t.max_partition_order = 0;
        t.partition_order = 0;
        t.cur_blocksize = 0;
        let rc = super::flac_validate(&mut t);
        assert_eq!(rc, 0);
        assert_eq!(t.blocksize, 16);
        assert_eq!(t.samplerate, 1);
        assert_eq!(t.channels, 1);
        assert_eq!(t.bitdepth, 1);
        assert_eq!(t.channel_mode, 0);
        assert_eq!(t.max_rice_value, 14);
        assert_eq!(t.min_partition_order, 0);
        assert_eq!(t.max_partition_order, 0);
        assert_eq!(t.partition_order, 0);
        assert_eq!(t.cur_blocksize, 16);
    }

    #[test]
    fn oracle_6__6_json() {
        let mut t = tflac::default();
        t.blocksize = 16;
        t.samplerate = 1;
        t.channels = 1;
        t.bitdepth = 1;
        t.channel_mode = 0;
        t.max_rice_value = 1;
        t.min_partition_order = 0;
        t.max_partition_order = 0;
        t.partition_order = 0;
        t.cur_blocksize = 0;
        let rc = super::flac_validate(&mut t);
        assert_eq!(rc, 0);
        assert_eq!(t.blocksize, 16);
        assert_eq!(t.samplerate, 1);
        assert_eq!(t.channels, 1);
        assert_eq!(t.bitdepth, 1);
        assert_eq!(t.channel_mode, 0);
        assert_eq!(t.max_rice_value, 1);
        assert_eq!(t.min_partition_order, 0);
        assert_eq!(t.max_partition_order, 0);
        assert_eq!(t.partition_order, 0);
        assert_eq!(t.cur_blocksize, 16);
    }

}
