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

pub fn rev16(mut a: i32) -> i32 {
    a = (((((a & 43690)) as u32).wrapping_shr((1) as u32) as i32) | ((a & 21845) as i32).wrapping_shl((1) as u32));
    a = (((((a & 52428)) as u32).wrapping_shr((2) as u32) as i32) | ((a & 13107) as i32).wrapping_shl((2) as u32));
    a = (((((a & 61680)) as u32).wrapping_shr((4) as u32) as i32) | ((a & 3855) as i32).wrapping_shl((4) as u32));
    a = (((((a & 65280)) as u32).wrapping_shr((8) as u32) as i32) | ((a & 255) as i32).wrapping_shl((8) as u32));
    return a;
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1__1_json() {
        let a: i32 = 580244498;
        let rc = super::rev16(a);
        assert_eq!(rc, 18475);
    }

    #[test]
    fn oracle_2__2_json() {
        let a: i32 = 1089851626;
        let rc = super::rev16(a);
        assert_eq!(rc, 22283);
    }

    #[test]
    fn oracle_3__3_json() {
        let a: i32 = 1015771191;
        let rc = super::rev16(a);
        assert_eq!(rc, 60430);
    }

    #[test]
    fn oracle_4__4_json() {
        let a: i32 = -2131136287;
        let rc = super::rev16(a);
        assert_eq!(rc, 34574);
    }

    #[test]
    fn oracle_5__5_json() {
        let a: i32 = -536124392;
        let rc = super::rev16(a);
        assert_eq!(rc, 6182);
    }

}
