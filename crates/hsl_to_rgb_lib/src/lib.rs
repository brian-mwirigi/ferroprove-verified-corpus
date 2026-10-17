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

pub fn hsl_to_rgb(dest: &mut [f32], src: & [f32]) -> i32 {
    let mut h: f32 = (match src.get((0) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut s: f32 = (match src.get((1) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut l: f32 = (match src.get((2) as usize) { Some(__v) => *__v, None => return -100 as _ } as f32);
    let mut c: f32 = (0.0 as f32);
    let mut m: f32 = (0.0 as f32);
    let mut x: f32 = (0.0 as f32);
    if s == (0 as f32) {
        put!(dest, 0, (l as f32));
        put!(dest, 1, (l as f32));
        put!(dest, 2, (l as f32));
        return 0;
    }
    c = ((1.0f32 - (((2.0f32 * l) - 1.0f32) as f32).abs()) * s);
    m = (1.0f32 * (l - (0.5f32 * c)));
    x = (c * (1.0f32 - (((((h / 60.0f32) as f32) % (2 as f32)) - 1.0f32) as f32).abs()));
    if (h >= 0.0f32) && (h < 60.0f32) {
        put!(dest, 0, ((c + m) as f32));
        put!(dest, 1, ((x + m) as f32));
        put!(dest, 2, (m as f32));
    } else if (h >= 60.0f32) && (h < 120.0f32) {
        put!(dest, 0, ((x + m) as f32));
        put!(dest, 1, ((c + m) as f32));
        put!(dest, 2, (m as f32));
    } else if (h < 120.0f32) && (h < 180.0f32) {
        put!(dest, 0, (m as f32));
        put!(dest, 1, ((c + m) as f32));
        put!(dest, 2, ((x + m) as f32));
    } else if (h >= 180.0f32) && (h < 240.0f32) {
        put!(dest, 0, (m as f32));
        put!(dest, 1, ((x + m) as f32));
        put!(dest, 2, ((c + m) as f32));
    } else if (h >= 240.0f32) && (h < 300.0f32) {
        put!(dest, 0, ((x + m) as f32));
        put!(dest, 1, (m as f32));
        put!(dest, 2, ((c + m) as f32));
    } else if (h >= 300.0f32) && (h < 360.0f32) {
        put!(dest, 0, ((c + m) as f32));
        put!(dest, 1, (m as f32));
        put!(dest, 2, ((x + m) as f32));
    } else {
        put!(dest, 0, (m as f32));
        put!(dest, 1, (m as f32));
        put!(dest, 2, (m as f32));
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
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let rc = super::hsl_to_rgb(&mut dest, &src);
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
        let rc = super::hsl_to_rgb(&mut dest, &src);
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
        let rc = super::hsl_to_rgb(&mut dest, &src);
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
        let rc = super::hsl_to_rgb(&mut dest, &src);
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
        let rc = super::hsl_to_rgb(&mut dest, &src);
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
        let mut src: Vec<f32> = vec![0.0f32, 1.0f32, 0.5f32];
        let rc = super::hsl_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 1.0f32);
        close_f32(dest[1], 0.0f32);
        close_f32(dest[2], 0.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 0.0f32);
        close_f32(src[1], 1.0f32);
        close_f32(src[2], 0.5f32);
    }

    #[test]
    fn oracle_7__7_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![30.0f32, 1.0f32, 0.5f32];
        let rc = super::hsl_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 1.0f32);
        close_f32(dest[1], 0.5f32);
        close_f32(dest[2], 0.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 30.0f32);
        close_f32(src[1], 1.0f32);
        close_f32(src[2], 0.5f32);
    }

    #[test]
    fn oracle_8__8_json() {
        let mut dest: Vec<f32> = vec![0.0f32, 0.0f32, 0.0f32];
        let mut src: Vec<f32> = vec![59.900001525878906f32, 1.0f32, 0.5f32];
        let rc = super::hsl_to_rgb(&mut dest, &src);
        assert_eq!(rc, 0);
        assert_eq!(dest.len(), 3);
        close_f32(dest[0], 1.0f32);
        close_f32(dest[1], 0.9983333349227905f32);
        close_f32(dest[2], 0.0f32);
        assert_eq!(src.len(), 3);
        close_f32(src[0], 59.900001525878906f32);
        close_f32(src[1], 1.0f32);
        close_f32(src[2], 0.5f32);
    }

}
