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

fn init(N: usize, md: usize) -> (Vec<usize>, Vec<usize>) {
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
        // println!("{}, {:?}", md % i, inv[md % i] * (md / i) % md);
        fac[i] = i * fac[i - 1] % md;
        inv[i] = md - inv[md % i] * (md / i) % md;
        finv[i] = finv[i - 1] * inv[i] % md;
    }
    (fac, finv)
}

fn main() {
    input! {
        N:usize,M:usize,K:usize,
    }
    let mut md = 998244353;
    let (fac, finv) = init(N, md);
    let mut ans = 0;
    for k in 0..=K {
        let mut tmp = M;
        let mut i = N - k - 1;
        let mut base = M - 1;
        while i > 0 {
            if i % 2 == 1 {
                tmp *= base;
                tmp %= md;
            }
            base *= base;
            base %= md;
            i >>= 1;
        }
        ans += (tmp * fac[N - 1] % md * finv[(N - k - 1)] % md * finv[k] % md) % md;
        ans %= md;
    }
    println!("{}", ans);
}
