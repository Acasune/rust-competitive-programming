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
        L:i64,
        R:i64,
        mut A:[i64;N],
    }
    let mut a_sum = vec![vec![0i64, 0i64]; N + 1];
    for i in 1..=N {
        a_sum[i][0] = a_sum[i - 1][0] + L;
        a_sum[i][1] = i64::min(a_sum[i - 1][0], a_sum[i - 1][1]) + A[i - 1];
    }
    let mut b_sum = vec![vec![0i64, 0i64]; N + 1];
    A.reverse();
    for i in 1..=N {
        b_sum[i][0] = b_sum[i - 1][0] + R;
        b_sum[i][1] = i64::min(b_sum[i - 1][0], b_sum[i - 1][1]) + A[i - 1];
    }
    b_sum.reverse();
    let mut ans = inf_i;
    for i in 0..N + 1 {
        let a = a_sum[i][0].min(a_sum[i][1]);
        let b = b_sum[i][0].min(b_sum[i][1]);
        ans = ans.min(a + b);
    }
    println!("{}", ans);
}
