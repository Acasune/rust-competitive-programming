#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:i64,
    }
    let mut sieve = vec![true; 2_000_000 + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..2_000_000 + 1 {
        if !sieve[i] {
            continue;
        }
        let mut q = 2 * i;
        while q < 2_000_000 + 1 {
            sieve[q] = false;
            q += i;
        }
    }
    let mut primes = vec![];
    for i in 0..2_000_000 + 1 {
        if sieve[i] {
            primes.push(i as i64);
        }
    }

    let mut i = 2;
    let mut ans = 0;
    while 2 * i * i * i <= N {
        if !sieve[i as usize] {
            i += 1;
            continue;
        }
        let q = i * i * i;
        let mut r = primes.len();
        let mut l = 0;
        while r - l > 1 {
            let m = (r - l) / 2 + l;
            if primes[m] >= i || primes[m] > N / q {
                r = m;
            } else {
                l = m;
            }
        }

        if primes[l] != i && primes[l] * q <= N {
            ans += l + 1;
        } else if l > 0 && primes[l - 1] != i && primes[l - 1] * q <= N {
            ans += l;
        }

        i += 1;
    }
    println!("{}", ans);
}
