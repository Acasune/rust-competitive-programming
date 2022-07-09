#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
        N:usize,X:usize,
        AB:[(usize,usize);N]
    }
    let mut A = vec![];
    let mut B = vec![];
    for (a, b) in AB {
        A.push(a);
        B.push(b);
    }
    let mut sum = vec![];
    sum.push(A[0] + B[0]);
    for i in 1..N {
        sum.push(A[i] + B[i] + sum[i - 1]);
    }
    for i in 1..N {
        B[i] = B[i].min(B[i - 1]);
    }
    let mut ans = inf_u;
    // println!("{:?}", sum);
    for i in 0..N.min(X) {
        let mut rem = X - i - 1;
        let mut tmp = sum[i];
        tmp += B[i] * rem;
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
