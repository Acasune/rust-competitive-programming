#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        H:[usize;3],
        W:[usize;3],
    }
    let mut ans = 0;
    for a in 1..=30 {
        for b in  1..=30 {
            for d in 1..=30 {
                for e in 1..=30 {
                    if a + b >=H[0] || d+e>=H[1] || a + d >= W[0] || b + e >= W[1]{
                        continue;
                    }
                    let c = H[0] - a - b;
                    let f = H[1] - d - e;
                    let g = W[0] - a - d;
                    let h = W[1] - b - e;
                    if c + f >= W[2] {
                        continue;
                    }
                    let i = W[2] - c -f;
                    if i + g + h == H[2] {
                        ans +=1;
                    }

                }
            }
        }
    }
    println!("{}",ans);
}
