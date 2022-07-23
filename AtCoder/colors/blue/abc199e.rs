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
        N:usize, M:usize,
        A:[(usize, usize, usize);M]
    }
    let mut restrictions = vec![vec![]; 20];
    for (a, b, c) in A {
        restrictions[a].push((b, c));
    }
    let mut dp = vec![0i64; (1 << N)];
    dp[0] = 1;
    for msk in 0..(1 << N) {
        for nxt in 0..N {
            if msk & (1 << nxt) != 0 {
                continue;
            }
            if check(msk + (1 << nxt), &restrictions) {
                dp[msk + (1 << nxt)] += dp[msk];
            }
        }
    }
    println!("{}", dp[(1 << N) - 1]);
}

fn check(msk: usize, restrictions: &Vec<Vec<(usize, usize)>>) -> bool {
    let mut cnts = vec![0; 20];
    let mut cnt = 0;
    for i in 0..20 {
        if msk & (1 << i) != 0 {
            cnts[i + 1] += 1;
            cnt += 1;
        }
    }
    for i in 2..20 {
        cnts[i] += cnts[i - 1];
    }
    for &(b, c) in &restrictions[cnt] {
        if c < cnts[b] {
            return false;
        }
    }

    return true;
}
