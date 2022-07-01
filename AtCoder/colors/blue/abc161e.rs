#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Sub;
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
        N:usize,K:i64,C:usize,
        S:Chars,
    }
    let mut dp = vec![-inf_i; 201010];
    let mut pre = vec![vec![]; 201010];
    dp[0] = 0;
    for i in 0..N {
        if dp[i] > dp[i + 1] {
            pre[i + 1].clear();
            dp[i + 1] = dp[i];
        }
        if dp[i + 1] == dp[i] {
            pre[i + 1].push(i);
        }
        if S[i] == 'o' {
            let upper = N.min(i + 1 + C);
            if dp[i] + 1 > dp[upper] {
                pre[upper].clear();
                dp[upper] = dp[i] + 1;
            }
            if dp[upper] == dp[i] + 1 {
                pre[upper].push(i);
            }
        }
    }
    if dp[N] > K {
        return;
    }
    let mut que = VecDeque::new();
    que.push_back(N);
    let mut visited = vec![false; 201010];
    let mut dpt = vec![vec![]; 201010];
    visited[N] = true;
    while let Some(idx) = que.pop_front() {
        for &to in pre[idx].iter() {
            if !visited[to] {
                visited[to] = true;
                que.push_back(to);
            }
            if dp[idx] == dp[to] + 1 {
                dpt[dp[to] as usize].push(to + 1);
            }
        }
    }
    let mut ans = vec![];
    for i in 0..K as usize {
        if dpt[i].len() == 1 {
            ans.push(dpt[i][0]);
        }
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}
