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
       mut N:usize,
    }
    let mut ans = vec![];
    while N > 0 {
        if N % 2 == 1 {
            N -= 1;
            ans.push('A');
        }
        if N > 0 {
            ans.push('B');
            N /= 2;
        }
    }
    ans.reverse();
    // println!("{}", ans.len());
    println!("{}", ans.iter().collect::<String>());
}
