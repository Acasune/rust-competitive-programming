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

fn main() {
    input! {
        N:usize,
        C:[usize;N],
        AB:[(usize,usize);N-1],
    }
    let C = Vec::from(C);
    let mut edges = vec![Vec::<usize>::new(); N];
    for i in 0..N - 1 {
        let a = AB[i].0 - 1;
        let b = AB[i].1 - 1;
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut used = HashMap::<usize, usize>::new();
    let mut ans = vec![];
    let mut visited = vec![false; N];
    visited[0] = true;
    *used.entry(C[0]).or_insert(0) += 1;
    ans.push(0);
    dfs(0, &edges, &mut visited, &C, &mut used, &mut ans);
    ans.sort();
    for a in ans {
        println!("{}", a + 1);
    }
}

fn dfs(
    s: usize,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    cols: &Vec<usize>,
    used: &mut HashMap<usize, usize>,
    ans: &mut Vec<usize>,
) {
    for &e in &edges[s] {
        if !visited[e] {
            let c = cols[e];
            if !used.contains_key(&c) || *used.get(&c).unwrap() == 0 {
                ans.push(e);
            }
            *used.entry(c).or_insert(0) += 1;
            visited[e] = true;
            dfs(e, edges, visited, cols, used, ans);
            *used.get_mut(&c).unwrap() -= 1;
        }
    }
}
