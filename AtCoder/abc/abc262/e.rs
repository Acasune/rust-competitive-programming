#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,M:usize,K:usize,
        UV:[(Usize1,Usize1);M],
    }
    let mut G = vec![vec![]; N];
    for (u, v) in UV {
        G[u].push(v);
        G[v].push(u);
    }
    let mut cnt = 0;
    for i in 0..N {
        if G[i].len() % 2 == 1 {
            cnt += 1;
        }
    }
    let md = md998244353 as usize;
    let mut ans = 0;
    let mut com = mod_combination::new(N + 10, md);
    for m in 0..=K {
        if m % 2 == 1 {
            continue;
        }
        if m <= cnt && K - m <= N - cnt {
            ans += com.com(cnt, m) * com.com(N - cnt, K - m);
            ans %= md;
        }
    }

    println!("{}", ans);
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
    fn com(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.md) % self.md
        }
    }
    fn multi_choose(&self, n: usize, k: usize) -> usize {
        self.com(n + k - 1, k)
    }
}
