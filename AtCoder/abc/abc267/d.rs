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
        N:usize,M:usize,
       mut A:[i64;N],
    }
    let mut dp = vec![vec![-inf_i; M + 1]; N + 1];
    A.insert(0, 0);
    for i in 1..=N {
        dp[i][1] = A[i];
    }
    for m in 1..=M {
        for i in 1..=N {
            dp[i][m] = dp[i][m].max(dp[i - 1][m]);
            dp[i][m] = dp[i][m].max(dp[i - 1][m - 1] + m as i64 * A[i]);
        }
    }

    let mut ans = -inf_i;
    for i in 1..=N {
        ans = ans.max(dp[i][M]);
    }
    println!("{}", ans);
}
