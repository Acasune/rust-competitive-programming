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
        N:usize,X:usize,Y:usize,Z:usize,
        A:[usize;N],
        B:[usize;N]
    }
    let mut heap = BinaryHeap::new();
    for i in 0..N {
        heap.push((A[i], Reverse(i), B[i]));
    }
    let mut ans = vec![];
    for j in 0..X {
        if let Some((a, Reverse(i), b)) = heap.pop() {
            ans.push(i + 1);
        }
    }

    let mut heap2 = BinaryHeap::new();
    while let Some((a, Reverse(i), b)) = heap.pop() {
        heap2.push((b, Reverse(i), a));
    }
    for j in 0..Y {
        if let Some((b, Reverse(i), a)) = heap2.pop() {
            ans.push(i + 1);
        }
    }

    let mut heap3 = BinaryHeap::new();
    while let Some((a, Reverse(i), b)) = heap2.pop() {
        heap3.push((a + b, Reverse(i)));
    }
    for j in 0..Z {
        if let Some((b, Reverse(i))) = heap3.pop() {
            ans.push(i + 1);
        }
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}
