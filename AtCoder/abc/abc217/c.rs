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
        P:[usize;N],
    }
    let mut Q = vec![0; N];
    for i in 0..N {
        Q[P[i] - 1] = i + 1;
    }
    print!("{}", Q[0]);
    for q in 1..N {
        print!(" {}", Q[q]);
    }
    println!("");
}
