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
        R:usize,C:usize,K:usize,
        mut A:[[usize;3];K],
    }
    let mut goods = vec![vec![0; C]; R];
    let mut dp = vec![vec![vec![0; 4]; C + 1]; R + 1];
    for a in A {
        goods[a[0] - 1][a[1] - 1] = a[2];
    }
    for r in 0..R {
        for c in 0..C {
            for k in (0..3).rev() {
                if goods[r][c] >= 0 {
                    dp[r][c][k + 1] = dp[r][c][k + 1].max(dp[r][c][k] + goods[r][c]);
                }
            }
            for k in 0..4 {
                if r + 1 < R {
                    dp[r + 1][c][0] = dp[r + 1][c][0].max(dp[r][c][k]);
                }
                if c + 1 < C {
                    dp[r][c + 1][k] = dp[r][c + 1][k].max(dp[r][c][k]);
                }
            }
        }
    }
    let mut ans = dp[R - 1][C - 1].iter().max().unwrap();
    println!("{}", ans);
}
