#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
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
        N:usize,K:usize,
        P:[Usize1;N]
    }
    let mut group = vec![inf_u; N + 1];
    let mut ans = vec![inf_u; N];
    let mut set = BTreeSet::<(usize, usize)>::new();
    for (i, p) in P.into_iter().enumerate() {
        if let Some(&(j, size)) = set.range((p, 0)..).next() {
            if size + 1 == K {
                ans[p] = i + 1;
                let mut idx = j;
                while group[idx] != inf_u {
                    ans[idx] = i + 1;
                    idx = group[idx];
                }
                set.remove(&(j, size));
            } else {
                set.remove(&(j, size));
                set.insert((p, size + 1));
                group[p] = j;
            }
        } else {
            if K == 1 {
                ans[p] = i + 1;
            } else {
                set.insert((p, 1));
                group[p] = N;
            }
        }
    }
    // println!("{:?}", set);
    for a in ans {
        if a == inf_u {
            println!("{}", -1);
        } else {
            println!("{}", a);
        }
    }
}
