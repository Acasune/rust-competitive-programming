#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
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
        mut XY:[(i64,i64);M],
    }
    let mut mp = BTreeMap::new();
    for &(x, y) in XY.iter() {
        mp.entry(x).or_insert_with(|| vec![]).push(y)
    }
    let mut ans = HashSet::new();
    ans.insert(N as i64);
    for (i, _) in mp.iter() {
        let mut ok = HashSet::new();
        let mut ng = HashSet::new();
        if let Some(vec) = mp.get(i) {
            for &y in vec.iter() {
                if ans.contains(&(y - 1)) {
                    ok.insert(y);
                } else if ans.contains(&(y + 1)) {
                    ok.insert(y);
                } else {
                    ng.insert(y);
                }
            }
        }
        for y in ng {
            ans.remove(&y);
        }
        for y in ok {
            if 0 <= y && y <= 2 * N as i64 {
                ans.insert(y);
            }
        }
    }
    println!("{}", ans.len());
}
