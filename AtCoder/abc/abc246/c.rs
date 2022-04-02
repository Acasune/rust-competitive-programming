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
        N:usize,K:i64, X:i64,
        mut A:[i64;N],
    }
    let mut k = 0i64;
    let sum = A.iter().sum::<i64>();
    for &a in &A {
        k += a / X;
    }
    if k > K {
        println!("{}", sum - K * X);
    } else {
        for i in 0..N {
            A[i] %= X;
        }
        A.sort();
        A.reverse();
        let mut ans = 0;
        for i in ((K - k) as usize)..N {
            ans += A[i];
        }
        println!("{}", ans);
    }
}
