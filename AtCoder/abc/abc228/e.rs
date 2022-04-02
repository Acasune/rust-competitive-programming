#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: usize = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize, K:usize,M:usize,
    }
    if M % md == 0 {
        println!("{}", 0);
        return;
    }
    let r = mod_pow(K, N, md - 1);
    let ans = mod_pow(M, r, md);
    println!("{}", ans % md);
}

fn mod_pow(base: usize, power: usize, p: usize) -> usize {
    let mut ret = 1;
    let mut base = base;
    let mut power = power;
    while power > 0 {
        if power % 2 == 1 {
            ret = (ret % p) * (base % p);
            ret %= p;
        }
        base = (base % p) * (base % p);
        base %= p;
        power >>= 1;
    }
    return ret;
}
