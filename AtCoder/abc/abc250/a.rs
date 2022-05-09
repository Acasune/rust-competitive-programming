#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        H:i64,W:i64,
        R:i64,C:i64,
    }
    let cand = [(R - 1, C), (R + 1, C), (R, C - 1), (R, C + 1)];
    let mut ans = 0;
    for &(a, b) in &cand {
        if 1 <= a && a <= H && 1 <= b && b <= W {
            ans += 1;
        }
    }
    println!("{}", ans);
}
