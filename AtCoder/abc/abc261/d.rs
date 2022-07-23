#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
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
        N:usize,
        M:usize,
        X:[usize;N],
        CY:[(usize,usize);M],
    }
    let mut mp = HashMap::new();
    for (c, y) in CY {
        mp.insert(c, y);
    }
    let size = 10010;
    let mut dp = vec![vec![0usize; size]; size];
    for j in 0..N {
        for i in 0..(N - j) {
            let bonus = mp.get(&(i + 1)).unwrap_or(&0).clone();
            dp[j][i + 1] = dp[j][i + 1].max(dp[j][i] + X[i + j] + bonus);
            dp[j + i + 1][0] = dp[j + i + 1][0].max(dp[j][i]);
        }
    }
    // println!("{:?}", dp);
    let mut ans = 0usize;
    for j in (0..=N).rev() {
        ans = ans.max(dp[j][N - j]);
        // for i in 0..=N {}
    }
    println!("{}", ans);
}
