#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,
    }
    let val = (108 * N) / 100;
    // println!("{}", val);
    if val < 206 {
        println!("{}", "Yay!");
    } else if val == 206 {
        println!("{}", "so-so");
    } else {
        println!("{}", ":(");
    }
}
