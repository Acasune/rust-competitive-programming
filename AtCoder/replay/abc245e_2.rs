#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,
        A:[i64;N],
        B:[i64;N],
        C:[i64;M],
        D:[i64;M],
    }
    let mut stk = vec![];
    for i in 0..N {
        stk.push((A[i], B[i], 0));
    }
    for i in 0..M {
        stk.push((C[i], D[i], 1));
    }
    stk.sort();
    let mut stock = BTreeSet::new();
    let mut idx = 0;
    while let Some((e, f, g)) = stk.pop() {
        if g == 0 {
            if let Some(&(d, i)) = stock.range((f, 0)..).next() {
                stock.remove(&(d, i));
            } else {
                println!("{}", "No");
                return;
            }
        } else {
            stock.insert((f, idx));
        }
        idx += 1;
    }
    println!("{}", "Yes");
}
