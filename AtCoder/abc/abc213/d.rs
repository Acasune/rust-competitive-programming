#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        AB:[(Usize1,Usize1);N-1],
    }
    let mut edges = vec![vec![]; N];
    for &(a, b) in &AB {
        edges[a].push(b);
        edges[b].push(a);
    }
    for i in 0..N {
        edges[i].sort();
    }
    let mut visited = vec![false; N];

    let mut roots = vec![];
    let mut is_finish = false;
    dfs(0, &edges, &mut visited, &mut roots, &mut is_finish);
    print!("{}", roots[0]);
    for i in 1..roots.len() {
        print!(" {}", roots[i]);
    }
    println!("");
}

fn dfs(
    s: usize,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    roots: &mut Vec<usize>,
    is_finish: &mut bool,
) {
    if *is_finish {
        return;
    }
    visited[s] = true;
    roots.push(s + 1);
    let mut is_go = false;
    for &e in &edges[s] {
        if visited[e] {
            continue;
        }
        dfs(e, edges, visited, roots, is_finish);
        roots.push(s + 1);

        is_go = true;
    }
    if !is_go && s == 0 {
        roots.push(1);
        *is_finish = true;
    }
}
