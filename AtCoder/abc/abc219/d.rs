#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        X:usize, Y:usize,
        AB:[(usize,usize);N],
    }
    let mut dp = vec![vec![vec![inf; 310]; 610]; 610];
    dp[0][0][0] = 0;
    for i in 0..N {
        let a = AB[i].0;
        let b = AB[i].1;
        for j in 0..=X {
            for k in 0..=Y {
                if dp[j][k][i] < inf {
                    dp[X.min(j + a)][Y.min(k + b)][i + 1] =
                        dp[X.min(j + a)][Y.min(k + b)][i + 1].min(dp[j][k][i] + 1);
                }
                dp[j][k][i + 1] = dp[j][k][i + 1].min(dp[j][k][i]);
            }
        }
    }

    let mut ans = dp[X][Y][N];
    if ans >= inf {
        ans = -1;
    }
    println!("{}", ans);
}
