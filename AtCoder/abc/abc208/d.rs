#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 0.000_000_000_001;

fn main() {
    input! {
        N:usize, M:usize,
        ABC:[(Usize1,Usize1,i64);M]
    }
    let mut dp = vec![vec![inf; N]; N];
    for &(a, b, c) in &ABC {
        dp[a][b] = c;
    }
    let mut ans = vec![vec![0; N]; N];
    for i in 0..N {
        dp[i][i] = 0;
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                if dp[i][j] != inf {
                    ans[i][j] += dp[i][j];
                }
            }
        }
    }
    let mut ret = 0;
    for i in 0..N {
        for j in 0..N {
            if ans[i][j] != inf {
                ret += ans[i][j];
            }
        }
    }
    println!("{}", ret);
}
