#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,
        A:[usize;N], B:[usize;N],
    }
    let mut ans1 = 0i64;
    let mut ans2 = 0i64;
    for i in 0..N {
        for j in 0..N {
            if i == j && A[i] == B[j] {
                ans1 += 1;
            } else if i != j && A[i] == B[j] {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);
}
