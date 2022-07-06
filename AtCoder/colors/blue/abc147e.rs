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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        H:usize,W:usize,
        A:[[usize;W];H],
        B:[[usize;W];H]
    }
    let mut dp = vec![vec![vec![false; 160 * 160 + 1]; W]; H];
    let mdl = (H + W) * 80;

    dp[0][0][mdl + A[0][0] - B[0][0]] = true;
    dp[0][0][mdl + B[0][0] - A[0][0]] = true;
    for h in 0..H {
        for w in 0..W {
            for k in 0..(160 * 160) as usize {
                if !dp[h][w][k] {
                    continue;
                }
                if h + 1 != H {
                    dp[h + 1][w][k + A[h + 1][w] - B[h + 1][w]] = true;
                    dp[h + 1][w][k + B[h + 1][w] - A[h + 1][w]] = true;
                }
                if w + 1 != W {
                    dp[h][w + 1][k + A[h][w + 1] - B[h][w + 1]] = true;
                    dp[h][w + 1][k + B[h][w + 1] - A[h][w + 1]] = true;
                }
            }
        }
    }
    let ans = dp[H - 1][W - 1]
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| (i as i64 - mdl as i64).abs())
        .min()
        .unwrap();
    println!("{}", ans);
}
