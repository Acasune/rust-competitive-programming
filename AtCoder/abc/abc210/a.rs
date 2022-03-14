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
        N:i64, A:i64, mut X:i64, Y:i64,
    }
    let mut ans = 0i64;
    for i in 0..N {
        if i == A {
            X = Y;
        }
        // println!("{}", ans);
        ans += X;
    }
    println!("{}", ans);
}
