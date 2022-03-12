#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::{BinaryHeap, HashMap};

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,
        A:[usize;N],
    }
    let mut mp = HashMap::<usize, usize>::new();
    for &a in &A {
        *mp.entry(a).or_insert(0) += 1;
    }
    let mut ans = (N * (N - 1)) / 2usize;
    for entry in mp {
        // println!("{}", ans);
        let tmp = entry.1;
        ans -= tmp * (tmp - 1) / 2;
    }
    println!("{}", ans);
}
