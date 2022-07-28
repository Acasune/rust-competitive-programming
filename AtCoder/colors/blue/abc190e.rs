#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
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
        N:usize, M:usize,
        AB:[(Usize1,Usize1);M],
        K :usize,
        C:[Usize1;K],
    }
    let mut G = vec![vec![]; N];
    for &(a, b) in &AB {
        G[a].push(b);
        G[b].push(a);
    }
    let mut dists = vec![];
    for i in 0..K {
        let c = C[i];
        let mut dist = vec![inf_u; N];
        let mut que = VecDeque::new();
        que.push_back(c);
        dist[c] = 0;
        while let Some(s) = que.pop_front() {
            for &nxt in &G[s] {
                if dist[nxt] > dist[s] + 1 {
                    dist[nxt] = dist[s] + 1;
                    que.push_back(nxt);
                }
            }
        }
        let mut save = vec![inf_u; K];
        for j in 0..K {
            save[j] = dist[C[j]];
        }
        dists.push(save);
    }
    let mut dp = vec![vec![inf_u; K]; (1 << K)];
    for i in 0..K {
        dp[1 << i][i] = 0;
    }
    for bit in 0..(1 << K) {
        for i in 0..K {
            if (bit >> i) & 1 != 1 {
                continue;
            }
            for j in 0..K {
                if (bit >> j) & 1 == 0 {
                    dp[bit | 1 << j][j] = dp[bit | 1 << j][j].min(dp[bit][i] + dists[i][j]);
                }
            }
        }
    }
    let ans = dp[(1 << K) - 1].iter().min().unwrap();
    if *ans >= inf_u {
        println!("{}", -1);
    } else {
        println!("{}", ans + 1);
    }
}
