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
        N:usize,
        Name:[(String,String);N],
    }
    let mut mp = HashMap::new();
    // let mut st = HashSet::new();
    // for (a, b) in &Name {
    //     st.insert(&a);
    //     st.insert(&b);
    // }

    for (a, b) in &Name {
        if a == b {
            *mp.entry(a.clone()).or_insert(0) += 1;
        } else {
            *mp.entry(a.clone()).or_insert(0) += 1;
            *mp.entry(b.clone()).or_insert(0) += 1;
        }
    }
    for (a, b) in &Name {
        if *mp.get(a).unwrap() > 1 && *mp.get(b).unwrap() > 1 {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
