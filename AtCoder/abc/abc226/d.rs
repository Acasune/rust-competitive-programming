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
        xy:[(i64,i64);N],
    }
    let mut st = HashSet::<(i64, i64)>::new();
    for i in 0..N {
        let (x1, y1) = xy[i];
        for j in i + 1..N {
            let (x2, y2) = xy[j];
            let y = y2 - y1;
            let x = x2 - x1;
            let (a, b, g) = ext_gcd(y, x);
            st.insert((x / g, y / g));
        }
    }
    println!("{}", st.len() * 2);
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}
