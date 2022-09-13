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

fn main() {
    input! {
        N:usize,mut X:i64,
        AB:[(i64,i64);N],
    }
    let mut ans = i64::MAX;
    let mut cum_time = 0;
    for i in 0..N {
        if i as i64 >= X {
            break;
        }
        cum_time += AB[i].0 + AB[i].1;
        ans = ans.min(cum_time + (X - (i as i64 + 1)) * AB[i].1);
    }
    println!("{}", ans);
}
