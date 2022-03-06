#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        N:usize,
    }
    let md = 998244353;
    let mut dp = vec![vec![0i64; 11]; N];
    for i in 1..=9 {
        dp[0][i] = 1;
    }
    for i in 1..N {
        for j in 1..=9 {
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j] + dp[i - 1][j + 1];
            dp[i][j] %= md;
        }
    }
    println!("{}", dp[N - 1].iter().sum::<i64>() % md);
}
