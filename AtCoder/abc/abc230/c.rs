#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:i64, A:i64, B:i64,
        P:i64, Q:i64, R:i64, S:i64,
    }
    let lower_1 = (1 - A).max(1 - B);
    let upper_1 = (N - A).min(N - B);
    let lower_2 = (1 - A).max(B - N);
    let upper_2 = (N - A).min(B - 1);
    let s1 = (A + lower_1, B + lower_1);
    let g1 = (A + upper_1, B + upper_1);
    let s2 = (A + lower_2, B - lower_2);
    let g2 = (A + upper_2, B - upper_2);
    // println!("{:?} {:?}", s2, g2);
    for i in P..=Q {
        for j in R..=S {
            if s1.0 <= i && i <= g1.0 && s1.1 <= j && j <= g1.1 && i == j + (s1.0 - s1.1) {
                print!("#");
            } else if s2.0 <= i && i <= g2.0 && g2.1 <= j && j <= s2.1 && i == -j + (s2.0 + s2.1) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
