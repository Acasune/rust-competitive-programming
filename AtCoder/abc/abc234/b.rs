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
        N:usize,
        ps:[(i64,i64);N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            let (x1, y1) = ps[i];
            let (x2, y2) = ps[j];
            let dist = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
            ans = ans.max(dist);
        }
    }
    println!("{}", (ans as f64).sqrt());
}
