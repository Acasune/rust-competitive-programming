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

type Circle = (i64, i64, i64);

fn main() {
    input! {
        N:usize,M:usize,
        A:[i64;N],
    }
    let mut dp = vec![vec![-inf_i; N + 1]; M + 1];
    for i in 1..=N {
        let a = A[i - 1];
        dp[1][i] = a.max(dp[1][i - 1]);
    }
    for i in 2..=M {
        for j in i..=N {
            let a = A[j - 1];
            dp[i][j] = (dp[i - 1][j - 1] + i as i64 * a).max(dp[i][j - 1]);
        }
    }

    let ans = dp[M][N];
    println!("{}", ans);
}
