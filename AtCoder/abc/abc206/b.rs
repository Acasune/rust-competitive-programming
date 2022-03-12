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
    let mut sum = 0;
    for i in 1.. {
        sum += i;
        if sum >= N {
            println!("{}", i);
            return;
        }
    }
}
