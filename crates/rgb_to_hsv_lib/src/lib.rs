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

pub fn rgb_to_hsv(dest: &mut [f32], src: & [f32]) -> i32 {
    let mut r: f32 = (match src.get((0) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut g: f32 = (match src.get((1) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut b: f32 = (match src.get((2) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut h: f32 = (0 as f32);
    let mut s: f32 = (0 as f32);
    let mut v: f32 = (0 as f32);
    let mut min: f32 = (r as f32);
    let mut max: f32 = (r as f32);
    let mut delta: f32 = (0.0 as f32);
    min = (if min < g { min } else { g });
    min = (if min < b { min } else { b });
    max = (if max > g { max } else { g });
    max = (if max > b { max } else { b });
    delta = (max - min);
    v = max;
    if (delta == (0 as f32)) || (max == (0 as f32)) {
        put!(dest, 0, (h as f32));
        put!(dest, 1, (s as f32));
        put!(dest, 2, (v as f32));
        return 0;
    }
    s = (delta / max);
    if r == max {
        h = ((g - b) / delta);
    } else if g == max {
        h = ((2 as f32) + ((b - r) / delta));
    } else {
        h = ((4 as f32) + ((r - g) / delta));
    }
    h = (h * (60 as f32));
    if h < (0 as f32) {
        h = (h + (360 as f32));
    }
    put!(dest, 0, (h as f32));
    put!(dest, 1, (s as f32));
    put!(dest, 2, (v as f32));
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
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![2.5580184e32f32, 1.0345711e-20f32, 28193286000000000000.0f32];
        let rc = super::rgb_to_hsv(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 360.0f32);
        close_f32(dest[1], 1.0f32);
        close_f32(dest[2], 2.5580183673180012e32f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 2.5580184e32f32);
        close_f32(src[1], 1.0345711e-20f32);
        close_f32(src[2], 28193286000000000000.0f32);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![-3.4547846e-12f32, -0.000044706365f32, 41262783000000.0f32];
        let rc = super::rgb_to_hsv(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 240.0f32);
        close_f32(dest[1], 1.0f32);
        close_f32(dest[2], 41262782611456.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], -3.4547846e-12f32);
        close_f32(src[1], -0.000044706365f32);
        close_f32(src[2], 41262783000000.0f32);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![-620678550000.0f32, -3.8153195f32, 9.080249e-21f32];
        let rc = super::rgb_to_hsv(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 180.0f32);
        close_f32(dest[1], 6.8354793550453655e31f32);
        close_f32(dest[2], 9.080249158504418e-21f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], -620678550000.0f32);
        close_f32(src[1], -3.8153195f32);
        close_f32(src[2], 9.080249e-21f32);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![-13417.771f32, -1.5067105e-8f32, -0.15898927f32];
        let rc = super::rgb_to_hsv(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 179.99928283691406f32);
        close_f32(dest[1], -890534166528.0f32);
        close_f32(dest[2], -1.506710489707075e-8f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], -13417.771f32);
        close_f32(src[1], -1.5067105e-8f32);
        close_f32(src[2], -0.15898927f32);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![2.9476963e-36f32, 8.3269427e24f32, -1.0019444e-29f32];
        let rc = super::rgb_to_hsv(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 120.0f32);
        close_f32(dest[1], 1.0f32);
        close_f32(dest[2], 8.326942708760071e24f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 2.9476963e-36f32);
        close_f32(src[1], 8.3269427e24f32);
        close_f32(src[2], -1.0019444e-29f32);
    }

}
