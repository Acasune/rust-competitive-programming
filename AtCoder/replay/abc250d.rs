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
        N:usize,
    }
    let mut ans = 0;
    let ret = eratosthenes(1_000_000);
    let primes = ret
        .into_iter()
        .enumerate()
        .filter(|&(i, x)| x)
        .map(|(i, x)| i)
        .collect::<Vec<usize>>();
    let n = primes.len();
    for i in 0..n {
        let p = primes[i];
        for j in i + 1..n {
            let q = primes[j];
            if p * q > N / (q * q) {
                break;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn eratosthenes(size: usize) -> Vec<bool> {
    let mut is_primes = vec![true; size + 1];
    is_primes[0] = false;
    is_primes[1] = false;
    for i in 2..=size {
        if is_primes[i] {
            let mut j = 2 * i;
            while j <= size {
                is_primes[j] = false;
                j += i;
            }
        }
    }
    is_primes
}
