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
         mut P:i64,
    }
    let mut ans = 0;
    for i in (1..=10).rev() {
        let m = frac(i);
        ans += P / m;
        P = P % m;
    }
    println!("{}", ans);
}

fn frac(m: i64) -> i64 {
    let mut m = m;
    let mut ret = 1;
    while m > 0 {
        ret *= m;
        m -= 1;
    }
    ret
}
