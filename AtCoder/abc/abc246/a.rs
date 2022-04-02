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
       mut XY:[(i64,i64);3],

    }
    XY.sort();
    let mut x4 = 0;
    let mut y4 = 0;
    if XY[0].0 == XY[1].0 {
        x4 = XY[2].0;
        if XY[0].1 == XY[2].1 {
            y4 = XY[1].1;
        } else {
            y4 = XY[0].1;
        }
    } else {
        x4 = XY[0].0;
        if XY[0].1 == XY[2].1 {
            y4 = XY[1].1;
        } else {
            y4 = XY[2].1;
        }
    }
    println!("{} {}", x4, y4);
}
