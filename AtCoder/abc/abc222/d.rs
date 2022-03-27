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
const md: usize = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        A:[usize;N],
        B:[usize;N],
    }
    let len = 3001;
    let mut dp = vec![vec![0usize; len]; len];
    for c in 0..len {
        if A[0] <= c && c <= B[0] {
            dp[0][c] += 1;
        }

        if c > 0 {
            dp[0][c] += dp[0][c - 1];
        }
    }
    // println!("{:?}", dp);
    for i in 1..N {
        for c in 0..len {
            if A[i] <= c && c <= B[i] {
                dp[i][c] = dp[i - 1][c];
            }

            if c > 0 {
                dp[i][c] += dp[i][c - 1];
            }
            dp[i][c] %= md;
        }
    }
    // println!("{:?}", dp);
    println!("{}", dp[N - 1][B[N - 1]]);
}
