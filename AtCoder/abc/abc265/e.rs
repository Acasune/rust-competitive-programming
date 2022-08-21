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
        N:usize,M:usize,
        A:i64,B:i64,C:i64,
        D:i64,E:i64,F:i64,
        XY:[(i64,i64);M],
    }
    let mut ban = HashSet::new();
    for (x, y) in XY {
        ban.insert((x, y));
    }
    let md = md998244353;
    let mut dp = vec![vec![vec![0; N + 1]; N + 1]; N + 1];
    let abcdef = vec![(A, B), (C, D), (E, F)];
    dp[0][0][0] = 1;
    for i in 0..(N as i64) {
        for a in 0..=i {
            for b in 0..=i - a {
                let c = i - a - b;
                let x = A * a + C * b + E * c;
                let y = B * a + D * b + F * c;
                for k in 0..3 {
                    let dx = abcdef[k].0;
                    let dy = abcdef[k].1;
                    if !ban.contains(&(x + dx, y + dy)) {
                        dp[i as usize + 1][a as usize + if k == 0 { 1 } else { 0 }]
                            [b as usize + if k == 1 { 1 } else { 0 }] +=
                            dp[i as usize][a as usize][b as usize];
                        dp[i as usize + 1][a as usize + if k == 0 { 1 } else { 0 }]
                            [b as usize + if k == 1 { 1 } else { 0 }] %= md;
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for x in 0..=N {
        for y in 0..=N {
            ans += dp[N as usize][x as usize][y as usize];
            ans %= md;
        }
    }

    println!("{}", ans % md);
}
