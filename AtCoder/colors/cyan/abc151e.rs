#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize, K:usize,
         mut A:[i64;N]
    }
    if K == 1 {
        println!("{}", 0);
        return;
    }
    A.sort();
    let md = md1000000007;
    let mut vec = vec![0; N];
    let mut memo = vec![0; N];
    let mut combs = mod_combination::new(1_000_000, md as usize);
    vec[K - 1] = 1;
    memo[K - 1] = A[K - 1];
    for i in K..N {
        vec[i] = vec[i - 1] + (combs.com(i - 1, K - 2) as i64);
        memo[i] = A[i] * vec[i] as i64;
        vec[i] %= md;
        memo[i] %= md;
    }

    vec = vec![0; N];
    let mut memo2 = vec![0; N];
    vec[N - K] = 1;
    memo2[N - K] = -A[N - K];
    for i in (0..N - K).rev() {
        vec[i] = vec[i + 1] + (combs.com(N - i - 2, K - 2) as i64);
        memo2[i] = -A[i] * vec[i] as i64;
        vec[i] %= md;
        memo2[i] %= md;
    }
    let mut ans = 0;
    for i in 0..N {
        ans += memo[i] + memo2[i];
        ans %= md;
    }
    println!("{}", ans % md);
}

struct mod_combination {
    fac: Vec<usize>,
    inv: Vec<usize>,
    finv: Vec<usize>,
    md: usize,
}

impl mod_combination {
    fn new(N: usize, md: usize) -> mod_combination {
        let mut fac = vec![0; N + 1];
        let mut inv = vec![0; N + 1];
        let mut finv = vec![0; N + 1];
        fac[0] = 1;
        fac[1] = 1;
        inv[0] = 1;
        inv[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        for i in 2..N + 1 {
            fac[i] = i * fac[i - 1] % md;
            inv[i] = md - inv[md % i] * (md / i) % md;
            finv[i] = finv[i - 1] * inv[i] % md;
        }
        mod_combination {
            fac: fac,
            inv: inv,
            finv: finv,
            md: md,
        }
    }
    fn com(&mut self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.md) % self.md
        }
    }
}
