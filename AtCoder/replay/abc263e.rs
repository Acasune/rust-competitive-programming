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
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,
        mut A:[i64;N-1]
    }
    let md = md998244353;
    let mut purq = PURQ::new(N + 10, 0, |&x, &y| (x + y) % md);

    A.reverse();
    for i in 1..N {
        let a = A[i - 1];
        let mut sum = 1 + a + purq.find(i - a as usize, i);
        sum = sum * mod_pow(a, md - 2, md) % md;
        purq.update(i, sum);
    }
    let ans = purq.find(N - 1, N);
    println!("{}", ans);
}

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

fn mod_pow(base: i64, power: i64, md: i64) -> i64 {
    // base^power % md
    let mut ret = 1;
    let mut base = base;
    let mut power = power;
    while power > 0 {
        if power % 2 == 1 {
            ret = (ret % md) * (base % md);
            ret %= md;
        }
        base = (base % md) * (base % md);
        base %= md;
        power >>= 1;
    }
    return ret;
}
