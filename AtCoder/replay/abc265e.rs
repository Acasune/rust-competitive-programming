#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,M:usize,
       delta:[(i64,i64);3],
       mut X:[(i64,i64);M]
    }
    let md = md998244353;
    let st = X.into_iter().collect::<HashSet<(i64, i64)>>();
    let mut dp = vec![vec![vec![0; N + 1]; N + 1]; N + 1];
    dp[0][0][0] = 1;
    for i in 0..N {
        for x in 0..=i {
            for y in 0..=(i - x) {
                let z = i - x - y;
                let xx = delta[0].0 * x as i64 + delta[1].0 * y as i64 + delta[2].0 * z as i64;
                let yy = delta[0].1 * x as i64 + delta[1].1 * y as i64 + delta[2].1 * z as i64;

                let xxx = xx + delta[0].0;
                let yyy = yy + delta[0].1;
                if !st.contains(&(xxx, yyy)) {
                    dp[i + 1][x + 1][y] += dp[i][x][y];
                    dp[i + 1][x + 1][y] %= md;
                }
                let xxx = xx + delta[1].0;
                let yyy = yy + delta[1].1;
                if !st.contains(&(xxx, yyy)) {
                    dp[i + 1][x][y + 1] += dp[i][x][y];
                    dp[i + 1][x][y + 1] %= md;
                }

                let xxx = xx + delta[2].0;
                let yyy = yy + delta[2].1;
                if !st.contains(&(xxx, yyy)) {
                    dp[i + 1][x][y] += dp[i][x][y];
                    dp[i + 1][x][y] %= md;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..=N {
        for j in 0..=N {
            ans += dp[N][i][j];
            ans %= md;
        }
    }
    println!("{}", ans);
}
