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
        N:usize,K:usize,
        A:[Usize1;K],
        XY:[(i64,i64);N],
    }
    let mut ans = 0;
    for i in 0..N {
        let mut r = inf_i;

        let (x,y) = XY[i];
        for j in 0..K {
            let (s,t) = XY[A[j]];
            let dist = (x-s)* (x-s)+ (y-t)* (y-t);
            r = r.min(dist);
        }
        ans = ans.max(r);
    }
    println!("{}",(ans as f64).sqrt());
}
