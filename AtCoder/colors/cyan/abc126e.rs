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
        N:usize,M:usize,
        XYZ:[(Usize1,Usize1,usize);M]
    }
    let mut edges = vec![vec![]; N];
    for (x, y, _) in XYZ {
        edges[x].push(y);
        edges[y].push(x);
    }
    let mut ans = 0i64;
    let mut visited = vec![false; N];
    for i in 0..N {
        if visited[i] {
            continue;
        }
        dfs(i, inf as usize, &edges, &mut visited);
        ans += 1;
    }

    println!("{}", ans);
}

fn dfs(s: usize, prev: usize, E: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    if visited[s] {
        return;
    }
    visited[s] = true;
    for &e in &E[s] {
        if e == prev || visited[e] {
            continue;
        }
        dfs(e, s, E, visited);
    }
}
