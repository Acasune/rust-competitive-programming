#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        H:usize, W:usize, C:i64,
        A:[[i64;W];H],
    }
    let mut dp = vec![vec![inf; W + 1]; H + 1];

    for j in 1..=H {
        for i in 1..=W {
            dp[j][i] = dp[j - 1][i].min(dp[j][i - 1]) + C;
            dp[j][i] = dp[j][i].min(A[j - 1][i - 1]);
        }
    }
    let mut XY = vec![vec![inf; W + 1]; H + 1];
    for j in 1..=H {
        for i in 1..=W {
            XY[j][i] = dp[j - 1][i].min(dp[j][i - 1]) + C + A[j - 1][i - 1];
        }
    }
    let mut ans = inf;
    for j in 1..=H {
        for i in 1..=W {
            ans = ans.min(XY[j][i]);
        }
    }

    // Next
    let mut dp = vec![vec![inf; W + 1]; H + 1];

    for j in 1..=H {
        for i in (0..W).rev() {
            dp[j][i] = dp[j - 1][i].min(dp[j][i + 1]) + C;
            dp[j][i] = dp[j][i].min(A[j - 1][i]);
        }
    }
    let mut XY = vec![vec![inf; W + 1]; H + 1];
    for j in 1..=H {
        for i in (0..W).rev() {
            XY[j][i] = dp[j - 1][i].min(dp[j][i + 1]) + C + A[j - 1][i];
        }
    }
    for j in 1..=H {
        for i in (0..W).rev() {
            ans = ans.min(XY[j][i]);
        }
    }
    println!("{}", ans);
}
