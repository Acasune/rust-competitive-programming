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
        N:usize,
        B:[Chars;N]
    }
    let mut A = vec![];
    for b in B {
        A.push(b.into_iter().collect::<Vec<char>>());
    }
    let mut is_correct = true;
    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            if A[i][j] == 'W' {
                if A[j][i] != 'L' {
                    is_correct = false;
                }
            } else if A[i][j] == 'L' {
                if A[j][i] != 'W' {
                    is_correct = false;
                }
            } else {
                if A[j][i] != 'D' {
                    is_correct = false;
                }
            }
        }
    }
    if is_correct {
        println!("{}", "correct");
    } else {
        println!("{}", "incorrect");
    }
}
