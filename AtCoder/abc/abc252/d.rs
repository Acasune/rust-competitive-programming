#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,
        mut A:[usize;N],
    }
    A.sort();
    let mut mp = HashMap::new();
    for a in A {
        *mp.entry(a).or_insert(0i64) += 1;
    }
    let mut vec = vec![0];
    for (k, v) in mp {
        vec.push(v);
    }
    let mut cum = vec![0; vec.len()];
    for i in 1..vec.len() {
        cum[i] += cum[i - 1] + vec[i];
    }
    let mut ans = 0;
    for i in 1..vec.len() - 1 {
        let left = cum[i - 1];
        let right = cum[cum.len() - 1] - cum[i];
        ans += left * vec[i] * right;
    }
    println!("{}", ans);
}
