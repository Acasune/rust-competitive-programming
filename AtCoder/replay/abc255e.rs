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
        N:usize,M:usize,
        S:[i64;N-1],
        X:[i64;M],
    }
    let mut mp = HashMap::new();
    for j in 0..M {
        let mut b = 0;
        for i in 0..N {
            let mut z = X[j] - b;
            if i % 2 == 1 {
                z *= -1;
            }
            *mp.entry(z).or_insert(0i64) += 1;
            if i != N - 1 {
                b = S[i] - b;
            }
        }
    }
    let ans = mp.values().max().unwrap();
    println!("{}", ans);
}
