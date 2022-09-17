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
        N:usize,
        A:[usize;N],
    }
    let md = md998244353 as usize;
    let mut ans = 0;
    for i in 1..=N {
        let mut dp = vec![vec![vec![0; i]; i + 1]; N + 1];
        dp[0][0][0] = 1;
        for j in 0..N {
            for k in 0..=i {
                for l in 0..i {
                    dp[j + 1][k][l] += dp[j][k][l];
                    dp[j + 1][k][l] %= md;
                    if k != i {
                        dp[j + 1][k + 1][(l + A[j]) % i] += dp[j][k][l];
                        dp[j + 1][k + 1][(l + A[j]) % i] %= md;
                    }
                }
            }
        }
        ans += dp[N][i][0];
        ans %= md;
    }
    println!("{}", ans);
}
