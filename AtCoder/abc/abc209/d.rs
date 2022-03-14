#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f32::consts::E;
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize, Q:usize,
        AB:[(Usize1,Usize1);N-1],
        CD:[(Usize1,Usize1);Q],
    }
    let mut edges = vec![Vec::<usize>::new(); N];
    for &(a, b) in &AB {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut dist = vec![inf as usize; N];
    let mut visited = vec![false; N];
    dist[0] = 0;
    visited[0] = true;
    dfs(0, &edges, &mut dist, &mut visited, 1);
    for &(c, d) in &CD {
        if (dist[c] + dist[d]) % 2 == 0 {
            println!("{}", "Town");
        } else {
            println!("{}", "Road");
        }
    }
}

fn dfs(
    s: usize,
    edges: &Vec<Vec<usize>>,
    dist: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    c: usize,
) {
    for &e in &edges[s] {
        if !visited[e] {
            dist[e] = c;
            visited[e] = true;
            dfs(e, edges, dist, visited, c + 1);
        }
    }
}
