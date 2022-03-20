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
        H:usize,W:usize,N:usize,
        AB:[(i64,i64);N],
    }
    let mut rows = vec![];
    let mut cols = vec![];
    rows.push(-inf);
    cols.push(-inf);

    for &(a, b) in &AB {
        rows.push(a);
        cols.push(b);
    }
    rows.push(inf);
    cols.push(inf);
    rows.sort();
    rows.dedup();
    cols.sort();
    cols.dedup();

    for i in 0..N {
        let (aa, bb) = AB[i];
        let row_i = rows
            .binary_search_by_key(&(2 * aa), |&b| 2 * b + 1)
            .err()
            .unwrap();
        let col_i = cols
            .binary_search_by_key(&(2 * bb), |&b| 2 * b + 1)
            .err()
            .unwrap();
        println!("{} {}", row_i, col_i);
    }
}
