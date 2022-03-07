#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize, M:usize,X:usize, Y:usize,
        ABTK:[(usize,usize,i64,i64);M],
    }
    let mut edges = vec![vec![]; N];
    for &(a, b, t, k) in &ABTK {
        edges[a - 1].push((b - 1, t, k));
        edges[b - 1].push((a - 1, t, k));
    }

    println!("{}", dijkstra(X - 1, Y - 1, N, &edges));
}

fn dijkstra(s: usize, g: usize, N: usize, edges: &Vec<Vec<(usize, i64, i64)>>) -> i64 {
    let inf = std::i64::MAX / 10;
    let mut memo = vec![inf; N];
    memo[s] = 0;
    let mut que = BinaryHeap::<(i64, usize)>::new();
    que.push((0, s));
    while !que.is_empty() {
        let (cc, s) = que.pop().unwrap();
        let cc = -cc;
        if memo[s] < cc {
            continue;
        }
        for &(nxt, c, k) in &edges[s] {
            let waiting_time: i64 = (k - (memo[s] % k)) % k;
            if memo[nxt] > memo[s] + c + waiting_time {
                memo[nxt] = memo[s] + c + waiting_time;
                que.push((-(memo[s] + c + waiting_time), nxt));
            }
        }
    }
    if memo[g] == inf {
        return -1;
    } else {
        return memo[g];
    }
}
