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
       N:usize,Q:usize,X:usize,
       W:[usize;N],
       K:[usize;Q],
    }
    if N == 1 {
        for k in K {
            println!("{}", (X + W[0] - 1) / W[0]);
        }
        return;
    }
    let mut tree = FenwickTree::new(N * 2, 0);
    let mut k = 1;
    for i in 0..2 * N {
        tree.add(i, W[i % N]);
    }
    let mut size = 0;
    let mut cut = 0;
    let mut mp = HashMap::new();

    let sum = W.iter().sum::<usize>();
    let mut l = 0;
    loop {
        let mut circle = 0;
        let mut x = X;
        if X > sum {
            circle = X / sum;
            x = X % sum;
        }
        let mut ll = l;
        let mut rr = 2 * N;
        if x != 0 || circle == 0 {
            while rr - ll > 1 {
                let m = ll + rr >> 1;
                if tree.query(l, m) >= x {
                    rr = m;
                } else {
                    ll = m;
                }
            }
            rr %= N;
        } else {
            rr = l;
        }

        if let Some(cnt) = mp.get(&(l, rr, circle)) {
            size = k - *cnt;
            cut = *cnt;
            break;
        }
        mp.insert((l, rr, circle), k);
        l = rr;
        k += 1;
    }
    // println!("{:?}", mp);
    let mut cnts = HashMap::new();
    for ((l, r, circle), cnt) in mp {
        if l < r {
            cnts.insert(cnt, r - l + N * circle);
        } else if l == r && circle > 0 {
            cnts.insert(cnt, N * circle);
        } else {
            cnts.insert(cnt, N + r - l + N * circle);
        }
    }
    // println!("{:?}", cnts);

    for ask in K {
        if ask <= cut {
            println!("{}", cnts.get(&ask).unwrap());
        } else {
            let k = (ask - cut) % size + cut;
            println!("{}", cnts.get(&(k)).unwrap());
        }
    }
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
