#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        H:i64, W:i64,mut K:i64,
        x1:i64, y1:i64, x2:i64, y2:i64
    }
    // dp[0][0] : x=x2, y=y2
    // dp[0][1] : x=x2 y!=y2
    // dp[1][0] : x!=x2, y=y2
    // dp[1][1] : x!=x2 y!=y2
    let mut dp = vec![vec![0i64, 0i64]; 2];
    let x = if x1 != x2 { 1 } else { 0 };
    let y = if y1 != y2 { 1 } else { 0 };
    let md = md998244353;
    dp[x][y] = 1;
    let col = vec![1, W - 1];
    let row = vec![1, H - 1];
    while K > 0 {
        let mut dp2 = vec![vec![0i64, 0i64]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for a in 0..2 {
                    let b = if j == a { 1 } else { 0 };
                    let c = if i == a { 1 } else { 0 };
                    dp2[i][a] += dp[i][j] * (col[a] - b);
                    dp2[i][a] %= md;
                    dp2[a][j] += dp[i][j] * (row[a] - c);
                    dp2[a][j] %= md;
                }
            }
        }
        dp = dp2;
        K -= 1;
    }
    println!("{}", dp[0][0] % md);
}
