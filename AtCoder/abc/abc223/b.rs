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
        S:Chars,
    }
    let mut vec = vec![];
    let n = S.len();
    for i in 0..n {
        let mut cs = vec![];
        for j in 0..n {
            cs.push(S[(i + j) % n]);
        }
        vec.push(cs);
    }
    vec.sort();
    let n = vec.len();
    println!("{}", vec[0].iter().collect::<String>());
    println!("{}", vec[n - 1].iter().collect::<String>());
}
