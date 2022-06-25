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
        N:usize,
        A:[usize;N],
    }
    let mut vec = vec![];
    let mut now = 0;
    vec.push(now);
    for a in A {
        now +=a;
        vec.push(now % 360);
    }
    vec.push(360);
    vec.sort();
    let mut ans = 0;
    for i in 1..vec.len() {
        ans = ans.max(vec[i] - vec[i-1]);
    }
    println!("{}",ans);

}