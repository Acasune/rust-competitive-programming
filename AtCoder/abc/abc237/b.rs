#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        H:usize, W:usize,
        A:[[usize;W];H]
    }
    let mut B = left_rotate(&A);
    B.reverse();
    for bb in B {
        for (i, b) in bb.into_iter().enumerate() {
            if i == 0 {
                print!("{}", b);
            } else {
                print!(" {}", b);
            }
        }
        println!();
    }
}

fn left_rotate<T>(graph: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let H = graph.len();
    let W = graph[0].len();
    let mut ret = vec![vec![graph[0][0]; H]; W];
    for h in 0..H {
        for w in 0..W {
            ret[W - w - 1][h] = graph[h][w];
        }
    }
    ret
}
