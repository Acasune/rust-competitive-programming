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
        N:usize,  M:usize,  K:usize,
        UV:[(usize,usize);M]
    }
    let mut edges = vec![vec![]; N];
    for i in 0..N {
        edges[i].push(i);
    }
    for &(u, v) in &UV {
        edges[u - 1].push(v - 1);
        edges[v - 1].push(u - 1);
    }
    let mut memo = vec![vec![0i64; N]; K + 1];
    memo[0][0] = 1;
    for k in 1..K + 1 {
        let mut sum = 0i64;
        for i in 0..N {
            sum += memo[k - 1][i];
        }
        for i in 0..N {
            memo[k][i] = sum;
            for &v in &edges[i] {
                memo[k][i] -= memo[k - 1][v];
                memo[k][i] = (memo[k][i] + md) % md;
            }
        }
    }

    println!("{:?}", memo[K][0]);
}
