#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Sub;
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
        N:usize,K:usize,
        A:[usize;N],
    }
    if K == 1 {
        println!("{}", 0);
        return;
    }
    let mut sm = vec![];
    sm.push(0);
    for i in 0..N {
        sm.push((sm[i] + A[i]) % K);
    }
    let mut cnts = HashMap::new();
    *cnts.entry(0usize).or_insert(0usize) += 1;

    let mut ans = 0usize;
    for i in 1..=N {
        let a = (1_000_000_000 * K + sm[i] - i) % K;
        if let Some(val) = cnts.get(&a) {
            ans += val;
        }
        *cnts.entry(a).or_insert(0) += 1;
        if K - 1 <= i {
            let b = (1_000_000_000 * K + sm[i + 1 - K] - (i + 1 - K)) % K;
            *cnts.get_mut(&b).unwrap() -= 1;
        }
    }
    println!("{}", ans);
}
