#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;

fn main() {
    input! {
        mut A:i64,  B:i64,  C:i64,  D:i64,
    }
    if D * C - B <= 0 {
        println!("{}", -1);
    } else {
        println!("{}", (A + D * C - B - 1) / (D * C - B));
    }
}
