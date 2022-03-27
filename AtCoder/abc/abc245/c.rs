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
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,K:i64,
        A:[i64;N],
        B:[i64;N],
    }
    let mut dp = vec![vec![false; 2]; N];
    dp[0][0] = true;
    dp[0][1] = true;
    for i in 1..N {
        for j in 0..2 {
            if dp[i - 1][0] {
                if i64::abs(A[i] - A[i - 1]) <= K {
                    dp[i][0] = true;
                }
                if i64::abs(B[i] - A[i - 1]) <= K {
                    dp[i][1] = true;
                }
            }
            if dp[i - 1][1] {
                if i64::abs(A[i] - B[i - 1]) <= K {
                    dp[i][0] = true;
                }
                if i64::abs(B[i] - B[i - 1]) <= K {
                    dp[i][1] = true;
                }
            }
        }
    }
    if dp[N - 1][0] || dp[N - 1][1] {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
