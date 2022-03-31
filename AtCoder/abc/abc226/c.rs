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
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
    }
    let mut times = vec![0; N];
    let mut edges = vec![vec![]; N];
    for i in 0..N {
        input! {
            t:usize,
            k:usize,
        }
        times[i] = t;
        input! {
            a_s:[Usize1;k],
        }
        edges[i] = a_s;
    }
    let mut visited = vec![0; N];
    let a = dfs(N - 1, inf as usize, &edges, &mut visited, &times);
    println!("{}", a);
}

fn dfs(
    s: usize,
    pre: usize,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<usize>,
    times: &Vec<usize>,
) -> usize {
    if visited[s] > 0 {
        return 0;
    }
    for &e in &edges[s] {
        if e == pre {
            continue;
        }
        visited[s] += dfs(e, s, edges, visited, times);
        // println!("{} {:?}", s, visited);
    }
    visited[s] += times[s];
    return visited[s];
}
