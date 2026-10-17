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

pub fn hsv_to_rgb(dest: &mut [f32], src: & [f32]) -> i32 {
    let mut r: f32 = (0.0 as f32);
    let mut g: f32 = (0.0 as f32);
    let mut b: f32 = (0.0 as f32);
    let mut f: f32 = (0.0 as f32);
    let mut p: f32 = (0.0 as f32);
    let mut q: f32 = (0.0 as f32);
    let mut t: f32 = (0.0 as f32);
    let mut h: f32 = (match src.get((0) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut s: f32 = (match src.get((1) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut v: f32 = (match src.get((2) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut i: i32 = 0;
    if s == (0 as f32) {
        put!(dest, 0, (v as f32));
        put!(dest, 1, (v as f32));
        put!(dest, 2, (v as f32));
        return 0;
    }
    h = (h / 60.0f32);
    i = ((h as f32).floor() as i32);
    f = (h - (i as f32));
    p = (v * ((1 as f32) - s));
    q = (v * ((1 as f32) - (s * f)));
    t = (v * ((1 as f32) - (s * ((1 as f32) - f))));
    if i == 0 {
        r = v;
        g = t;
        b = p;
    } else if i == 1 {
        r = q;
        g = v;
        b = p;
    } else if i == 2 {
        r = p;
        g = v;
        b = t;
    } else if i == 3 {
        r = p;
        g = q;
        b = v;
    } else if i == 4 {
        r = t;
        g = p;
        b = v;
    } else {
        r = v;
        g = p;
        b = q;
    }
    put!(dest, 0, (r as f32));
    put!(dest, 1, (g as f32));
    put!(dest, 2, (b as f32));
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
        let mut src: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 0.0f32);
        close_f32(dest[1], 0.0f32);
        close_f32(dest[2], 0.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 0.0f32);
        close_f32(src[1], 0.0f32);
        close_f32(src[2], 0.0f32);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![180.0f32, 0.0f32, 0.5f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 0.5f32);
        close_f32(dest[1], 0.5f32);
        close_f32(dest[2], 0.5f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 180.0f32);
        close_f32(src[1], 0.0f32);
        close_f32(src[2], 0.5f32);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![360.0f32, 0.0f32, 1.0f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 1.0f32);
        close_f32(dest[1], 1.0f32);
        close_f32(dest[2], 1.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 360.0f32);
        close_f32(src[1], 0.0f32);
        close_f32(src[2], 1.0f32);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![90.0f32, 0.0f32, 0.25f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 0.25f32);
        close_f32(dest[1], 0.25f32);
        close_f32(dest[2], 0.25f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 90.0f32);
        close_f32(src[1], 0.0f32);
        close_f32(src[2], 0.25f32);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![270.0f32, 0.0f32, 0.75f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 0.75f32);
        close_f32(dest[1], 0.75f32);
        close_f32(dest[2], 0.75f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 270.0f32);
        close_f32(src[1], 0.0f32);
        close_f32(src[2], 0.75f32);
    }

    #[test]
    fn oracle_6__6_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![0.0f32, 1.0f32, 1.0f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 1.0f32);
        close_f32(dest[1], 0.0f32);
        close_f32(dest[2], 0.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 0.0f32);
        close_f32(src[1], 1.0f32);
        close_f32(src[2], 1.0f32);
    }

    #[test]
    fn oracle_7__7_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![30.0f32, 0.800000011920929f32, 0.6000000238418579f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 0.6000000238418579f32);
        close_f32(dest[1], 0.36000001430511475f32);
        close_f32(dest[2], 0.11999999731779099f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 30.0f32);
        close_f32(src[1], 0.800000011920929f32);
        close_f32(src[2], 0.6000000238418579f32);
    }

    #[test]
    fn oracle_8__8_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![59.900001525878906f32, 0.699999988079071f32, 0.8999999761581421f32];
        let rc = super::hsv_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 0.8999999761581421f32);
        close_f32(dest[1], 0.8989499807357788f32);
        close_f32(dest[2], 0.27000001072883606f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 59.900001525878906f32);
        close_f32(src[1], 0.699999988079071f32);
        close_f32(src[2], 0.8999999761581421f32);
    }

}
