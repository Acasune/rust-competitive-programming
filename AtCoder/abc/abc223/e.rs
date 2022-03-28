#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        X:i64,  Y:i64,
        A:i64,  B:i64,  C:i64,
    }
    let abc = vec![A, B, C];
    for bit in 0..(1 << 3) {
        'outer: for perm in (0..3).permutations(3) {
            let mut x = X;
            let mut y = Y;
            for i in 0..3 {
                if x <= 0 || y <= 0 {
                    break 'outer;
                }
                let a = abc[perm[i]];
                if bit >> i & 1 == 1 {
                    // h
                    let used_x = (a + y - 1) / y;
                    x -= used_x;
                } else {
                    let used_y = (a + x - 1) / x;
                    y -= used_y;
                }
            }
            if x >= 0 && y >= 0 {
                // println!("[{},{},{}] {:b}", perm[0], perm[1], perm[2], bit);
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}
