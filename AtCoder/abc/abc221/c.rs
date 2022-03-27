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
        mut N:Chars,
    }
    N.sort();
    N.reverse();
    let mut a = 0;
    let mut b = 0;
    for c in N {
        let val = (c).to_digit(10).unwrap();
        if a <= b {
            a *= 10;
            a += val;
        } else {
            b *= 10;
            b += val;
        }
    }
    println!("{}", a * b);
}
