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
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,
        UV:[(Usize1,Usize1);M],
        S:Usize1,T:Usize1
    }
    let mut edges = vec![vec![]; N];
    for (u, v) in UV {
        edges[u].push(v);
    }

    let mut visited = vec![vec![inf_u; 3]; N];
    visited[S][0] = 0;
    let mut que = VecDeque::new();
    que.push_front((S, 0));
    while let Some((s, cnt)) = que.pop_front() {
        let next_cnt = cnt + 1;
        for &v in &edges[s] {
            if visited[v][next_cnt % 3] == inf_u || next_cnt < visited[v][next_cnt % 3] {
                visited[v][next_cnt % 3] = next_cnt;
                que.push_back((v, next_cnt));
            }
        }
    }
    if visited[T][0] == inf_u {
        println!("{}", -1);
    } else {
        println!("{}", visited[T][0] / 3);
    }
}
