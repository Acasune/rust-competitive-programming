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
        N:usize,K:usize,
        P:[(usize,usize,usize);N],
    }
    let mut ps = vec![];
    for (i, &(x, y, z)) in P.iter().enumerate() {
        ps.push((x + y + z, i));
    }
    ps.sort();
    let pnt = ps[N - K].0;
    let mut ans = vec![""; N];
    for (x, i) in ps {
        if x + 300 >= pnt {
            ans[i] = "Yes";
        } else {
            ans[i] = "No";
        }
    }
    for s in ans {
        println!("{}", s);
    }
}
