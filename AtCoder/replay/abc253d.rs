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
        N:usize,A:usize,B:usize
    }
    let mut ans = N * (N + 1) / 2;
    let g = lcm(A as i64, B as i64) as usize;
    let m3 = N / g;
    ans += g * (m3 * (m3 + 1) / 2);
    let m1 = N / A;
    ans -= A * (m1 * (m1 + 1) / 2);
    let m2 = N / B;
    ans -= B * (m2 * (m2 + 1) / 2);
    println!("{}", ans);
}
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    ext_gcd(a, b).2
}

fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    (a / g) * b
}
