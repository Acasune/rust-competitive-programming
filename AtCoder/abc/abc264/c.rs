#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        H1:usize,
        W1:usize,
        A:[[usize;W1];H1],
        H2:usize,
        W2:usize,
        B:[[usize;W2];H2],
    }
    let mut set = HashSet::new();
    for h_bit in 1..(1 << H1) {
        if (h_bit as u64).count_ones() != H2.try_into().unwrap() {
            continue;
        }
        for w_bit in 1..(1 << W1) {
            if (w_bit as u64).count_ones() != W2.try_into().unwrap() {
                continue;
            }
            let mut vec = vec![];
            for j in 0..H1 {
                if h_bit >> j & 1 == 0 {
                    continue;
                }
                let mut vv = vec![];
                for i in 0..W1 {
                    if w_bit >> i & 1 == 0 {
                        continue;
                    }
                    vv.push(A[j][i])
                }
                vec.push(vv);
            }
            set.insert(vec);
        }
    }

    if set.contains(&B) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
