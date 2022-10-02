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
use superslice::*;

const inf_i: i64 = (i64::MAX / 10) * 9;
const inf_u: usize = (usize::MAX / 10) * 9;
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
        mut A:[Usize1;N]
    }
    A.sort();
    A.dedup();
    let mut ok = 0;
    let mut ng = inf_u;
    while ng - ok > 1 {
        let m = (ng + ok) / 2;
        let save = A.lower_bound(&m);
        let short = m - save;
        let sell = N - save;
        if short <= sell / 2 {
            // ok
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}
