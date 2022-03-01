#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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
        N:usize,M:usize,
        A:[usize;N],
        B:[usize;M],
    }
    let inf = 1_000_000_000_000_000_000i64;
    let mut dp = vec![vec![inf; M + 1]; N + 1];
    dp[0][0] = 0;

    for i in 1..=N {
        dp[i][0] = i as i64;
    }
    for j in 1..M {
        dp[0][j] = j as i64;
    }

    for i in 1..=N {
        for j in 1..=M {
            dp[i][j] = dp[i - 1][j - 1] + if A[i - 1] == B[j - 1] { 0 } else { 1 };
            dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
        }
    }
    println!("{}", dp[N][M]);
}
