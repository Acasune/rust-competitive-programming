#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        A:(f64,f64),
        B:(f64,f64),
        C:(f64,f64),
        D:(f64,f64),
    }

    let a = outer(D, A, B);
    let b = outer(A, B, C);
    let c = outer(B, C, D);
    let d = outer(C, D, A);

    if ck(a) && ck(b) && ck(c) && ck(d) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

fn ck(v: f64) -> bool {
    v > 0.
}
fn outer(l: (f64, f64), m: (f64, f64), r: (f64, f64)) -> f64 {
    let (lx, ly) = l;
    let (mx, my) = m;
    let (rx, ry) = r;
    (ly - my) * (rx - mx) - (ry - my) * (lx - mx)
}
