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
        AB:[(Usize1,Usize1);N-1],
    }
    let mut edge = vec![vec![]; N];
    for (a, b) in AB {
        edge[a].push(b);
        edge[b].push(a);
    }
    for i in 0..N {
        if edge[i].len() == N - 1 {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
