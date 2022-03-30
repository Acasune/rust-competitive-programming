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
        mut S:Chars,
    }
    S.sort();
    S.dedup();
    let cnt = S.len();
    if cnt == 3 {
        println!("{}", 6);
    } else if cnt == 2 {
        println!("{}", 3);
    } else {
        println!("{}", 1);
    }
}
