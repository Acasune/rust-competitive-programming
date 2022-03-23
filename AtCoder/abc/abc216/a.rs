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
        N:Chars,
    }
    let y = (N[N.len() - 1] as usize - '0' as usize) as usize;
    print!("{}", N[0..(N.len() - 2)].iter().collect::<String>());
    if 0 <= y && y <= 2 {
        println!("{}", '-');
    } else if 3 <= y && y <= 6 {
        println!("");
    } else {
        println!("{}", '+');
    }
}
