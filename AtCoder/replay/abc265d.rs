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

type Circle = (i64, i64, i64);

fn main() {
    input! {
        N:usize,P:i64,Q:i64,R:i64,
        mut A:[i64;N]
    }
    A.insert(0, 0);
    let mut B: Vec<i64> = A
        .into_iter()
        .scan(0, |cum, x| {
            *cum += x;
            Some(*cum)
        })
        .collect();
    B.push(inf_i);
    for x in 0..=N - 2 {
        let p = B[x..=N].lower_bound(&(P + B[x])) + x;
        if P != B[p] - B[x] {
            continue;
        }

        let q = B[p + 1..=N].lower_bound(&(Q + B[p])) + p + 1;
        if Q != B[q] - B[p] {
            continue;
        }

        let r = B[q + 1..=N].lower_bound(&(R + B[q])) + q + 1;
        if R == B[r] - B[q] {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
