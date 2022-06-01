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
         mut N:Chars,
    }
    let n = N.len();
    let mut dp = vec![vec![inf; 2]; N.len() + 1];
    N.reverse();
    let a: i64 = N[0].to_digit(10).unwrap().into();

    // not carry
    dp[0][0] = a;
    // get debt
    dp[0][1] = 10 - a;

    for i in 1..n {
        let a: i64 = N[i].to_digit(10).unwrap().into();
        // bounds
        dp[i][0] = dp[i][0].min(dp[i - 1][0] + a);
        // pay debts beck
        dp[i][0] = dp[i][0].min(dp[i - 1][1] + a + 1);
        // get debts
        dp[i][1] = dp[i][1].min(dp[i - 1][0] + 10 - a);
        // continue to have debts
        dp[i][1] = dp[i][1].min(dp[i - 1][1] + 9 - a);
    }
    let ans = dp[n - 1][0].min(dp[n - 1][1] + 1);
    println!("{}", ans);
}
