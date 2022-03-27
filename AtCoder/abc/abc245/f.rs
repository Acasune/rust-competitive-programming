#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        M:usize,
        UV:[(Usize1,Usize1);M],
    }
    let mut G = vec![vec![]; N];
    let mut cnt = vec![0; N];
    for m in 0..M {
        G[UV[m].1].push(UV[m].0);
        cnt[UV[m].0] += 1;
    }
    let mut que = VecDeque::<usize>::new();
    for i in 0..N {
        if cnt[i] == 0 {
            que.push_back(i);
        }
    }
    let mut ans = N;
    while let Some(u) = que.pop_front() {
        ans -= 1;
        for &v in &G[u] {
            cnt[v] -= 1;
            if cnt[v] == 0 {
                que.push_back(v);
            }
        }
    }
    println!("{}", ans);
}
