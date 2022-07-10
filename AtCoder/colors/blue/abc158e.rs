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
        N:usize,P:usize,
        S:Chars
    }
    let mut ans = 0usize;
    let mut memo = vec![0usize; P];
    if P % 2 == 0 || P % 5 == 0 {
        for i in 0..N {
            if (S[i].to_digit(10).unwrap() as usize % P) == 0 {
                ans += i + 1;
            }
        }
    } else {
        let mut cu = 0;
        let mut dg = 1;
        for i in (0..N).rev() {
            cu = (cu + dg * (S[i].to_digit(10).unwrap() as usize)) % P;
            dg = (dg * 10) % P;

            ans += memo[cu];
            if cu == 0 {
                ans += 1;
            }
            memo[cu] += 1;
        }
    }
    println!("{}", ans);
}
