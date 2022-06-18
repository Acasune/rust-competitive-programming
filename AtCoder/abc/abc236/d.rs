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
        N:usize,
    }
    let mut points = vec![];
    for i in 1..=2 * N - 1 {
        input! {
            A:[usize;2*N-i],
        }
        points.push(A);
    }
    let mut ans = 0;
    let mut pairs = vec![];
    let mut pairs_idx = vec![];
    for i in 0..2 * N {
        for j in 0..i {
            pairs.push((1 << i) | (1 << j));
            pairs_idx.push((j, i));
        }
    }
    let mut memo = vec![false; 2 * N];
    memo[0] = true;
    let mut ans = 0;
    for i in 1..2 * N {
        memo[i] = true;
        dfs(0, points[0][i - 1], 2, &mut memo, &points, &mut ans);
        memo[i] = false;
    }
    println!("{}", ans);
}

fn dfs(
    prev: usize,
    p: usize,
    n_pairs: usize,
    memo: &mut Vec<bool>,
    points: &Vec<Vec<usize>>,
    ans: &mut usize,
) {
    let n = memo.len();
    if n_pairs == n {
        *ans = (*ans).max(p);
        return;
    }
    for i in prev + 1..n {
        if memo[i] {
            continue;
        }
        memo[i] = true;
        for j in i + 1..n {
            if !memo[j] {
                memo[j] = true;
                dfs(i, p ^ points[i][j - i - 1], n_pairs + 2, memo, points, ans);
                memo[j] = false;
            }
        }
        memo[i] = false;
    }
}
