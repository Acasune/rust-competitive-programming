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
        N:i64,
    }
    let mut ans = 0;
    let mut a = 1i64;
    while a * a <= N {
        let mut b = a;
        while a * b <= N {
            let c = N / (a * b);
            if c < b {
                break;
            }
            ans += c - b + 1;
            // println!("{} {} {}", ans, c, b);
            b += 1;
        }
        a += 1;
    }
    println!("{}", ans);
}
