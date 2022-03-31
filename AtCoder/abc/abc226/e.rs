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
        N:usize,M:usize,
        UV:[(Usize1,Usize1);M],
    }
    let mut edges = vec![vec![]; N];
    for &(u, v) in &UV {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut visited = vec![false; N];
    let mut ans = 0i64;
    for i in 0..N {
        if visited[i] {
            continue;
        }
        let mut n_node = 0usize;
        let mut n_edge = 0usize;
        let cnt = dfs(
            i,
            inf as usize,
            &edges,
            &mut visited,
            &mut n_edge,
            &mut n_node,
        );
        if n_edge != n_node * 2 {
            println!("{}", 0);
            return;
        }
        ans += 1;
    }

    let mut ret = 1;
    for i in 0..ans {
        ret *= 2;
        ret %= md;
    }
    println!("{}", ret);
}

fn dfs(
    s: usize,
    pre: usize,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    n_edge: &mut usize,
    n_node: &mut usize,
) {
    if visited[s] {
        return;
    }
    *n_node += 1;
    *n_edge += edges[s].len();
    visited[s] = true;
    let mut cnt = 0;
    for &e in &edges[s] {
        if e == pre {
            continue;
        }
        dfs(e, s, edges, visited, n_edge, n_node);
    }
}
