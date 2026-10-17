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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct spritebatch_sprite_t {
    pub texture_id: i32,
    pub sort_bits: i32,
}

pub fn spritebatch_internal_sprite_less_than_or_equal(a: &mut spritebatch_sprite_t, b: &mut spritebatch_sprite_t) -> i32 {
    if a.sort_bits <= b.sort_bits {
        return 1;
    }
    if (a.sort_bits == b.sort_bits) && (a.texture_id <= b.texture_id) {
        return 1;
    }
    return 0;
}

pub fn spritebatch_internal_merge_sort_iteration(a: &mut [spritebatch_sprite_t], mut lo: i32, mut split: i32, mut hi: i32, b: &mut [spritebatch_sprite_t]) -> i32 {
    let mut i: i32 = lo;
    let mut j: i32 = split;
    let mut k: i32 = lo;
    while k < hi {
        if (i < split) && ((j >= hi) || ({
        let __fp_i0 = (i) as usize;
        if __fp_i0 >= a.len() { return -100; }
        let mut __fp_e0 = a[__fp_i0];
        let __fp_i1 = (j) as usize;
        if __fp_i1 >= a.len() { return -100; }
        let mut __fp_e1 = a[__fp_i1];
        let __rc = spritebatch_internal_sprite_less_than_or_equal(&mut __fp_e0, &mut __fp_e1);
        a[__fp_i0] = __fp_e0;
        a[__fp_i1] = __fp_e1;
        __rc
    } != 0)) {
            {
                let __di = (k) as usize;
                let __si = (i) as usize;
                if __di >= b.len() || __si >= a.len() { return -100; }
                b[__di] = a[__si];
            }
            i = (i + 1);
        } else {
            {
                let __di = (k) as usize;
                let __si = (j) as usize;
                if __di >= b.len() || __si >= a.len() { return -100; }
                b[__di] = a[__si];
            }
            j = (j + 1);
        }
        k = (k + 1);
    }
    0
}

pub fn spritebatch_internal_merge_sort_recurse(b: &mut [spritebatch_sprite_t], mut lo: i32, mut hi: i32, a: &mut [spritebatch_sprite_t]) -> i32 {
    if (hi - lo) <= 1 {
        return 0;
    }
    let mut split: i32 = ((lo + hi) / 2);
    spritebatch_internal_merge_sort_recurse(a, lo, split, b);
    spritebatch_internal_merge_sort_recurse(a, split, hi, b);
    spritebatch_internal_merge_sort_iteration(b, lo, split, hi, a);
    0
}

