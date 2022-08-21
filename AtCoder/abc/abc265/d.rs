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
        N:usize,P:i64,Q:i64,R:i64,
        A:[i64;N]
    }
    let mut tree = FenwickTree::new(N + 1, 0);
    for i in 0..N {
        tree.add(i, A[i]);
    }
    for x in 0..N {
        let mut y = 0;
        let mut z = 0;
        let mut w = 0;

        let mut l = x;
        let mut r = N;
        while r - l > 1 {
            let m = (r + l) / 2;
            if P <= tree.query(x, m) {
                r = m;
            } else {
                l = m;
            }
        }
        y = r;
        let mut l = y;
        let mut r = N;
        while r - l > 1 {
            let m = (r + l) / 2;
            if Q <= tree.query(y, m) {
                r = m;
            } else {
                l = m;
            }
        }
        z = r;
        let mut l = z;
        let mut r = N;
        while r - l > 1 {
            let m = (r + l) / 2;
            if R <= tree.query(z, m) {
                r = m;
            } else {
                l = m;
            }
        }
        w = r;
        let p = tree.query(x, y);
        let q = tree.query(y, z);
        let r = tree.query(z, w);
        // println!("{} {} {} {} {} {} {}", x, y, z, w, p, q, r);
        if P == p && Q == q && R == r {
            // println!("{} {} {} {} {} {} {}", x, y, z, w, p, q, r);
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}

struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Clone,
    T: std::ops::AddAssign<T>,
{
    fn new(n: usize, e: T) -> Self {
        let size = n.next_power_of_two();
        FenwickTree {
            n: size,
            data: vec![e.clone(); size],
            e: e,
        }
    }
    fn add(&mut self, mut pos: usize, x: T) {
        pos += 1;
        while pos <= self.n {
            self.data[pos - 1] += x.clone();
            pos += pos & pos.wrapping_neg();
        }
    }
    fn sum(&self, mut pos: usize) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        let data = &self.data;
        let mut s = self.e.clone();
        while pos > 0 {
            s += data[pos - 1].clone();
            pos -= pos & pos.wrapping_neg();
        }
        s += data[pos].clone();
        s
    }

    fn query(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
        T: std::ops::Add<Output = T>,
    {
        self.sum(r) - self.sum(l)
    }
}
