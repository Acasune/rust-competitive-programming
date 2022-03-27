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
        N:usize,M:usize,
        mut A:[i64;N+1],
        mut C:[i64;N+M+1],
    }

    let mut B = vec![0i64; M + 1];
    B[M] = C[N + M] / A[N];
    for a in 0..=N {
        C[a + M] -= B[M] * A[a];
    }
    for i in (0..M).rev() {
        B[i] = C[i + N] / A[N];
        for a in 0..=N {
            C[a + i] -= B[i] * A[a];
        }
    }

    print!("{}", B[0]);
    for i in 1..=M {
        print!(" {}", B[i]);
    }
    println!("");
}
