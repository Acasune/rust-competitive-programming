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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,
    }
    let mut ans = vec![];
    let mut r = M;
    let mut l = 0;
    while r > l {
        ans.push((l + 1, r + 1));
        r -= 1;
        l += 1;
    }
    l = M + 1;
    r = l + M - 1;
    while r > l {
        ans.push((l + 1, r + 1));
        r -= 1;
        l += 1;
    }
    for i in 0..M {
        let (l, r) = ans[i];
        println!("{} {}", l, r);
    }
}
