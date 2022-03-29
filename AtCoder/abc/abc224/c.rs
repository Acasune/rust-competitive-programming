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
        N:usize,
        XY:[(i64,i64);N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                let x1 = XY[i].0 - XY[j].0;
                let y1 = XY[i].1 - XY[j].1;
                let x2 = XY[i].0 - XY[k].0;
                let y2 = XY[i].1 - XY[k].1;
                if y1 * x2 != y2 * x1 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
