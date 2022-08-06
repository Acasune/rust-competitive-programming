#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,mut K:i64,
    }
    let mut dp = vec![vec![0i64; 30101010]; 4];
    dp[0][0] = 1i64;
    for cnt in 0..3 {
        for tot in 0..2010101 {
            dp[cnt + 1][tot + 1] += dp[cnt][tot];
            dp[cnt + 1][tot + N + 1] -= dp[cnt][tot];
        }
        for tot in 1..3010101 {
            dp[cnt + 1][tot] += dp[cnt + 1][tot - 1];
            dp[cnt + 1][tot] = dp[cnt + 1][tot].min(inf_i);
        }
    }

    let mut cu = 1;
    let mut tot = 0;
    for i in 3..3030301 {
        if cu <= K as i64 && (K as i64) < cu + dp[3][i] {
            tot = i as i64;
            break;
        }
        cu += dp[3][i];
    }
    for i in 0..tot {
        K -= dp[3][i as usize];
    }
    cu = 1;
    let mut ans_i = 0;
    for i in 1..=N {
        if cu <= K as i64 && (K as i64) < cu + dp[2][tot as usize - i] {
            ans_i = i;
            break;
        }
        cu += dp[2][tot as usize - i]
    }
    for i in 1..ans_i {
        K -= dp[2][tot as usize - i as usize];
    }

    cu = 1;
    let mut ans_j = 0;
    for j in 1..=N {
        if cu <= K as i64 && (K as i64) < cu + dp[1][tot as usize - ans_i - j] {
            ans_j = j;
            break;
        }
        cu += dp[1][tot as usize - ans_i - j];
    }
    let ans_k = tot as usize - ans_i - ans_j;
    println!("{} {} {}", ans_i, ans_j, ans_k);
}
