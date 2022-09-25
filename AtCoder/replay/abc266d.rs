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

type Circle = (i64, i64, i64);

fn main() {
    input! {
        N:usize,
        txa:[(usize,usize,usize);N],
    }
    let mut mp = HashMap::new();
    for (t, x, a) in txa {
        mp.insert((t, x), a);
    }
    let mut dp = vec![0, 0, 0, 0, 0];
    for i in 1..=100_000 {
        let mut tmp = dp.clone();
        for x in 0..5 {
            let mut a = 0;
            if let Some(&b) = mp.get(&(i, x)) {
                a = b;
            }
            if x == 0 {
                let m = if x < i { dp[x] } else { 0 };
                let r = if x + 1 < i { dp[x + 1] } else { 0 };
                tmp[x] = m.max(r) + a;
            } else if x == 4 {
                let l = if x - 1 < i { dp[x - 1] } else { 0 };
                let m = if x < i { dp[x] } else { 0 };
                tmp[x] = l.max(m) + a;
            } else {
                let l = if x - 1 < i { dp[x - 1] } else { 0 };
                let m = if x < i { dp[x] } else { 0 };
                let r = if x + 1 < i { dp[x + 1] } else { 0 };
                tmp[x] = l.max(m.max(r)) + a;
            }
        }
        dp = tmp;
    }
    // println!("{:?}", dp);
    let ans = dp.into_iter().max().unwrap();
    println!("{}", ans);
}
