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
        N:usize,W:usize,
        mut AB:[(usize,usize);N],
    }

    AB.sort();
    AB.reverse();
    let mut ans = 0usize;
    let mut net = 0usize;
    for &(a, b) in &AB {
        if net + b > W {
            ans += (W - net) * a;
            break;
        }
        ans += a * b;
        net += b;
    }
    println!("{}", ans);
}
