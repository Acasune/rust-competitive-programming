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
        C:usize,
        D:usize,
    }
    let mut ans = ((N+1)*N)/2;
    let A = C.min(D);
    let B = C.max(D);
    let is_dividable = B%A == 0;
    let a = N/A;
    let a_sum =(A+A*a)*a/2;
    let b = N/B;
    let b_sum =(B+B*b)*b/2;
    let (_,_,g) = ext_gcd(A as i64, B as i64);
    let ab_lcm = (A/(g as usize))*B;
    let ab = N/(ab_lcm);
    let ab_sum =(ab_lcm+(ab_lcm)*ab)*ab/2;

    if is_dividable {
        ans -= a_sum;
    } else {
        ans = ans + ab_sum-a_sum-b_sum;
    }
    println!("{}",ans);

}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}
