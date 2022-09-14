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
        N:usize,
        S:(i64,i64),
        T:(i64,i64),
        cls:[(i64,i64,i64);N],
    }
    let mut input = vec![];
    let mut visited = vec![false; N];
    for i in 0..N {
        if on_circle(S.0, S.1, cls[i]) {
            input.push(i);
            visited[i] = true;
        }
    }
    while let Some(s) = input.pop() {
        for i in 0..N {
            if !visited[i] {
                if is_crossed(cls[s], cls[i]) {
                    visited[i] = true;
                    input.push(i);
                }
            }
        }
    }
    for i in 0..N {
        if on_circle(T.0, T.1, cls[i]) && visited[i] {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}

fn is_crossed(cl1: Circle, cl2: Circle) -> bool {
    !(pow(cl1.2 + cl2.2) < pow(cl1.0 - cl2.0) + pow(cl1.1 - cl2.1)
        || pow(cl1.2 - cl2.2) > pow(cl1.0 - cl2.0) + pow(cl1.1 - cl2.1))
}

fn on_circle(sx: i64, sy: i64, cl: Circle) -> bool {
    pow(cl.2) == pow(sx - cl.0) + pow(sy - cl.1)
}

fn pow(x: i64) -> i64 {
    x * x
}
