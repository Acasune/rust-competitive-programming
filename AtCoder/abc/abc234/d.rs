#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,K:i64,
        P:[i64;N]
    }
    let mut tree = BinaryHeap::new();
    for i in 0..K as usize {
        tree.push(-P[i]);
    }
    println!("{}", -tree.peek().unwrap());
    for i in K as usize..N {
        tree.push(-P[i]);
        let p = tree.pop().unwrap();
        let q = tree.pop().unwrap();
        let r = p.min(q);
        println!("{}", -r);
        tree.push(r);
    }
}