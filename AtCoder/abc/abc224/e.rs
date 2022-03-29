#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: usize = usize::MAX / 100000000;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        H:usize, W:usize, N:usize,
        rca:[(Usize1,Usize1,usize);N],
    }
    let mut mp = BTreeMap::<usize, Vec<usize>>::new();
    for i in 0..N {
        mp.entry(rca[i].2).or_insert_with(|| vec![]).push(i);
    }

    let mut dp = vec![0i64; N];
    let mut cmax = vec![0i64; W];
    let mut rmax = vec![0i64; H];
    let mut ans = vec![0; N];
    mp.iter().rev().for_each(|e| {
        let &a = e.0;
        for &i in e.1 {
            dp[i] = rmax[rca[i].0].max(cmax[rca[i].1]);
        }
        for &i in e.1 {
            rmax[rca[i].0] = rmax[rca[i].0].max(dp[i] + 1);
            cmax[rca[i].1] = cmax[rca[i].1].max(dp[i] + 1);
        }
    });

    for a in dp {
        println!("{}", a);
    }
}
