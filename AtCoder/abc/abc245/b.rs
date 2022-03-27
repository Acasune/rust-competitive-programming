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
        mut A:[usize;N]
    }
    A.sort();
    A.dedup();
    for i in 0..A.len() {
        if A[i] != i {
            println!("{}", i);
            return;
        }
    }
    println!("{}", A.len());
}
