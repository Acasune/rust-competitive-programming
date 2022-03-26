#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: usize = usize::MAX / 10;
const md: usize = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        A:[usize;N]
    }
    let mut dp = vec![vec![0usize; 10]; N];
    dp[0][A[0]] += 1;
    for i in 1..N {
        let a = A[i];
        for j in 0..10usize {
            dp[i][(j + a) % 10] += dp[i - 1][j];
            dp[i][(j + a) % 10] %= md;
            dp[i][(j * a) % 10] += dp[i - 1][j];
            dp[i][(j * a) % 10] %= md;
        }
    }
    for i in 0..10usize {
        println!("{}", dp[N - 1][i] % md);
    }
}
