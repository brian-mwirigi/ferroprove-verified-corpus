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

pub fn encode_quant(mut uni: i32, mut step: i32, mut pred: i32, mut tgt: i32, mut tgt2: i32, mut lsbit: i32) -> i32 {
    let mut uni1: i32 = 0;
    let mut uni2: i32 = 0;
    let mut diff: i32 = 0;
    let mut p0: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    let mut d0: i32 = 0;
    let mut d1: i32 = 0;
    let mut d2: i32 = 0;
    let mut d3: i32 = 0;
    uni1 = (uni + 1);
    uni2 = (uni - 1);
    if ((uni ^ uni1) & (!7)) != 0 {
        uni1 = uni;
    }
    if ((uni ^ uni2) & (!7)) != 0 {
        uni2 = uni;
    }
    if lsbit != 0 {
        if lsbit == 4 {
            uni = (uni & (!1));
            uni1 = (uni1 & (!1));
            uni2 = (uni2 & (!1));
            uni = (uni | (((uni >> 1) & (uni >> 2)) & 1));
            uni1 = (uni1 | (((uni1 >> 1) & (uni1 >> 2)) & 1));
            uni2 = (uni2 | (((uni2 >> 1) & (uni2 >> 2)) & 1));
        } else if (lsbit & 1) != 0 {
            uni = (uni | 1);
            uni1 = (uni1 | 1);
            uni2 = (uni2 | 1);
        } else {
            uni = (uni & (!1));
            uni1 = (uni1 & (!1));
            uni2 = (uni2 & (!1));
        }
    }
    diff = ((((2 as i32).wrapping_mul((uni & 7)) + 1) as i32).wrapping_mul(step) / 8);
    if (uni & 8) != 0 {
        diff = -diff;
    }
    p0 = (pred + diff);
    d0 = (tgt - p0);
    d0 = (d0 ^ (d0 >> 31));
    diff = ((((2 as i32).wrapping_mul((uni1 & 7)) + 1) as i32).wrapping_mul(step) / 8);
    if (uni1 & 8) != 0 {
        diff = -diff;
    }
    p1 = (pred + diff);
    d1 = (tgt - p1);
    d1 = (d1 ^ (d1 >> 31));
    diff = ((((2 as i32).wrapping_mul((uni2 & 7)) + 1) as i32).wrapping_mul(step) / 8);
    if (uni2 & 8) != 0 {
        diff = -diff;
    }
    p2 = (pred + diff);
    d2 = (tgt - p2);
    d2 = (d2 ^ (d2 >> 31));
    d3 = (tgt2 - p0);
    d3 = (d3 ^ (d3 >> 31));
    d0 = (d0 + (d3 >> 5));
    d3 = (tgt2 - p1);
    d3 = (d3 ^ (d3 >> 31));
    d1 = (d1 + (d3 >> 5));
    d3 = (tgt2 - p2);
    d3 = (d3 ^ (d3 >> 31));
    d2 = (d2 + (d3 >> 5));
    if d1 < d0 {
        uni = uni1;
    }
    if d2 < d0 {
        uni = uni2;
    }
    return uni;
}

#[cfg(test)]
mod tests {

    #[test]
    fn oracle_1__1_json() {
        let uni: i32 = 7;
        let step: i32 = 10;
        let pred: i32 = 100;
        let tgt: i32 = 50;
        let tgt2: i32 = 60;
        let lsbit: i32 = 0;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 6);
    }

    #[test]
    fn oracle_2__2_json() {
        let uni: i32 = 5;
        let step: i32 = 10;
        let pred: i32 = 100;
        let tgt: i32 = 50;
        let tgt2: i32 = 60;
        let lsbit: i32 = 0;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 4);
    }

    #[test]
    fn oracle_3__3_json() {
        let uni: i32 = 8;
        let step: i32 = 10;
        let pred: i32 = 100;
        let tgt: i32 = 50;
        let tgt2: i32 = 60;
        let lsbit: i32 = 0;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 9);
    }

    #[test]
    fn oracle_4__4_json() {
        let uni: i32 = 6;
        let step: i32 = 10;
        let pred: i32 = 100;
        let tgt: i32 = 50;
        let tgt2: i32 = 60;
        let lsbit: i32 = 0;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 5);
    }

    #[test]
    fn oracle_5__5_json() {
        let uni: i32 = 10;
        let step: i32 = 15;
        let pred: i32 = 200;
        let tgt: i32 = 180;
        let tgt2: i32 = 190;
        let lsbit: i32 = 0;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 11);
    }

    #[test]
    fn oracle_6__6_json() {
        let uni: i32 = 10;
        let step: i32 = 15;
        let pred: i32 = 200;
        let tgt: i32 = 180;
        let tgt2: i32 = 190;
        let lsbit: i32 = 4;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 10);
    }

    #[test]
    fn oracle_7__7_json() {
        let uni: i32 = 10;
        let step: i32 = 15;
        let pred: i32 = 200;
        let tgt: i32 = 180;
        let tgt2: i32 = 190;
        let lsbit: i32 = 1;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 11);
    }

    #[test]
    fn oracle_8__8_json() {
        let uni: i32 = 10;
        let step: i32 = 15;
        let pred: i32 = 200;
        let tgt: i32 = 180;
        let tgt2: i32 = 190;
        let lsbit: i32 = 2;
        let rc = super::encode_quant(uni, step, pred, tgt, tgt2, lsbit);
        assert_eq!(rc, 10);
    }

}
