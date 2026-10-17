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

pub fn max_size_frame(mut blocksize: i32, mut channels: i32, mut bitdepth: i32) -> i32 {
    return ((18 as i32).wrapping_add(channels) as i32).wrapping_add(((((((blocksize as i32).wrapping_mul(bitdepth) as i32).wrapping_mul((channels as i32).wrapping_mul((if channels != 2 { 1 } else { 0 }))) as i32).wrapping_add(((blocksize as i32).wrapping_mul(bitdepth) as i32).wrapping_mul((if channels == 2 { 1 } else { 0 }))) as i32).wrapping_add(((blocksize as i32).wrapping_mul((bitdepth as i32).wrapping_add((if bitdepth != 32 { 1 } else { 0 }))) as i32).wrapping_mul((if channels == 2 { 1 } else { 0 }))) as i32).wrapping_add(7) / 8));
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1__1_json() {
        let blocksize: i32 = 1024;
        let channels: i32 = 2;
        let bitdepth: i32 = 16;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 4244);
    }

    #[test]
    fn oracle_2__2_json() {
        let blocksize: i32 = 512;
        let channels: i32 = 2;
        let bitdepth: i32 = 24;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 3156);
    }

    #[test]
    fn oracle_3__3_json() {
        let blocksize: i32 = 2048;
        let channels: i32 = 2;
        let bitdepth: i32 = 8;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 4372);
    }

    #[test]
    fn oracle_4__4_json() {
        let blocksize: i32 = 4096;
        let channels: i32 = 2;
        let bitdepth: i32 = 32;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 32788);
    }

    #[test]
    fn oracle_5__5_json() {
        let blocksize: i32 = 256;
        let channels: i32 = 2;
        let bitdepth: i32 = 20;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 1332);
    }

    #[test]
    fn oracle_6__6_json() {
        let blocksize: i32 = 1024;
        let channels: i32 = 1;
        let bitdepth: i32 = 16;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 2067);
    }

    #[test]
    fn oracle_7__7_json() {
        let blocksize: i32 = 1024;
        let channels: i32 = 3;
        let bitdepth: i32 = 16;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 6165);
    }

    #[test]
    fn oracle_8__8_json() {
        let blocksize: i32 = 1024;
        let channels: i32 = 4;
        let bitdepth: i32 = 16;
        let rc = super::max_size_frame(blocksize, channels, bitdepth);
        assert_eq!(rc, 8214);
    }

}
