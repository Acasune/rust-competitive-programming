#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,
        AB:[(Usize1,Usize1);M],
    }
    let mut edges = vec![vec![]; N];
    for &(a, b) in &AB {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut dist = vec![inf; N];
    let mut cnts = vec![0; N];
    let mut que = VecDeque::<(usize, usize)>::new();
    que.push_back((0, 0));
    cnts[0] = 1;
    dist[0] = 0;
    while !que.is_empty() {
        let (s, c) = que.pop_front().unwrap();
        let cnt = dist[s];
        for &g in &edges[s] {
            if dist[g] > cnt + 1 {
                cnts[g] = cnts[s];
                dist[g] = cnt + 1;
                que.push_back((g, 0));
            } else if dist[g] == cnt + 1 {
                cnts[g] += cnts[s];
                cnts[g] %= md;
            }
        }
    }

    println!("{}", cnts[N - 1]);
}
