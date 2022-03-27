#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        M:usize,
        A:[usize;N],
        B:[usize;N],
        C:[usize;M],
        D:[usize;M],
    }
    let mut AB = vec![];
    for i in 0..N {
        AB.push((A[i], B[i], 0));
    }
    for i in 0..M {
        AB.push((C[i], D[i], 1));
    }
    AB.sort();
    AB.reverse();
    let mut set = BTreeSet::<(usize, usize)>::new();
    for i in 0..N + M {
        let (_, b, z) = AB[i];
        if z == 0 {
            if let Some(&(d, j)) = set.range((b, 0)..).next() {
                set.remove(&(d, j));
            } else {
                println!("{}", "No");
                return;
            }
        } else {
            set.insert((b, i));
        }
    }
    println!("{}", "Yes");
}
