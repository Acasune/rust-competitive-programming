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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,mut S:usize,
        UVAB:[(Usize1,Usize1,usize,usize);M],
        CD:[(usize,usize);N]
    }
    S = S.min(3000);
    let mut G = vec![vec![]; N];
    for &(u, v, a, b) in &UVAB {
        G[u].push((v, a, b));
        G[v].push((u, a, b));
    }
    let mut ans = vec![inf_u; N];
    let mut dists = vec![vec![inf_u; 5010]; 50];
    let mut visited = vec![vec![false; 5010]; N];
    let mut heap = BinaryHeap::<Reverse<(usize, usize)>>::new();
    heap.push(Reverse((0, S)));
    dists[0][S] = 0;
    while let Some(Reverse((dst, s))) = heap.pop() {
        let now = s / 5010;
        let rem = s % 5010;
        if visited[now][rem] {
            continue;
        }
        visited[now][rem] = true;
        ans[now] = ans[now].min(dst);
        for &(nxt, a, b) in &G[now] {
            if rem < a {
                continue;
            }
            if dists[nxt][rem - a] > dst + b {
                dists[nxt][rem - a] = dst + b;
                heap.push(Reverse((dists[nxt][rem - a], 5010 * nxt + rem - a)));
            }
        }
        let dst2 = dst + CD[now].1;
        let s2 = 3000.min(rem + CD[now].0);
        if dists[now][s2] > dst2 {
            dists[now][s2] = dst2;
            heap.push(Reverse((dst2, 5010 * now + s2)));
        }
    }
    for i in 1..N {
        println!("{}", ans[i]);
    }
}
