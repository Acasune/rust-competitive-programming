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
        mut L:Chars,
    }

    let md = md1000000007;

    let mut dp = vec![vec![0; 2]; L.len() + 1];
    dp[0][0] = 1;
    // dp[][1] means c is less than any other patterns.
    for i in 0..L.len() {
        let c = L[i];
        for j in 0..2 {
            // (a,b) = (0,0)
            if c == '1' {
                dp[i + 1][1] += dp[i][j];
                dp[i + 1][1] %= md;
            } else {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= md;
            }

            // (a,b) = (1,0) , (0,1)
            if c == '1' || j == 1 {
                dp[i + 1][j] += dp[i][j] * 2;
                dp[i + 1][j] %= md;
            }
        }
    }
    println!("{}", (dp[L.len()][0] + dp[L.len()][1]) % md);
}
