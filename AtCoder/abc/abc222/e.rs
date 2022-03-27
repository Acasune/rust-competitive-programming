#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,K:i64,
        A:[Usize1;M],
        UV:[(Usize1,Usize1);N-1],
    }
    let mut E = vec![vec![]; N];
    for i in 0..N - 1 {
        let &(u, v) = &UV[i];
        E[u].push((v, i));
        E[v].push((u, i));
    }
    let mut cnt = vec![0i64; N - 1];
    for i in 0..M - 1 {
        dfs(A[i], inf as usize, A[i + 1], &E, &mut cnt);
    }

    let sum = cnt.iter().sum::<i64>();
    if sum + K < 0 || (sum + K) % 2 == 1 {
        println!("{}", 0);
        return;
    }

    let mut dp = vec![0; 100001];
    dp[0] = 1;

    for i in 1..N {
        let c = cnt[i - 1] as usize;
        for j in (c..=100000).rev() {
            dp[j] += dp[j - c];
            dp[j] %= md;
        }
    }
    println!("{}", dp[((sum + K) as usize) / 2]);
}
fn dfs(s: usize, pre: usize, g: usize, E: &Vec<Vec<(usize, usize)>>, cnt: &mut Vec<i64>) -> bool {
    if s == g {
        return true;
    }
    for &(u, i) in &E[s] {
        if u == pre {
            continue;
        }
        if dfs(u, s, g, E, cnt) {
            cnt[i] += 1;
            return true;
        }
    }
    return false;
}
