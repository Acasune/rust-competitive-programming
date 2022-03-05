#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        N:usize,
        AB:[(usize,usize);N-1],
        Q:usize,
        QS:[(usize,usize,i64);Q],
    }
    let mut vs = vec![0i64; N];
    let mut depths = vec![-1i64; N];
    let mut es = vec![Vec::<usize>::new(); N];
    for &(a, b) in &AB {
        es[a - 1].push(b - 1);
        es[b - 1].push(a - 1);
    }
    let mut que = VecDeque::<usize>::new();
    que.push_back(0);
    depths[0] = 0;
    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        for &i in &es[v] {
            if depths[i] == -1 {
                depths[i] = depths[v] + 1;
                que.push_back(i);
            }
        }
    }

    for (t, e, x) in QS {
        let mut t = t;
        let (mut a, mut b) = AB[e - 1];
        a -= 1;
        b -= 1;
        if depths[a] > depths[b] {
            a ^= b;
            b ^= a;
            a ^= b;
            t ^= 3;
        }
        if t == 1 {
            vs[0] += x;
            vs[b] -= x;
        } else {
            vs[b] += x;
        }
    }

    que = VecDeque::<usize>::new();
    que.push_back(0);
    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        for &i in &es[v] {
            if depths[i] > depths[v] {
                vs[i] += vs[v];
                que.push_back(i);
            }
        }
    }
    for val in vs {
        println!("{}", val);
    }
}
