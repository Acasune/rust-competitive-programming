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
        N:usize,
        A:[Usize1;N]
    }
    let As = vec![inf_u; 1_00010];
    let mut cum = vec![];
    for i in 0..N {
        if A[i] == i {
            cum.push(i);
        }
    }
    let mut ans = 0;
    let sz = cum.len();
    for i in 0..sz {
        ans += sz - i - 1;
    }
    for j in (0..N).rev() {
        let i = A[j];
        if A[i] == j && i > j {
            ans += 1;
        }
    }
    println!("{}", ans);
}
