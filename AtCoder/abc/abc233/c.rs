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
        N:usize,X:usize,
    }
    let mut ll = vec![];
    let mut aa = vec![];
    for i in 0..N {
        input! {
            L:usize,A:[usize;L],
        }
        ll.push(L);
        aa.push(A);
    }
    let mut ans = 0;
    dfs(&aa, 1, &mut ans, 0, X);
    println!("{}", ans);
}

fn dfs(A: &Vec<Vec<usize>>, memo: usize, ans: &mut usize, i: usize, X: usize) {
    if i == A.len() {
        if memo == X {
            *ans += 1;
        }
        return;
    }
    for &a in &A[i] {
        if memo <= (X + a - 1) / a {
            dfs(A, memo * a, ans, i + 1, X);
        }
    }
}
