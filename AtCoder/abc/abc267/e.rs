#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
        N:usize,M:usize,
        A:[usize;N],
        UV:[(Usize1,Usize1);M]
    }
    let mut G = vec![vec![]; N];
    for (i, j) in UV {
        G[i].push(j);
        G[j].push(i);
    }
    let mut heap = BinaryHeap::<(Reverse<usize>, usize)>::new();
    for i in 0..N {
        let mut sum = 0;
        for j in &G[i] {
            sum += A[*j];
        }
        heap.push((Reverse(sum), i));
    }

    let (Reverse(x), s) = heap.pop().unwrap();
    let mut dists = vec![inf_u; N];
    dists[s] = x;
    heap.push((Reverse(x), s));

    while let Some((Reverse(x), s)) = heap.pop() {
        if dists[s] < x {
            continue;
        }
        dists[s] = x;
        for m in &G[s] {
            if dists[*m] != inf_u {
                continue;
            }
            let mut sum = 0;
            for j in &G[*m] {
                if dists[*j] == inf_u {
                    sum += A[*j];
                }
            }

            heap.push((Reverse(sum), *m));
        }
    }
    let ans = dists.into_iter().max().unwrap();
    println!("{}", ans);
}
