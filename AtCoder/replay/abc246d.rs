#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:i64,
    }
    let mut ans = i64::MAX;
    for a in 0..=1_009_010 {
        let ret = binary_search(a, N);
        ans = ans.min(ret);
    }
    println!("{}", ans);
}

fn binary_search(a: i64, N: i64) -> i64 {
    let b = 1_009_010;
    let mut l = -1;
    let mut r = b;
    while r - l > 1 {
        let m = (r + l) / 2;
        let x = x(a, m);
        if x >= N {
            r = m;
        } else {
            l = m;
        }
    }
    x(a, r)
}

fn x(a: i64, b: i64) -> i64 {
    a * a * a + a * a * b + a * b * b + b * b * b
}
