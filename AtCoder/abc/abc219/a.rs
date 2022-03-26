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
    }
    if N < 40 {
        println!("{}", 40 - N);
    } else if N < 70 {
        println!("{}", 70 - N);
    } else if N < 90 {
        println!("{}", 90 - N);
    } else {
        println!("{}", "expert");
    }
}
