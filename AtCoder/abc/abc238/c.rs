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
        N:i128,
    }
    let mut base = 10;
    let mut ans = 0;
    let  md = md998244353 as i128;
    while base/10 <= N {
        let mut a = base - base/10;
        a = (a +1)*a /2;
        let b = (N-base/10 +2) * (N-base/10+1)/2;
        ans += i128::min(a,b);
        ans %= md as i128;
        base *= 10;
    }
    println!("{}",ans);
}
