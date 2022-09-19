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

const inf_i: i64 = (i64::MAX / 10) * 9;
const inf_u: usize = (usize::MAX / 10) * 9;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,mut K:usize,
        mut A:[usize;N],
    }
    let mut l = 0usize;
    let mut r = 20_000_000_000_000usize;
    while r > l + 1 {
        let m = l + (r - l) / 2;
        let mut sum = 0;
        for i in 0..N {
            sum += A[i].min(m);
        }
        if sum > K {
            r = m;
        } else {
            l = m;
        }
    }
    for i in 0..N {
        let sub = A[i].min(l);
        A[i] -= sub;
        K -= sub;
    }
    // println!("{} {}", l, K);
    for i in 0.. {
        if K == 0 {
            break;
        }
        if A[i] > 0 {
            K -= 1;
            A[i] -= 1;
        }
    }
    print!("{}", A[0]);
    for i in 1..N {
        print!(" {}", A[i]);
    }
    println!();
}
