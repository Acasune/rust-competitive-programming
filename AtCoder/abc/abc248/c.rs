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
        N:usize,
        M:usize,
        K:usize,
    }
    let mut memo = vec![vec![0; K + 1]; N + 1];
    let md = md998244353 as usize;
    memo[0][0] = 1;
    for i in 1..=N {
        for j in 0..=K {
            for k in 1..=M {
                if j < k {
                    continue;
                }
                memo[i][j] += memo[i - 1][j - k];
                memo[i][j] %= md;
            }
        }
    }
    // println!("{:?}", memo);
    let mut ans = 0;
    for i in 1..=K {
        ans += memo[N][i];
        ans %= md;
    }
    println!("{}", ans);
}
