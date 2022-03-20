#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        S:[char;3],
        T:[char;3],
    }
    let mut diff = 0;
    for i in 0..3 {
        if S[i] != T[i] {
            diff += 1;
        }
    }
    if diff == 2 {
        println!("{}", "No");
    } else {
        println!("{}", "Yes");
    }
}
