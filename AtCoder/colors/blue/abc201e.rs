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
        UVW:[(Usize1,Usize1,i64);N-1],
    }
    let md = md1000000007;
    let mut G = vec![vec![]; N];
    for (u, v, w) in UVW {
        G[u].push((v, w));
        G[v].push((u, w));
    }
    let mut dists = vec![inf_i; N];
    dists[0] = 0;
    let mut que = VecDeque::new();
    que.push_back(0);
    while let Some(s) = que.pop_front() {
        for &(nxt, w) in &G[s] {
            if dists[nxt] == inf_i {
                dists[nxt] = dists[s] ^ w;
                que.push_back(nxt);
            }
        }
    }
    // println!("{:?}", G);
    let mut ans = 0;
    for i in 0..60 {
        let mut cnt = vec![0, 0];
        for j in 0..N {
            cnt[((dists[j] >> i) & 1) as usize] += 1;
        }
        ans += (1 << i) % md * cnt[0] % md * cnt[1];
        ans %= md;
    }
    println!("{}", ans);
}
