#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
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
        N:usize,
        P:[Usize1;N-1],
        Q:usize,
        asks:[(Usize1,i64);Q],
    }
    let mut G = vec![vec![]; N];
    for (i, &p) in P.iter().enumerate() {
        G[p].push(i + 1);
    }
    let mut in_out = vec![vec![0, 0]; N];
    let mut depths = vec![0; N];
    let mut list = vec![vec![]; 5 * N];
    let mut time = 0;
    dfs(0, &mut time, &mut in_out, &mut depths, &mut list, &G);
    // println!("{:?}", in_out);

    for (u, d) in asks {
        let lst = &list[d as usize];
        // println!("{:?}", lst);
        let upper = lst
            .binary_search_by_key(&(2 * in_out[u][1]), |&val| 2 * val + 1)
            .err()
            .unwrap();
        let lower = lst
            .binary_search_by_key(&(2 * in_out[u][0]), |&val| 2 * val + 1)
            .err()
            .unwrap();
        println!("{}", upper - lower);
    }
}

fn dfs(
    s: usize,
    time: &mut i64,
    in_out: &mut Vec<Vec<i64>>,
    depths: &mut Vec<i64>,
    list: &mut Vec<Vec<i64>>,
    G: &Vec<Vec<usize>>,
) {
    in_out[s][0] = *time;
    list[depths[s] as usize].push(*time);
    *time += 1;
    for &nxt in &G[s] {
        depths[nxt] = depths[s] + 1;
        dfs(nxt, time, in_out, depths, list, G);
    }
    in_out[s][1] = *time;
    *time += 1;
}
