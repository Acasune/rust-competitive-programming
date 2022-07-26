#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
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
        T:usize,
    }
    'outer: for _ in 0..T {
        input! {
            N:usize,
            LR:[(Usize1,usize);N]
        }
        let mut mp = BTreeMap::<usize, Vec<usize>>::new();
        for &(l, r) in &LR {
            let val = mp.entry(l).or_insert_with(|| vec![]);
            val.push(r);
        }
        let mut pos = 0;
        if let Some((i, _)) = mp.range(pos..).next() {
            pos = *i;
        }
        let mut heap = BinaryHeap::<Reverse<usize>>::new();
        'inner: loop {
            if let Some((i, _)) = mp.range(pos..).next() {
                let vec = mp.get(i).unwrap();
                heap.extend(vec.iter().cloned().map(|a| Reverse(a)));
            }
            if heap.len() == 0 {
                break;
            }
            let mut i_nxt = 1_000_000_000_000;
            if let Some((i, _)) = mp.range((pos + 1)..).next() {
                i_nxt = *i;
            }
            while pos < i_nxt {
                if let Some(Reverse(nxt)) = heap.pop() {
                    if pos >= nxt {
                        println!("{}", "No");
                        continue 'outer;
                    }
                    pos += 1;
                } else {
                    pos = i_nxt;
                    continue 'inner;
                }
            }
        }
        println!("{}", "Yes");
    }
}
