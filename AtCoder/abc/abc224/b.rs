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
        H:usize,W:usize,
        A:[[usize;W];H],
    }
    for i in 0..H {
        for j in 0..W {
            for k in i + 1..H {
                for l in j + 1..W {
                    if A[i][j] + A[k][l] > A[k][j] + A[i][l] {
                        println!("{}", "No");
                        return;
                    }
                }
            }
        }
    }
    println!("{}", "Yes");
}
