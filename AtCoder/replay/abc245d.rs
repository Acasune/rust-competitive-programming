#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

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
        mut A:[i64;N+1],
        mut C:[i64;N+M+1]
    }
    let mut B = vec![0; M + 1];
    for i in 0..=M {
        let mut d = 0;
        for j in 1..=i.min(N) {
            d += A[N - j] * B[M - i + j];
        }
        B[M - i] = (C[N + M - i] - d) / A[N];
    }
    for b in B {
        print!("{} ", b);
    }
    println!();
}
