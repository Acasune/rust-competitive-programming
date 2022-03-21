#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: usize = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        S:Chars,
    }
    let mut dp = vec![vec![vec![0; 10]; 1 << 10]; N];
    let mut c = S[0] as usize - 'A' as usize;
    dp[0][1 << c][c] = 1;
    for i in 1..N {
        let c = S[i] as usize - 'A' as usize;
        for bit in 0..(1 << 10) {
            for j in 0..10 {
                // c is already used
                dp[i][bit][j] = dp[i - 1][bit][j];
                if j == c {
                    // last char is the same as c
                    dp[i][bit][j] += dp[i - 1][bit][j];
                }
                dp[i][bit][c] %= md;
            }
        }
        for bit in 0..(1 << 10) {
            if bit >> c & 1 == 1 {
                continue;
            }
            for j in 0..10 {
                dp[i][bit | (1 << c)][c] += dp[i - 1][bit][j];
                dp[i][bit | (1 << c)][c] %= md;
            }
        }
        dp[i][1 << c][c] += 1;
        dp[i][1 << c][c] %= md;
    }
    let mut ans = 0;
    for bit in 0..1 << 10 {
        for j in 0..10 {
            ans += dp[N - 1][bit][j];
            ans %= md
        }
    }
    println!("{}", ans);
}
