#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
        N:usize,Q:usize,mut X:usize,
        W:[usize;N],
        ask:[usize;Q]
    }
    // next start
    let mut W2 = W.clone();
    W2.extend(W.clone());
    let acc = W.iter().sum::<usize>();
    let mut cnts = vec![(X / acc) * N; N];
    X %= acc;
    let mut i = 0;
    let mut j = 0;
    let mut s = 0;
    while i < N {
        if j < i {
            j = i;
            s = 0;
        }
        while s < X {
            s += W2[j];
            j += 1;
        }
        cnts[i] += j - i;
        s -= W[i];
        i += 1;
    }
    let mut order = vec![inf_u; N];
    let mut path = vec![];
    let mut u = 0;
    let mut k = 0;
    let mut cycle = 0;
    loop {
        if order[u] != inf_u {
            cycle = k - order[u];
            break;
        }
        order[u] = k;
        path.push(u);
        u = (u + cnts[u]) % N;
        k += 1;
    }
    // println!("{:?}", cnt);
    let not_cycle = path.len() - cycle;
    for k in ask {
        let k = k - 1;
        if k < not_cycle {
            println!("{}", cnts[path[k]]);
        } else {
            let k = (k - not_cycle) % cycle;
            println!("{}", cnts[path[not_cycle + k]]);
        }
    }
}
