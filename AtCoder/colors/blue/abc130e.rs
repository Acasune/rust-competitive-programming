#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,
        S:[usize;N],
        T:[usize;M]
    }
    let md = md1000000007 as usize;
    let mut dp = vec![vec![0;M+1];N+1];
    for i in 0..M+1 {
        dp[0][i] = 1;
    }
    for i in 0..N+1 {
        dp[i][0] = 1;
    }
    for i in 0..N {
        let s = S[i];
        for j in 0..M {
            let t = T[j];
            dp[i+1][j+1] += md + dp[i+1][j] + dp[i][j+1] - dp[i][j];
            if s == t {
                dp[i+1][j+1] += dp[i][j];;
            }
            dp[i+1][j+1] %= md;
        }
    }
    println!("{}",dp[N][M]);


}
