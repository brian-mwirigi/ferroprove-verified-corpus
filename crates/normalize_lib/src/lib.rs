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

pub fn normalize(dest: &mut [f32], src: & [f32], mut size: i32) -> i32 {
    let mut sum: f32 = (0.0f32 as f32);
    let mut i: i32 = 0;
    i = 0;
    while i < size {
        sum = (sum + (match src.get((i) as usize) { Some(__v) => *__v, None => return -100 as _ } * match src.get((i) as usize) { Some(__v) => *__v, None => return -100 as _ }));
        i = (i + 1);
    }
    if sum > 0.0f32 {
        sum = (1.0f32 / (sum as f32).sqrt());
        i = 0;
        while i < size {
            put!(dest, i, ((match src.get((i) as usize) { Some(__v) => *__v, None => return -100 as _ } * sum) as f32));
            i = (i + 1);
        }
    } else if dest != src {
        {
        let __n = ((size as i32).wrapping_mul(1)) as usize;
        if __n > dest.len() { return -100; }
        for __slot in &mut dest[..__n] { *__slot = (0) as f32; }
        0
    };
    }
    0
}

#[cfg(test)]
mod tests {

    fn close_f32(got: f32, want: f32) {
        let scale = got.abs().max(want.abs()).max(1.0);
        assert!((got - want).abs() <= scale * 1e-4, "got {got} want {want}");
    }
    fn close_f64(got: f64, want: f64) {
        let scale = got.abs().max(want.abs()).max(1.0);
        assert!((got - want).abs() <= scale * 1e-4, "got {got} want {want}");
    }

    #[test]
    fn oracle_1__1_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![0.1f32, 0.2f32];
        let size: i32 = 2;
        let rc = super::normalize(&mut dest, &src, size);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 2);
        close_f32(dest[0], 0.4472135901451111f32);
        close_f32(dest[1], 0.8944271802902222f32);
        assert_eq!(src.len(), 2);
        close_f32(src[0], 0.1f32);
        close_f32(src[1], 0.2f32);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut dest: Vec<f32> = vec![];
        let mut src: Vec<f32> = vec![];
        let size: i32 = 0;
        let rc = super::normalize(&mut dest, &src, size);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 0);
        assert_eq!(src.len(), 0);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![0.1f32, 0.2f32, 1000.0f32, 100000.0f32, 1000000000.0f32];
        let size: i32 = 5;
        let rc = super::normalize(&mut dest, &src, size);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 5);
        close_f32(dest[0], 9.999999439624929e-11f32);
        close_f32(dest[1], 1.9999998879249858e-10f32);
        close_f32(dest[2], 9.999999974752427e-7f32);
        close_f32(dest[3], 0.00009999999747378752f32);
        close_f32(dest[4], 1.0f32);
        assert_eq!(src.len(), 5);
        close_f32(src[0], 0.1f32);
        close_f32(src[1], 0.2f32);
        close_f32(src[2], 1000.0f32);
        close_f32(src[3], 100000.0f32);
        close_f32(src[4], 1000000000.0f32);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![-0.00021050152f32, -70.1147f32, -0.00028400574f32, -17160019000000.0f32, 1370915294.0f32, 0.44721356f32, 0.8944271f32, -0.00021050152f32, -70.1147f32, -0.00028400574f32, -17160019000000.0f32, 1370915294.0f32, 0.44721356f32, 0.8944271f32];
        let size: i32 = 14;
        let rc = super::normalize(&mut dest, &src, size);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 14);
        close_f32(dest[0], -8.674060748692364e-18f32);
        close_f32(dest[1], -2.8891912733780556e-12f32);
        close_f32(dest[2], -1.1702923182259409e-17f32);
        close_f32(dest[3], -0.7071067690849304f32);
        close_f32(dest[4], 0.000056490815040888265f32);
        close_f32(dest[5], 1.8428169079143068e-14f32);
        close_f32(dest[6], 3.6856338158286137e-14f32);
        close_f32(dest[7], -8.674060748692364e-18f32);
        close_f32(dest[8], -2.8891912733780556e-12f32);
        close_f32(dest[9], -1.1702923182259409e-17f32);
        close_f32(dest[10], -0.7071067690849304f32);
        close_f32(dest[11], 0.000056490815040888265f32);
        close_f32(dest[12], 1.8428169079143068e-14f32);
        close_f32(dest[13], 3.6856338158286137e-14f32);
        assert_eq!(src.len(), 14);
        close_f32(src[0], -0.00021050152f32);
        close_f32(src[1], -70.1147f32);
        close_f32(src[2], -0.00028400574f32);
        close_f32(src[3], -17160019000000.0f32);
        close_f32(src[4], 1370915294.0f32);
        close_f32(src[5], 0.44721356f32);
        close_f32(src[6], 0.8944271f32);
        close_f32(src[7], -0.00021050152f32);
        close_f32(src[8], -70.1147f32);
        close_f32(src[9], -0.00028400574f32);
        close_f32(src[10], -17160019000000.0f32);
        close_f32(src[11], 1370915294.0f32);
        close_f32(src[12], 0.44721356f32);
        close_f32(src[13], 0.8944271f32);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1000000000000.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32];
        let size: i32 = 25;
        let rc = super::normalize(&mut dest, &src, size);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 25);
        close_f32(dest[0], 0.0f32);
        close_f32(dest[1], 0.0f32);
        close_f32(dest[2], 0.0f32);
        close_f32(dest[3], 0.0f32);
        close_f32(dest[4], 0.0f32);
        close_f32(dest[5], 0.0f32);
        close_f32(dest[6], 0.0f32);
        close_f32(dest[7], 0.0f32);
        close_f32(dest[8], 0.0f32);
        close_f32(dest[9], 0.0f32);
        close_f32(dest[10], 0.0f32);
        close_f32(dest[11], 0.0f32);
        close_f32(dest[12], 1.0f32);
        close_f32(dest[13], 0.0f32);
        close_f32(dest[14], 0.0f32);
        close_f32(dest[15], 0.0f32);
        close_f32(dest[16], 0.0f32);
        close_f32(dest[17], 0.0f32);
        close_f32(dest[18], 0.0f32);
        close_f32(dest[19], 0.0f32);
        close_f32(dest[20], 0.0f32);
        close_f32(dest[21], 0.0f32);
        close_f32(dest[22], 0.0f32);
        close_f32(dest[23], 0.0f32);
        close_f32(dest[24], 0.0f32);
        assert_eq!(src.len(), 25);
        close_f32(src[0], 0.0f32);
        close_f32(src[1], 0.0f32);
        close_f32(src[2], 0.0f32);
        close_f32(src[3], 0.0f32);
        close_f32(src[4], 0.0f32);
        close_f32(src[5], 0.0f32);
        close_f32(src[6], 0.0f32);
        close_f32(src[7], 0.0f32);
        close_f32(src[8], 0.0f32);
        close_f32(src[9], 0.0f32);
        close_f32(src[10], 0.0f32);
        close_f32(src[11], 0.0f32);
        close_f32(src[12], 1000000000000.0f32);
        close_f32(src[13], 0.0f32);
        close_f32(src[14], 0.0f32);
        close_f32(src[15], 0.0f32);
        close_f32(src[16], 0.0f32);
        close_f32(src[17], 0.0f32);
        close_f32(src[18], 0.0f32);
        close_f32(src[19], 0.0f32);
        close_f32(src[20], 0.0f32);
        close_f32(src[21], 0.0f32);
        close_f32(src[22], 0.0f32);
        close_f32(src[23], 0.0f32);
        close_f32(src[24], 0.0f32);
    }

}
