#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,K:i64,
        mut A:[i64;N]
    }
    A.sort();
    A.reverse();
    let mut l = 0i64;
    let mut r = 1_000_000_000_000_000_000 / K;

    while r - l > 1 {
        let m = (r + l) / 2;
        if is_possible(m, K, &A) {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}

fn is_possible(m: i64, K: i64, A: &Vec<i64>) -> bool {
    let mut sum = 0;
    for &a in A {
        sum += a.min(m);
    }
    return sum >= K * m;
}
