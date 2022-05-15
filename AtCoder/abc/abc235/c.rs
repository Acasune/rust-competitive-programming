#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,Q:usize,
        A:[usize;N],
        asks:[(usize,usize);Q]
    }
    let mut mp = HashMap::new();
    for i in 0..N {
        mp.entry(A[i]).or_insert(vec![]).push(i + 1);
    }
    for (x, k) in asks {
        let entry = mp.get(&x);
        if entry.is_some() && entry.unwrap().len() >= k {
            println!("{}", entry.unwrap()[k - 1]);
        } else {
            println!("{}", -1);
        }
    }
}
