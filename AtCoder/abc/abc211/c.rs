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
        S:Chars,
    }
    let N = S.len();
    let T = "chokudai".as_bytes();
    let mut dp = vec![[0i64; 9]; N + 1];

    dp[0][0] = 1;

    for i in 0..N {
        dp[i + 1] = dp[i];
        for j in 0..8 {
            if S[i] == T[j] as char {
                dp[i + 1][j + 1] = dp[i + 1][j + 1] + dp[i][j];
                dp[i + 1][j + 1] %= md;
            }
        }
    }

    println!("{}", dp[N][8] % md);
}
