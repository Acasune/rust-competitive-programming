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
        mut C:[i64;N]
    }
    C.sort();
    let mut ans = C[0];
    for i in 1..N {
        ans *= C[i] - i as i64;
        ans %= md;
    }
    println!("{}", ans);
}
