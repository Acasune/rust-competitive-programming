#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize, M:usize,
        H:[i64;N],
        UV:[(Usize1,Usize1);M]
    }
    let mut edges = vec![vec![]; N];
    for (u, v) in UV {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut vals = vec![inf_i; N];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    vals[0] = 0;
    let mut ans = 0;
    while let Some((Reverse(val), s)) = heap.pop() {
        ans = ans.max(H[0] - H[s] - vals[s]);
        for &t in &edges[s] {
            let val = (H[t] - H[s]).max(0) + vals[s];
            if val < vals[t] {
                vals[t] = val;
                heap.push((Reverse(val), t));
            }
        }
    }
    println!("{}", ans);
}