pub fn merge_sort(a: &mut [spritebatch_sprite_t], b: &mut [spritebatch_sprite_t], mut size: i32) -> i32 {
    {
        let __n = ((1 as i32).wrapping_mul(size)) as usize;
        if __n > b.len() || __n > a.len() { return -100; }
        b[..__n].copy_from_slice(&a[..__n]);
        0
    };
    spritebatch_internal_merge_sort_recurse(b, 0, size, a);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1__1_json() {
        let mut a = vec![spritebatch_sprite_t::default(); 5];
        a[0].texture_id = 5;
        a[0].sort_bits = 5;
        a[1].texture_id = 1;
        a[1].sort_bits = 3;
        a[2].texture_id = 2;
        a[2].sort_bits = 2;
        a[3].texture_id = 3;
        a[3].sort_bits = 1;
        a[4].texture_id = 4;
        a[4].sort_bits = 4;
        let mut b = vec![spritebatch_sprite_t::default(); 5];
        let size: i32 = 5;
        let rc = super::merge_sort(&mut a, &mut b, size);
        assert_eq!(rc, 0);
        assert_eq!(a.len(), 5);
        assert_eq!(a[0].texture_id, 3);
        assert_eq!(a[0].sort_bits, 1);
        assert_eq!(a[1].texture_id, 2);
        assert_eq!(a[1].sort_bits, 2);
        assert_eq!(a[2].texture_id, 1);
        assert_eq!(a[2].sort_bits, 3);
        assert_eq!(a[3].texture_id, 4);
        assert_eq!(a[3].sort_bits, 4);
        assert_eq!(a[4].texture_id, 5);
        assert_eq!(a[4].sort_bits, 5);
        assert_eq!(b.len(), 5);
        assert_eq!(b[0].texture_id, 1);
        assert_eq!(b[0].sort_bits, 3);
        assert_eq!(b[1].texture_id, 5);
        assert_eq!(b[1].sort_bits, 5);
        assert_eq!(b[2].texture_id, 3);
        assert_eq!(b[2].sort_bits, 1);
        assert_eq!(b[3].texture_id, 2);
        assert_eq!(b[3].sort_bits, 2);
        assert_eq!(b[4].texture_id, 4);
        assert_eq!(b[4].sort_bits, 4);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut a = vec![spritebatch_sprite_t::default(); 0];
        let mut b = vec![spritebatch_sprite_t::default(); 0];
        let size: i32 = 0;
        let rc = super::merge_sort(&mut a, &mut b, size);
        assert_eq!(rc, 0);
        assert_eq!(a.len(), 0);
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut a = vec![spritebatch_sprite_t::default(); 5];
        a[0].texture_id = 15;
        a[0].sort_bits = 7;
        a[1].texture_id = 3;
        a[1].sort_bits = 7;
        a[2].texture_id = 33;
        a[2].sort_bits = 7;
        a[3].texture_id = 1;
        a[3].sort_bits = 7;
        a[4].texture_id = 14;
        a[4].sort_bits = 7;
        let mut b = vec![spritebatch_sprite_t::default(); 5];
        let size: i32 = 5;
        let rc = super::merge_sort(&mut a, &mut b, size);
        assert_eq!(rc, 0);
        assert_eq!(a.len(), 5);
        assert_eq!(a[0].texture_id, 15);
        assert_eq!(a[0].sort_bits, 7);
        assert_eq!(a[1].texture_id, 3);
        assert_eq!(a[1].sort_bits, 7);
        assert_eq!(a[2].texture_id, 33);
        assert_eq!(a[2].sort_bits, 7);
        assert_eq!(a[3].texture_id, 1);
        assert_eq!(a[3].sort_bits, 7);
        assert_eq!(a[4].texture_id, 14);
        assert_eq!(a[4].sort_bits, 7);
        assert_eq!(b.len(), 5);
        assert_eq!(b[0].texture_id, 15);
        assert_eq!(b[0].sort_bits, 7);
        assert_eq!(b[1].texture_id, 3);
        assert_eq!(b[1].sort_bits, 7);
        assert_eq!(b[2].texture_id, 33);
        assert_eq!(b[2].sort_bits, 7);
        assert_eq!(b[3].texture_id, 1);
        assert_eq!(b[3].sort_bits, 7);
        assert_eq!(b[4].texture_id, 14);
        assert_eq!(b[4].sort_bits, 7);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut a = vec![spritebatch_sprite_t::default(); 5];
        a[0].texture_id = 15;
        a[0].sort_bits = 5;
        a[1].texture_id = 3;
        a[1].sort_bits = 4;
        a[2].texture_id = 33;
        a[2].sort_bits = 3;
        a[3].texture_id = 1;
        a[3].sort_bits = 2;
        a[4].texture_id = 14;
        a[4].sort_bits = 1;
        let mut b = vec![spritebatch_sprite_t::default(); 5];
        let size: i32 = 5;
        let rc = super::merge_sort(&mut a, &mut b, size);
        assert_eq!(rc, 0);
        assert_eq!(a.len(), 5);
        assert_eq!(a[0].texture_id, 14);
        assert_eq!(a[0].sort_bits, 1);
        assert_eq!(a[1].texture_id, 1);
        assert_eq!(a[1].sort_bits, 2);
        assert_eq!(a[2].texture_id, 33);
        assert_eq!(a[2].sort_bits, 3);
        assert_eq!(a[3].texture_id, 3);
        assert_eq!(a[3].sort_bits, 4);
        assert_eq!(a[4].texture_id, 15);
        assert_eq!(a[4].sort_bits, 5);
        assert_eq!(b.len(), 5);
        assert_eq!(b[0].texture_id, 3);
        assert_eq!(b[0].sort_bits, 4);
        assert_eq!(b[1].texture_id, 15);
        assert_eq!(b[1].sort_bits, 5);
        assert_eq!(b[2].texture_id, 14);
        assert_eq!(b[2].sort_bits, 1);
        assert_eq!(b[3].texture_id, 1);
        assert_eq!(b[3].sort_bits, 2);
        assert_eq!(b[4].texture_id, 33);
        assert_eq!(b[4].sort_bits, 3);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut a = vec![spritebatch_sprite_t::default(); 20];
        a[0].texture_id = 16;
        a[0].sort_bits = 800;
        a[1].texture_id = 4;
        a[1].sort_bits = 16000;
        a[2].texture_id = 34;
        a[2].sort_bits = 0;
        a[3].texture_id = 2;
        a[3].sort_bits = 22;
        a[4].texture_id = 15;
        a[4].sort_bits = 16000;
        a[5].texture_id = 15;
        a[5].sort_bits = 800;
        a[6].texture_id = 3;
        a[6].sort_bits = 16000;
        a[7].texture_id = 33;
        a[7].sort_bits = 0;
        a[8].texture_id = 1;
        a[8].sort_bits = 22;
        a[9].texture_id = 14;
        a[9].sort_bits = 16000;
        a[10].texture_id = 14;
        a[10].sort_bits = 800;
        a[11].texture_id = 2;
        a[11].sort_bits = 16000;
        a[12].texture_id = 32;
        a[12].sort_bits = 0;
        a[13].texture_id = 0;
        a[13].sort_bits = 22;
        a[14].texture_id = 13;
        a[14].sort_bits = 16000;
        a[15].texture_id = 16;
        a[15].sort_bits = 800;
        a[16].texture_id = 4;
        a[16].sort_bits = 16000;
        a[17].texture_id = 34;
        a[17].sort_bits = 0;
        a[18].texture_id = 2;
        a[18].sort_bits = 22;
        a[19].texture_id = 15;
        a[19].sort_bits = 16000;
        let mut b = vec![spritebatch_sprite_t::default(); 20];
        let size: i32 = 20;
        let rc = super::merge_sort(&mut a, &mut b, size);
        assert_eq!(rc, 0);
        assert_eq!(a.len(), 20);
        assert_eq!(a[0].texture_id, 34);
        assert_eq!(a[0].sort_bits, 0);
        assert_eq!(a[1].texture_id, 33);
        assert_eq!(a[1].sort_bits, 0);
        assert_eq!(a[2].texture_id, 32);
        assert_eq!(a[2].sort_bits, 0);
        assert_eq!(a[3].texture_id, 34);
        assert_eq!(a[3].sort_bits, 0);
        assert_eq!(a[4].texture_id, 2);
        assert_eq!(a[4].sort_bits, 22);
        assert_eq!(a[5].texture_id, 1);
        assert_eq!(a[5].sort_bits, 22);
        assert_eq!(a[6].texture_id, 0);
        assert_eq!(a[6].sort_bits, 22);
        assert_eq!(a[7].texture_id, 2);
        assert_eq!(a[7].sort_bits, 22);
        assert_eq!(a[8].texture_id, 16);
        assert_eq!(a[8].sort_bits, 800);
        assert_eq!(a[9].texture_id, 15);
        assert_eq!(a[9].sort_bits, 800);
        assert_eq!(a[10].texture_id, 14);
        assert_eq!(a[10].sort_bits, 800);
        assert_eq!(a[11].texture_id, 16);
        assert_eq!(a[11].sort_bits, 800);
        assert_eq!(a[12].texture_id, 4);
        assert_eq!(a[12].sort_bits, 16000);
        assert_eq!(a[13].texture_id, 15);
        assert_eq!(a[13].sort_bits, 16000);
        assert_eq!(a[14].texture_id, 3);
        assert_eq!(a[14].sort_bits, 16000);
        assert_eq!(a[15].texture_id, 14);
        assert_eq!(a[15].sort_bits, 16000);
        assert_eq!(a[16].texture_id, 2);
        assert_eq!(a[16].sort_bits, 16000);
        assert_eq!(a[17].texture_id, 13);
        assert_eq!(a[17].sort_bits, 16000);
        assert_eq!(a[18].texture_id, 4);
        assert_eq!(a[18].sort_bits, 16000);
        assert_eq!(a[19].texture_id, 15);
        assert_eq!(a[19].sort_bits, 16000);
        assert_eq!(b.len(), 20);
        assert_eq!(b[0].texture_id, 34);
        assert_eq!(b[0].sort_bits, 0);
        assert_eq!(b[1].texture_id, 33);
        assert_eq!(b[1].sort_bits, 0);
        assert_eq!(b[2].texture_id, 2);
        assert_eq!(b[2].sort_bits, 22);
        assert_eq!(b[3].texture_id, 1);
        assert_eq!(b[3].sort_bits, 22);
        assert_eq!(b[4].texture_id, 16);
        assert_eq!(b[4].sort_bits, 800);
        assert_eq!(b[5].texture_id, 15);
        assert_eq!(b[5].sort_bits, 800);
        assert_eq!(b[6].texture_id, 4);
        assert_eq!(b[6].sort_bits, 16000);
        assert_eq!(b[7].texture_id, 15);
        assert_eq!(b[7].sort_bits, 16000);
        assert_eq!(b[8].texture_id, 3);
        assert_eq!(b[8].sort_bits, 16000);
        assert_eq!(b[9].texture_id, 14);
        assert_eq!(b[9].sort_bits, 16000);
        assert_eq!(b[10].texture_id, 32);
        assert_eq!(b[10].sort_bits, 0);
        assert_eq!(b[11].texture_id, 34);
        assert_eq!(b[11].sort_bits, 0);
        assert_eq!(b[12].texture_id, 0);
        assert_eq!(b[12].sort_bits, 22);
        assert_eq!(b[13].texture_id, 2);
        assert_eq!(b[13].sort_bits, 22);
        assert_eq!(b[14].texture_id, 14);
        assert_eq!(b[14].sort_bits, 800);
        assert_eq!(b[15].texture_id, 16);
        assert_eq!(b[15].sort_bits, 800);
        assert_eq!(b[16].texture_id, 2);
        assert_eq!(b[16].sort_bits, 16000);
        assert_eq!(b[17].texture_id, 13);
        assert_eq!(b[17].sort_bits, 16000);
        assert_eq!(b[18].texture_id, 4);
        assert_eq!(b[18].sort_bits, 16000);
        assert_eq!(b[19].texture_id, 15);
        assert_eq!(b[19].sort_bits, 16000);
    }

}
