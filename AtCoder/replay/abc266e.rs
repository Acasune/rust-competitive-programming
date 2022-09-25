#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,
    }
    let mut dp = vec![0., 1., 2., 3., 4., 5., 6.];
    for i in 1..N {
        let expected = dp.iter().cloned().sum::<f64>() / 6.;
        let mut tmp = vec![0., 0., 0., 0., 0., 0., 0.];
        for j in 1..=6 {
            let jj = j as f64;
            if jj <= expected {
                tmp[j] = expected
            } else {
                tmp[j] = dp[j];
            }
        }
        dp = tmp;
    }
    let ans = dp.into_iter().sum::<f64>() / 6.;
    println!("{}", ans);
}
