#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize, T:usize,
         mut AB:[(usize,i64);N]
    }
    let mut dp = vec![vec![(-inf, 0); T + 1]; N + 1];
    dp[0][0] = (0, 0);
    for i in 0..N {
        for t in 0..=T {
            if dp[i][t].0 >= 0 {
                // case: he does not eat cuisine i
                dp[i + 1][t] = max_(dp[i + 1][t], (dp[i][t].0, dp[i][t].1.max(AB[i].1)));
                if t + AB[i].0 <= T {
                    // case: he eats cuisine i
                    dp[i + 1][t + AB[i].0] = max_(dp[i + 1][t], (dp[i][t].0 + AB[i].1, dp[i][t].1));
                }
            }
        }
    }
    let mut ans = 0;
    for t in 0..T {
        ans = ans.max(dp[N][t].0 + dp[N][t].1);
    }
    println!("{}", ans);
}

fn max_(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    if a.0 + a.1 == b.0 + b.1 {
        if a.0 > b.0 {
            return a;
        }
        return b;
    }
    if a.0 + a.1 > b.0 + b.1 {
        return a;
    }
    return b;
}
