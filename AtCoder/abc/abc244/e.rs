#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,K:usize,S:Usize1,T:Usize1,X:Usize1,
        UV:[(Usize1,Usize1);M],
    }
    let mut edges = vec![vec![]; N];
    for &(u, v) in &UV {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut dp = vec![vec![vec![0; 2]; N]; K + 1];
    dp[0][S][0] = 1;
    for k in 1..=K {
        for i in 0..N {
            for &e in &edges[i] {
                // i もらう from e
                if i == X {
                    dp[k][i][0] += dp[k - 1][e][1];
                    dp[k][i][1] += dp[k - 1][e][0];
                } else {
                    dp[k][i][0] += dp[k - 1][e][0];
                    dp[k][i][1] += dp[k - 1][e][1];
                }
                dp[k][i][0] %= md;
                dp[k][i][1] %= md;
            }
        }
    }
    println!("{}", dp[K][T][0] % md);
}
