#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        S:Chars
    }
    let mut xy = (0, 0);
    let mut dir = 0;
    for i in 0..N {
        if S[i] == 'S' {
            if dir == 0 {
                xy.0 += 1;
            } else if dir == 1 {
                xy.1 -= 1;
            } else if dir == 2 {
                xy.0 -= 1;
            } else {
                xy.1 += 1;
            }
        } else {
            dir = (dir + 1) % 4;
        }
    }
    println!("{} {}", xy.0, xy.1);
}
