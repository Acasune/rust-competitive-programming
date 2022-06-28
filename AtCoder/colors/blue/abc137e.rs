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
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,P:i64,
        ABC:[(Usize1,Usize1,i64);M],
    }
    let mut edges = vec![vec![]; N];
    for (a, b, c) in ABC {
        edges[a].push((b, c - P));
    }
    let mut dists = vec![-inf_i; N];
    dists[0] = 0;
    for _ in 0..N - 1 {
        for from in 0..N {
            if dists[from] > -inf_i {
                for &(to, dist) in &edges[from] {
                    if dists[to] < dists[from] + dist {
                        dists[to] = dists[from] + dist;
                    }
                }
            }
        }
    }
    let mut memo = vec![false; N];
    for _ in 0..N {
        for from in 0..N {
            if dists[from] > -inf_i {
                for &(to, dist) in &edges[from] {
                    if dists[to] < dists[from] + dist {
                        dists[to] = dists[from] + dist;
                        memo[to] = true;
                    }
                    if memo[from] {
                        memo[to] = true;
                    }
                }
            }
        }
    }
    for i in 0..N {
        if memo[i] {
            dists[i] = inf_i;
        }
    }
    if dists[N - 1] == inf_i {
        println!("{}", -1);
    } else {
        println!("{}", dists[N - 1].max(0));
    }
}
