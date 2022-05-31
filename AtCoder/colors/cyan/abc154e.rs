#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:Chars,
         K:usize,
    }
    let mut dp = vec![vec![vec![0; K + 2]; 2]; N.len() + 1];
    dp[0][0][0] = 1;

    for i in 0..N.len() {
        for j in 0..2 {
            for k in 0..=K {
                let a: i64 = N[i].to_digit(10).unwrap().into();
                for num in 0..10 {
                    if a < num && j == 0 {
                        continue;
                    }
                    let mut j2 = j;
                    let mut k2 = k;
                    if num < a {
                        j2 = 1;
                    }
                    if num != 0 {
                        k2 += 1;
                    }
                    dp[i + 1][j2][k2] += dp[i][j][k];
                }
            }
        }
    }
    let n = N.len();
    let ans = dp[n][0][K] + dp[n][1][K];
    println!("{}", ans);
}
