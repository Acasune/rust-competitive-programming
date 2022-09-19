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

const inf_i: i64 = (i64::MAX / 10) * 9;
const inf_u: usize = (usize::MAX / 10) * 9;
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
        N:usize,X:Usize1,Y:Usize1,
        UV:[(Usize1,Usize1);N-1]
    }
    let mut G = vec![vec![]; N];
    for (u, v) in UV {
        G[u].push(v);
        G[v].push(u);
    }
    let mut stk = vec![X];
    let mut visited = vec![false; N];
    let mut ans = vec![];
    visited[X] = true;
    dfs(X, &mut stk, &mut visited, &G, &mut ans, Y);
    print!("{}", ans[0] + 1);
    for i in 1..ans.len() {
        print!(" {}", ans[i] + 1);
    }
    println!();
}

fn dfs(
    s: usize,
    stk: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    G: &Vec<Vec<usize>>,
    ans: &mut Vec<usize>,
    Y: usize,
) {
    if s == Y {
        *ans = stk.clone();
        return;
    }
    for &t in &G[s] {
        if !visited[t] {
            visited[t] = true;
            stk.push(t);
            dfs(t, stk, visited, G, ans, Y);
            stk.pop();
        }
    }
}
