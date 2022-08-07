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
        N:usize,L:i64,R:i64,
        mut A:[i64;N],
    }

    let mut f = vec![0i64; N + 1];
    for i in 0..N {
        let cum = (i + 1) as i64 * L;
        f[i + 1] = cum.min(f[i] + A[i]);
    }
    A.reverse();
    let mut g = vec![0i64; N + 1];
    for i in 0..N {
        let cum = (i + 1) as i64 * R;
        g[i + 1] = cum.min(g[i] + A[i]);
    }
    g.reverse();

    let mut ans = inf_i;
    for i in 0..=N {
        ans = ans.min(f[i] + g[i]);
    }
    println!("{}", ans);
}
