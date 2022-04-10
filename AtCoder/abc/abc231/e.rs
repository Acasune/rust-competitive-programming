// This code is a study referring to @sansen 's code
// https://atcoder.jp/contests/abc231/submissions/27830591

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,X:usize,
        A:[usize;N],
    }
    let mut dp = BTreeMap::new();
    dp.insert(X, 0);
    for a in A.windows(2) {
        let mut next = BTreeMap::new();
        for (x, v) in dp {
            let r = x % a[1];
            let c = r / a[0];
            // Pay fraction (a buyer pays fractions)
            // Clearly (x - r) is divisible by a[0]
            next.entry(x - r).or_insert(inf as usize).chmin(v + c);
            if c > 0 {
                // Pay fraction (a seller pays fractions)
                let c = a[1] / a[0] - c;
                // When x = 87, then x + c * a[0] equals 87 + 3 (3 is changes that a seller sends to a buyer)
                next.entry(x + c * a[0])
                    .or_insert(inf as usize)
                    .chmin(v + c);
            }
        }
        dp = next;
    }
    let ans = dp.into_iter().map(|(k, v)| v + k / A[N - 1]).min().unwrap();
    println!("{}", ans);
}

pub trait ChangeMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}

impl<T: PartialOrd> ChangeMinMax for T {
    fn chmin(&mut self, x: Self) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }
    fn chmax(&mut self, x: Self) -> bool {
        *self < x && {
            *self = x;
            true
        }
    }
}
