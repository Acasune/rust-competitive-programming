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
        S:Chars,
        mut X:Chars,
    }
    let mut dp = vec![];
    let mut S = S
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    S.reverse();
    X.reverse();
    dp.push(vec![0usize]);
    for i in 0..N {
        let nxt = S[i];
        let per = X[i];
        let mut vec = vec![];
        if per == 'T' {
            for num in 0..10 {
                if dp[i].iter().any(|&x| x == ((10 * num + nxt) % 7))
                    || dp[i].iter().any(|&x| x == ((10 * num) % 7))
                {
                    vec.push(num);
                }
            }
        } else {
            for num in 0..10 {
                if dp[i].iter().any(|&x| x == ((10 * num + nxt) % 7))
                    && dp[i].iter().any(|&x| x == ((10 * num) % 7))
                {
                    vec.push(num);
                }
            }
        }
        dp.push(vec);
    }
    if dp[N].iter().any(|&x| x == 0) {
        println!("{}", "Takahashi");
    } else {
        println!("{}", "Aoki");
    }
}
