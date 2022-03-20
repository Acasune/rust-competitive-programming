#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        A:[i64;N],
    }
    let mut AA = vec![];
    for i in 0..A.len() {
        AA.push((A[i], i));
    }
    AA.sort();

    println!("{}", AA[N - 2].1 + 1);
}
