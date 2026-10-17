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
pub struct tflac_md5 {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

pub fn md5_digest(m: & tflac_md5, out: &mut [u8]) -> i32 {
    put!(out, 0, (m.a as u8));
    put!(out, 1, ((m.a >> 8) as u8));
    put!(out, 2, ((m.a >> 16) as u8));
    put!(out, 3, ((m.a >> 24) as u8));
    put!(out, 4, (m.b as u8));
    put!(out, 5, ((m.b >> 8) as u8));
    put!(out, 6, ((m.b >> 16) as u8));
    put!(out, 7, ((m.b >> 24) as u8));
    put!(out, 8, (m.c as u8));
    put!(out, 9, ((m.c >> 8) as u8));
    put!(out, 10, ((m.c >> 16) as u8));
    put!(out, 11, ((m.c >> 24) as u8));
    put!(out, 12, (m.d as u8));
    put!(out, 13, ((m.d >> 8) as u8));
    put!(out, 14, ((m.d >> 16) as u8));
    put!(out, 15, ((m.d >> 24) as u8));
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1__1_json() {
        let mut m = tflac_md5::default();
        m.a = 61539269;
        m.b = -1924970356;
        m.c = -1411181604;
        m.d = -1079261113;
        let mut out: Vec<u8> = vec![152u8, 121u8, 169u8, 198u8, 140u8, 229u8, 21u8, 65u8, 8u8, 20u8, 180u8, 38u8, 0u8, 64u8, 240u8, 162u8];
        let rc = super::md5_digest(&m, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(m.a, 61539269);
        assert_eq!(m.b, -1924970356);
        assert_eq!(m.c, -1411181604);
        assert_eq!(m.d, -1079261113);
        assert_eq!(out, vec![197u8, 3u8, 171u8, 3u8, 140u8, 72u8, 67u8, 141u8, 220u8, 19u8, 227u8, 171u8, 71u8, 200u8, 171u8, 191u8]);
    }

    #[test]
    fn oracle_2__2_json() {
        let mut m = tflac_md5::default();
        m.a = 274051987;
        m.b = 2147196113;
        m.c = 1360955111;
        m.d = 1053418827;
        let mut out: Vec<u8> = vec![134u8, 83u8, 207u8, 210u8, 232u8, 192u8, 126u8, 178u8, 202u8, 216u8, 176u8, 63u8, 199u8, 53u8, 74u8, 232u8];
        let rc = super::md5_digest(&m, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(m.a, 274051987);
        assert_eq!(m.b, 2147196113);
        assert_eq!(m.c, 1360955111);
        assert_eq!(m.d, 1053418827);
        assert_eq!(out, vec![147u8, 179u8, 85u8, 16u8, 209u8, 156u8, 251u8, 127u8, 231u8, 134u8, 30u8, 81u8, 75u8, 229u8, 201u8, 62u8]);
    }

    #[test]
    fn oracle_3__3_json() {
        let mut m = tflac_md5::default();
        m.a = 705554095;
        m.b = 1101994371;
        m.c = -94464709;
        m.d = 1880582026;
        let mut out: Vec<u8> = vec![158u8, 138u8, 56u8, 120u8, 4u8, 4u8, 39u8, 198u8, 97u8, 114u8, 175u8, 123u8, 255u8, 116u8, 192u8, 236u8];
        let rc = super::md5_digest(&m, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(m.a, 705554095);
        assert_eq!(m.b, 1101994371);
        assert_eq!(m.c, -94464709);
        assert_eq!(m.d, 1880582026);
        assert_eq!(out, vec![175u8, 230u8, 13u8, 42u8, 131u8, 25u8, 175u8, 65u8, 59u8, 149u8, 94u8, 250u8, 138u8, 103u8, 23u8, 112u8]);
    }

    #[test]
    fn oracle_4__4_json() {
        let mut m = tflac_md5::default();
        m.a = -917610122;
        m.b = -644071863;
        m.c = 1053678897;
        m.d = 1476143889;
        let mut out: Vec<u8> = vec![7u8, 224u8, 14u8, 216u8, 233u8, 100u8, 244u8, 151u8, 181u8, 242u8, 14u8, 88u8, 152u8, 113u8, 194u8, 234u8];
        let rc = super::md5_digest(&m, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(m.a, -917610122);
        assert_eq!(m.b, -644071863);
        assert_eq!(m.c, 1053678897);
        assert_eq!(m.d, 1476143889);
        assert_eq!(out, vec![118u8, 97u8, 78u8, 201u8, 73u8, 62u8, 156u8, 217u8, 49u8, 221u8, 205u8, 62u8, 17u8, 43u8, 252u8, 87u8]);
    }

    #[test]
    fn oracle_5__5_json() {
        let mut m = tflac_md5::default();
        m.a = -59823601;
        m.b = -1444458266;
        m.c = -2064024557;
        m.d = -832993048;
        let mut out: Vec<u8> = vec![63u8, 69u8, 65u8, 28u8, 102u8, 193u8, 154u8, 34u8, 251u8, 194u8, 123u8, 45u8, 175u8, 177u8, 225u8, 146u8];
        let rc = super::md5_digest(&m, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(m.a, -59823601);
        assert_eq!(m.b, -1444458266);
        assert_eq!(m.c, -2064024557);
        assert_eq!(m.d, -832993048);
        assert_eq!(out, vec![15u8, 42u8, 111u8, 252u8, 230u8, 80u8, 231u8, 169u8, 19u8, 124u8, 249u8, 132u8, 232u8, 136u8, 89u8, 206u8]);
    }

}
