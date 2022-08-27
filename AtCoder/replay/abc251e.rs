#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

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
        A:[usize;N],
    }
    let mut ans = inf_u;
    let mut dp = vec![vec![0; 2]; N];
    for j in 0..=1 {
        if j == 0 {
            dp[0][0] = A[0];
            dp[0][1] = inf_u;
        } else {
            dp[0][0] = inf_u;
            dp[0][1] = 0;
        }

        for i in 1..N {
            dp[i][0] = dp[i - 1][0].min(dp[i - 1][1]) + A[i];
            dp[i][1] = dp[i - 1][0];
        }
        if j == 0 {
            ans = dp[N - 1][0].min(dp[N - 1][1]);
        } else {
            ans = ans.min(dp[N - 1][0]);
        }
    }
    println!("{}", ans);
}
