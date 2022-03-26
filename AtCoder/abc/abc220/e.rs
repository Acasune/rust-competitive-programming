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
const md: usize = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,D:usize,
    }
    let mut ans = 0;
    let mut memo = vec![1usize; 2 * N + 1];

    for i in 1..=2 * N {
        memo[i] = memo[i - 1] * 2;
        memo[i] %= md;
    }
    for d in 0..N {
        if d + D <= N - 1 {
            ans += memo[d] * memo[D + 1];
            ans %= md;
        }
        if 2 * (N as i64 - 1 - d as i64) < D as i64 || D == 1 {
        } else if d + D <= N - 1 {
            ans += ((memo[d] * memo[D - 1]) % md * (D - 1)) % md;
            ans %= md;
        } else {
            ans += (memo[d] * (memo[D - 1] * (2 * N - 2 * d - D - 1) % md)) % md;
            ans %= md;
        }
        // println!("{} {}", ans, D);
    }

    println!("{}", ans % md);
}
