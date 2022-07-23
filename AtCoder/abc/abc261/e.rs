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
        N:usize,C:i32,
        TA:[(i32,i32);N]
    }
    let mut ans = vec![0; N];
    for i in 0..30 {
        let mut cur = (C >> i) & 1;
        let mut fnc = vec![0, 1];
        for (j, &(t, a)) in TA.iter().enumerate() {
            let mut f = vec![0; 0];
            let x = (a >> i) & 1;
            if t == 1 {
                f = vec![0 & x, 1 & x];
            } else if t == 2 {
                f = vec![0 | x, 1 | x];
            } else {
                f = vec![0 ^ x, 1 ^ x];
            }
            fnc = vec![f[fnc[0] as usize], f[fnc[1] as usize]];
            cur = fnc[cur as usize];
            ans[j] |= cur << i;
        }
    }
    for i in 0..N {
        println!("{}", ans[i]);
    }
}
