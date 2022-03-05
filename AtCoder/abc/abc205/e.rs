#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

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
    fn comb(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.md) % self.md
        }
    }
}

fn main() {
    input! {
        N:usize,M:usize,K:usize,
    }
    let mut md = 1_000_000_007;
    if N > M + K {
        println!("{}", 0);
        return;
    }
    let comb = mod_combination::new(N + M, md);
    let ans = md + comb.comb(N + M, M) - comb.comb(N + M, M + K + 1);
    println!("{}", ans % md);
}
