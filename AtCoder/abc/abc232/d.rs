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
        H:usize,W:usize,
        maze:[Chars;H],
    }
    let mut dp = vec![vec![0i64; W]; H];
    dp[0][0] = 1;
    let mut ans = 1;
    for c in 1..W {
        if maze[0][c] == '.' {
            if dp[0][c - 1] > 0 {
                dp[0][c] = dp[0][c - 1] + 1;
                ans = ans.max(dp[0][c]);
            }
        }
    }
    for h in 1..H {
        if maze[h][0] == '.' && dp[h - 1][0] > 0 {
            dp[h][0] = dp[h - 1][0] + 1;
            ans = ans.max(dp[h][0]);
        }
        for c in 1..W {
            if maze[h][c] == '.' {
                if dp[h - 1][c] > 0 || dp[h][c - 1] > 0 {
                    dp[h][c] = i64::max(dp[h - 1][c], dp[h][c - 1]) + 1;
                    ans = ans.max(dp[h][c]);
                }
            }
        }
    }
    // dbg!(&dp);
    println!("{}", ans);
}
