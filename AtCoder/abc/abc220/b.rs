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
        N:i64,
        mut A:i64,mut B:i64,
    }
    let mut a = 0i64;
    let mut base = 1;
    while A > 0 {
        a += base * (A % 10);
        A /= 10;
        base *= N;
    }
    let mut b = 0i64;
    let mut base = 1;
    while B > 0 {
        b += base * (B % 10);
        B /= 10;
        base *= N;
    }
    println!("{}", a * b);
}
