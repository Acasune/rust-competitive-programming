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
        A:[usize;N],
    }
    let mut memo = vec![vec![inf as usize; 2]; N];

    memo[0][0] = A[0];
    let mut i = 1;
    while i % N != 0 {
        let j = i % N;
        let j_1 = (i - 1) % N;
        // use
        memo[j][0] = memo[j_1][0].min(memo[j_1][1]) + A[j];
        // not use
        memo[j][1] = memo[j_1][0];
        i += 1;
    }
    let mut ans = memo[N - 1][0].min(memo[(N - 1)][1]);
    memo = vec![vec![inf as usize; 2]; N];
    memo[1][0] = A[1];
    let mut i = 2;
    while i % N != 1 {
        let j = i % N;
        let j_1 = (i - 1) % N;
        // use
        memo[j][0] = memo[j_1][0].min(memo[j_1][1]) + A[j];
        // not use
        memo[j][1] = memo[j_1][0];

        i += 1;
    }
    ans = ans.min(memo[0][0].min(memo[0][1]));
    println!("{}", ans);
}
