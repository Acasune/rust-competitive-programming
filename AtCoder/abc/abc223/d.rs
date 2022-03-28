#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Binary;
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize, M:usize,
        AB:[(usize,usize);M]
    }
    let mut used = vec![0usize; N + 1];
    let mut edges = vec![vec![]; N + 1];
    for &(a, b) in &AB {
        edges[a].push(b);
        used[b] += 1;
    }
    let mut s = vec![];
    for i in 1..=N {
        if used[i] == 0 {
            s.push(i);
        }
    }
    let mut heap = BinaryHeap::<Reverse<usize>>::new();
    for e in s {
        heap.push(Reverse(e));
    }
    let mut ans = vec![];
    while let Some(Reverse(e)) = heap.pop() {
        ans.push(e);
        for &nxt in &edges[e] {
            used[nxt] -= 1;
            if used[nxt] == 0 {
                heap.push(Reverse(nxt));
            }
        }
    }
    if ans.len() != N {
        println!("{}", -1);
    } else {
        print!("{}", ans[0]);
        for i in 1..N {
            print!(" {}", ans[i]);
        }
        println!("");
    }
}
