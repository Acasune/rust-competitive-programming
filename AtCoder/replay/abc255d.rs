#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,Q:usize,
        mut A:[usize;N],
        ask:[usize;Q],
    }
    A.sort();
    let mut cum: Vec<usize> = A
        .iter()
        .scan(0, |cum, &x| {
            *cum += x;
            Some(*cum)
        })
        .collect();
    cum.insert(0, 0);
    for x in ask {
        let bnd = A.lower_bound(&x);
        let mut ans = x * bnd - cum[bnd];
        ans += cum[N] - cum[bnd] - (N - bnd) * x;
        println!("{}", ans);
    }
}
