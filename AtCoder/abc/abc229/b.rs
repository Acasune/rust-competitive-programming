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
        mut A:usize,
        mut B:usize,
    }
    while A > 0 && B > 0 {
        let a = A % 10;
        let b = B % 10;
        if a + b >= 10 {
            println!("{}", "Hard");
            return;
        }
        A /= 10;
        B /= 10;
    }
    println!("{}", "Easy");
}
