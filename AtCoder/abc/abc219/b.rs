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
        S1:Chars,
        S2:Chars,
        S3:Chars,
        T:Chars,
    }
    for t in T {
        if t == '1' {
            for &c in &S1 {
                print!("{}", c);
            }
        } else if t == '2' {
            for &c in &S2 {
                print!("{}", c);
            }
        } else {
            for &c in &S3 {
                print!("{}", c);
            }
        }
    }
    println!("");
}
