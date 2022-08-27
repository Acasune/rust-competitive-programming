#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,K:usize
    }
    let md = md998244353 as usize;
    let mut dp = vec![vec![0; M + 2]; N];
    for m in 1..=M {
        dp[0][m] = 1;
    }
    for i in 0..N - 1 {
        for m in 1..=M {
            if K == 0 {
                dp[i + 1][1] += dp[i][m];
                dp[i + 1][1] %= md;
                dp[i + 1][M + 1] += (md - dp[i][m]) % md;
                dp[i + 1][M + 1] %= md;
            } else {
                if m >= K {
                    dp[i + 1][1] += dp[i][m];
                    dp[i + 1][1] %= md;
                    dp[i + 1][m - K + 1] += (md - dp[i][m]) % md;
                    dp[i + 1][m - K + 1] %= md;
                }
                if m + K <= M {
                    dp[i + 1][m + K] += dp[i][m];
                    dp[i + 1][m + K] %= md;
                    dp[i + 1][M + 1] += (md - dp[i][m]) % md;
                    dp[i + 1][M + 1] %= md;
                }
            }
        }
        for m in 1..=M {
            dp[i + 1][m + 1] += dp[i + 1][m];
            dp[i + 1][m + 1] %= md;
        }
    }
    let mut ans = 0;
    for m in 1..=M {
        ans += dp[N - 1][m];
        ans %= md;
    }
    println!("{}", ans);
}
