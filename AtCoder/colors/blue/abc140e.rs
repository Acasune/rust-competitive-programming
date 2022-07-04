#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
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
        N:usize,
        P:[i64;N],
    }
    let mut tree = PURQ::new(N, 0, |&a, &b| a.max(b));
    for i in 0..N {
        tree.update_tmp(i, P[i]);
    }
    tree.update_all();
    let mut ans = 0;

    for i in 0..N {
        let mut a = -1;
        let mut b = -1;
        let mut c = -1;
        let mut d = N as i64;
        let mut comb = 0i64;

        if P[i] < tree.find(0, i) {
            let mut r = i as i64;
            let mut l = 0;
            while r - l > 1 {
                let m = ((r + l) / 2) as usize;
                if P[i] < tree.find(m, i) {
                    l = m as i64;
                } else {
                    r = m as i64;
                }
            }
            b = l as i64;
            if P[i] < tree.find(0, b as usize) {
                let mut r = b;
                let mut l = 0;
                while r - l > 1 {
                    let m = ((r + l) / 2) as usize;
                    if P[i] < tree.find(m, b as usize) {
                        l = m as i64;
                    } else {
                        r = m as i64;
                    }
                }
                a = l;
            }
        } else {
            b = -1;
        }
        if P[i] < tree.find(i, N) {
            let mut r = N as i64;
            let mut l = i as i64;
            while r - l > 1 {
                let m = ((r + l) / 2) as usize;
                if P[i] < tree.find(i, m) {
                    r = m as i64;
                } else {
                    l = m as i64;
                }
            }
            c = r - 1 as i64;
            if P[i] < tree.find(c as usize + 1, N) {
                let mut r = N as i64;
                let mut l = c + 1;
                while r - l > 1 {
                    let m = ((r + l) / 2) as usize;
                    if P[i] < tree.find(c as usize + 1, m) {
                        r = m as i64;
                    } else {
                        l = m as i64;
                    }
                }
                d = r - 1;
            }
        } else {
            c = N as i64;
        }
        if 0 <= b {
            comb += 1 * (b - a) * (c - i as i64);
        }
        if c < N as i64 {
            comb += (d - c) * (i as i64 - b);
        }
        ans += comb * P[i];
    }
    println!("{}", ans);
}

// Referring to https://atcoder.jp/contests/abc223/submissions/26655204
//PURQ stands for Point Update Range Query
struct PURQ<T, F> {
    n: usize,
    data: Vec<T>,
    e: T,
    op: F,
}

impl<T, F> PURQ<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    fn new(n: usize, e: T, op: F) -> Self {
        let size = n.next_power_of_two();
        PURQ {
            n: size,
            data: vec![e.clone(); 2 * size],
            e: e,
            op: op,
        }
    }
    fn update(&mut self, pos: usize, v: T) {
        assert!(pos < self.n);
        let mut pos = pos + self.n;
        let data = &mut self.data;
        data[pos] = v;
        pos >>= 1;
        while pos > 0 {
            data[pos] = (self.op)(&data[2 * pos], &data[2 * pos + 1]);
            pos >>= 1;
        }
    }
    fn update_tmp(&mut self, pos: usize, v: T) {
        assert!(pos < self.n);
        self.data[pos + self.n] = v;
    }
    fn update_all(&mut self) {
        let data = &mut self.data;
        for k in (1..self.n).rev() {
            data[k] = (self.op)(&data[2 * k], &data[2 * k + 1]);
        }
    }
    fn find(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e.clone();
        }
        let mut p = self.e.clone();
        let mut q = self.e.clone();
        let mut l = l + self.n;
        let mut r = r + self.n;
        let data = &self.data;
        while l < r {
            if l & 1 == 1 {
                p = (self.op)(&p, &data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                q = (self.op)(&data[r], &q)
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&p, &q)
    }
}
