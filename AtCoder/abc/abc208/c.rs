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
const eps: f64 = 0.000_000_000_001;

fn main() {
    input! {
        mut N:usize,mut K:usize,
        A:[usize;N],
    }
    let mut ans = K / N;
    K = K % N;
    let mut vec = vec![];
    let mut vec2 = vec![ans; N];
    for (i, a) in A.iter().enumerate() {
        vec.push((a, i));
    }
    vec.sort();
    for i in 0..K {
        vec2[vec[i].1] += 1;
    }
    for e in &vec2 {
        println!("{}", e);
    }
}
