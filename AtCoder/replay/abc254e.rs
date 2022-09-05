#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

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
        AB:[(usize,usize);M],
        Q:usize,
        XK:[(usize,usize);Q]
    }
    let mut G = vec![vec![]; N + 1];
    for (a, b) in AB {
        G[a].push(b);
        G[b].push(a);
    }
    let mut memo = vec![vec![0; 4]; N + 1];
    for i in 1..=N {
        let mut que = VecDeque::new();
        let mut visited = vec![false; N + 1];
        memo[i][0] = i;
        visited[i] = true;
        que.push_back((i, 0));
        while let Some((s, k)) = que.pop_front() {
            if k == 3 {
                continue;
            }
            for &nxt in &G[s] {
                if !visited[nxt] {
                    memo[i][k + 1] += nxt;
                    visited[nxt] = true;
                    que.push_back((nxt, k + 1));
                }
            }
        }
    }
    for q in 0..Q {
        let (x, k) = XK[q];
        let mut ans = 0;
        for i in 0..=k {
            ans += memo[x][i];
        }
        println!("{}", ans);
    }
}
