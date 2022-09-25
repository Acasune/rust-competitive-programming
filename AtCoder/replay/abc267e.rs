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
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,M:usize,
        A:[usize;N],
        UV:[(Usize1,Usize1);M],
    }
    let mut G = vec![vec![]; N];
    for (u, v) in UV {
        G[u].push(v);
        G[v].push(u);
    }
    let mut heap = BinaryHeap::new();
    for i in 0..N {
        let mut sum = 0;
        for &j in &G[i] {
            sum += A[j];
        }
        heap.push((Reverse(sum), i));
    }
    // let (Reverse(sum), s) = heap.pop().unwrap();
    let mut dis = vec![inf_u; N];
    // dis[s] = sum;
    // heap.push((Reverse(sum), s));
    while let Some((Reverse(sum), s)) = heap.pop() {
        if dis[s] <= sum {
            continue;
        }
        // println!("{} {}", s, sum);
        dis[s] = sum;

        for &t in &G[s] {
            if dis[t] != inf_u {
                continue;
            }
            let mut sum = 0;
            for &u in &G[t] {
                if dis[u] == inf_u {
                    sum += A[u];
                }
            }
            heap.push((Reverse(sum), t));
        }
    }
    let ans = dis.into_iter().max().unwrap();
    println!("{}", ans);
}
