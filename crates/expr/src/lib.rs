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

pub struct State {
    pub g_acc: i32,
    pub g_err: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            g_acc: 0,
            g_err: 0,
        }
    }
}

pub fn commit(state: &mut State, mut v: i32) -> i32 {
    state.g_acc = v;
    return state.g_acc;
}

pub fn eval_expr(state: &mut State, s: & [u8], out: &mut i32) -> i32 {
    let mut i: i32 = 0;
    let mut acc: i32 = 0;
    let mut op: i32 = 0;
    let mut v: i32 = 0;
    i = 0;
    acc = 0;
    op = 0;
    state.g_err = 0;
    if (s.len() as i32) == 0 {
        state.g_err = (state.g_err + 1);
        return -1;
    }
    while i < (s.len() as i32) {
        v = 0;
        if (at!(s, i) < (b'0' as i32)) || (at!(s, i) > (b'9' as i32)) {
            state.g_err = (state.g_err + 1);
            return -2;
        }
        while ((i < (s.len() as i32)) && (at!(s, i) >= (b'0' as i32))) && (at!(s, i) <= (b'9' as i32)) {
            v = ((v as i32).wrapping_mul(10) + (at!(s, i) - (b'0' as i32)));
            i = (i as i32).wrapping_add(1);
        }
        if op == 0 {
            acc = v;
        } else if op == 1 {
            acc = (acc + v);
        } else {
            acc = (acc as i32).wrapping_mul(v);
        }
        if i >= (s.len() as i32) {
            *out = commit(state, acc);
            return 0;
        }
        if at!(s, i) == (b'+' as i32) {
            op = 1;
        } else if at!(s, i) == (b'*' as i32) {
            op = 2;
        } else {
            state.g_err = (state.g_err + 1);
            return -3;
        }
        i = (i as i32).wrapping_add(1);
        if i >= (s.len() as i32) {
            state.g_err = (state.g_err + 1);
            return at!(s, i);
        }
    }
    *out = commit(state, acc);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_1_add() {
        let mut state = State::new();
        let mut s: Vec<u8> = vec![49u8, 50u8, 43u8, 51u8];
        let mut out: i32 = 0;
        let rc = super::eval_expr(&mut state, &s, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(out, 15);
        assert_eq!(s, vec![49u8, 50u8, 43u8, 51u8]);
        assert_eq!(state.g_acc, 15);
        assert_eq!(state.g_err, 0);
    }

    #[test]
    fn oracle_2_mul() {
        let mut state = State::new();
        let mut s: Vec<u8> = vec![50u8, 42u8, 52u8];
        let mut out: i32 = 0;
        let rc = super::eval_expr(&mut state, &s, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(out, 8);
        assert_eq!(s, vec![50u8, 42u8, 52u8]);
        assert_eq!(state.g_acc, 8);
        assert_eq!(state.g_err, 0);
    }

    #[test]
    fn oracle_3_left() {
        let mut state = State::new();
        let mut s: Vec<u8> = vec![51u8, 43u8, 52u8, 42u8, 50u8];
        let mut out: i32 = 0;
        let rc = super::eval_expr(&mut state, &s, &mut out);
        assert_eq!(rc, 0);
        assert_eq!(out, 14);
        assert_eq!(s, vec![51u8, 43u8, 52u8, 42u8, 50u8]);
        assert_eq!(state.g_acc, 14);
        assert_eq!(state.g_err, 0);
    }

    #[test]
    fn oracle_4_empty() {
        let mut state = State::new();
        let mut s: Vec<u8> = vec![];
        let mut out: i32 = 0;
        let rc = super::eval_expr(&mut state, &s, &mut out);
        assert_eq!(rc, -1);
        assert_eq!(out, 0);
        assert_eq!(s, vec![]);
        assert_eq!(state.g_acc, 0);
        assert_eq!(state.g_err, 1);
    }

    #[test]
    fn oracle_5_bad() {
        let mut state = State::new();
        let mut s: Vec<u8> = vec![120u8];
        let mut out: i32 = 0;
        let rc = super::eval_expr(&mut state, &s, &mut out);
        assert_eq!(rc, -2);
        assert_eq!(out, 0);
        assert_eq!(s, vec![120u8]);
        assert_eq!(state.g_acc, 0);
        assert_eq!(state.g_err, 1);
    }

    #[test]
    fn oracle_6_trail() {
        let mut state = State::new();
        let mut s: Vec<u8> = vec![49u8, 50u8, 43u8];
        let mut out: i32 = 0;
        let rc = super::eval_expr(&mut state, &s, &mut out);
        assert_eq!(rc, -100);
    }

}
