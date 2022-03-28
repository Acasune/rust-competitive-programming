#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Binary;
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize, Q:usize,
        mut S:Chars,
        QUERY:[(usize,Usize1,usize);Q],
    }
    let mut segtree = PURQ::new(N, (0i64, 0i64), |a, b| ((a.0.min(a.1 + b.0), a.1 + b.1)));
    for i in 0..N {
        match S[i] {
            '(' => segtree.update_tmp(i, (0, 1)),
            ')' => segtree.update_tmp(i, (-1, -1)),
            _ => unreachable!(),
        }
    }
    segtree.update_all();
    for (q, l, r) in QUERY {
        match q {
            1usize => {
                let r = r - 1;
                S.swap(l, r);
                for &x in [l, r].iter() {
                    if S[x] == '(' {
                        segtree.update(x, (0, 1));
                    } else {
                        segtree.update(x, (-1, -1));
                    }
                }
            }
            2usize => {
                let (lv, rv) = segtree.find(l, r);
                if lv == 0 && rv == 0 {
                    println!("{}", "Yes");
                } else {
                    println!("{}", "No");
                }
            }
            _ => unreachable!(),
        }
    }
}

// Referring to https://atcoder.jp/contests/abc223/submissions/26655204
//PURQ stands for Point Upgrade Range Query
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
        self.data[pos + self.n] = v;
    }
    fn update_all(&mut self) {
        let data = &mut self.data;
        for k in (1..self.n).rev() {
            data[k] = (self.op)(&data[2 * k], &data[2 * k + 1]);
        }
    }
    fn find(&self, l: usize, r: usize) -> T {
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
