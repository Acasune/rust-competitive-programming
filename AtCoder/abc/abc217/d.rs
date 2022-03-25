#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        L:i64,Q:usize,
        CX:[(i64,i64);Q],
    }
    let mut heap = BTreeSet::<i64>::new();
    heap.insert(0);
    heap.insert(L);
    for (c, x) in CX {
        if c == 2 {
            let pre = *heap.range(x..).next().unwrap();
            let nxt = *heap.range(..x).last().unwrap();
            println!("{}", pre - nxt);
        } else {
            heap.insert(x);
        }
    }
}
