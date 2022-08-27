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
        N:usize,
        TXA:[(usize,usize,usize);N],
    }

    let T = 100_010usize;
    let mut snukes = vec![vec![0usize; 5]; 1_00100];
    for (t, x, a) in TXA {
        if x > 0 && t < x {
            continue;
        }
        snukes[t][x] = a;
    }

    let mut dp = vec![vec![0; 5]; T + 10];
    dp[0][0] = snukes[0][0];
    for t in 1..=T {
        for x in 0..5 {
            if t <= 4 {
                if t < x {
                    continue;
                } else if t == x {
                    if x == 0 {
                        dp[t][x] = dp[t - 1][x] + snukes[t][x];
                    } else {
                        dp[t][x] = dp[t - 1][x - 1].max(dp[t - 1][x]) + snukes[t][x];
                    }
                } else {
                    if x == 0 {
                        dp[t][x] = dp[t - 1][x].max(dp[t - 1][x + 1]) + snukes[t][x];
                    } else if x == 4 {
                        dp[t][x] = dp[t - 1][x - 1].max(dp[t - 1][x]) + snukes[t][x];
                    } else {
                        let tmp = dp[t - 1][x - 1].max(dp[t - 1][x + 1]);
                        dp[t][x] = tmp.max(dp[t - 1][x]) + snukes[t][x];
                    }
                }
            } else {
                if x == 0 {
                    dp[t][x] = dp[t - 1][x].max(dp[t - 1][x + 1]) + snukes[t][x];
                } else if x == 4 {
                    dp[t][x] = dp[t - 1][x - 1].max(dp[t - 1][x]) + snukes[t][x];
                } else {
                    let tmp = dp[t - 1][x - 1].max(dp[t - 1][x + 1]);
                    dp[t][x] = tmp.max(dp[t - 1][x]) + snukes[t][x];
                }
            }
        }
    }
    // println!("{:?}", dp[0..100].to_vec());
    let mut ans = 0;
    for x in 0..5 {
        // println!("{}", ans.max(dp[T][x]));
        ans = ans.max(dp[T][x]);
    }
    println!("{}", ans);
}
