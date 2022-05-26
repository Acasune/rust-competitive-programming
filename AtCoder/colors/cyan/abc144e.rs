#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize, mut K:i64,
         mut A:[i64;N],
         mut F:[i64;N]
    }

    F.sort();
    F.reverse();
    A.sort();
    let mut ok = inf;
    let mut ng = -1;
    while ng + 1 != ok {
        let m = (ok + ng) / 2;
        if check(m, K, &A, &F) {
            ok = m;
        } else {
            ng = m;
        }
    }

    println!("{}", ok);
}

fn check(m: i64, K: i64, A: &Vec<i64>, F: &Vec<i64>) -> bool {
    let mut cnt = 0;
    for i in 0..A.len() {
        let a = A[i];
        let f = F[i];
        let mut ng = a + 1;
        let mut ok = 0;
        while ng != ok + 1 {
            let mm = (ng + ok) / 2;
            if mm * f <= m {
                ok = mm;
            } else {
                ng = mm;
            }
        }
        cnt += a - ok;
        if m == 0 {}
    }
    return cnt <= K;
}
