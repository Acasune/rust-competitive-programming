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
        AB:[(f64,f64);N]
    }
    let mut C = vec![];
    for &(a, b) in &AB {
        C.push(a / b);
    }
    let sum = C.iter().sum::<f64>();
    let time = sum / 2.;
    let mut ans = 0.;
    let mut idx = 0usize;
    let mut time_sum = 0.;
    while time_sum <= time {
        if time_sum + C[idx] > time {
            ans += (time - time_sum) * AB[idx].1;
            break;
        }
        ans += AB[idx].0;
        time_sum += C[idx];
        idx += 1;
    }
    println!("{:.8}", ans);
}
