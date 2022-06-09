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
        N:usize,
        AB: [(usize,usize);N],
    }
    let mut A = vec![];
    let mut B = vec![];
    for (a,b) in AB {
        A.push(a);
        B.push(b);
    }
    A.sort();
    B.sort();
    if N %2 == 0 {
        let a = A[N/2] + A[N/2-1];
        let b = B[N/2] + B[N/2-1];
        println!("{}",b-a+1);
    } else {
        let a = A[N/2];
        let b = B[N/2];
        println!("{}",b-a+1);
    }
}
