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
        N:usize,
        A:[usize;N],
        mut X:usize,
    }
    let mut sm = A.iter().sum::<usize>();
    let ans = (X / sm) * N;
    X %= sm;
    let mut cum = vec![0; N + 1];
    for i in 1..=N {
        cum[i] = cum[i - 1] + A[i - 1];
    }
    cum.push(inf as usize);
    // println!("{:?}", X);
    // println!("{:?}", cum);
    let idx = cum
        .binary_search_by_key(&(2 * (X + 1)), |&a| 2 * a + 1)
        .err()
        .unwrap();
    println!("{}", ans + idx);
}
