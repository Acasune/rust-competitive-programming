#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        Q:usize,
        ask:[(i64,i64);Q],
    }
    let m = 1048576;
    // let m = 5;
    let mut A = vec![inf as usize; m];
    let mut memo = vec![VecDeque::<usize>::new(); m];
    let mut heap = BinaryHeap::<Reverse<usize>>::new();
    for (i, &(t, x)) in ask.iter().enumerate() {
        if t == 1 {
            memo[(x % m as i64) as usize].push_back(i);
        }
    }
    for i in 0..memo.len() {
        for j in 0..memo[i].len() {
            let idx = memo[i].pop_front().unwrap();
            heap.push(Reverse(idx));
        }
        if let Some(Reverse(idx)) = heap.pop() {
            A[i] = idx;
        }
    }
    let mut i = 0usize;
    while let Some(Reverse(idx)) = heap.pop() {
        if A[i % m] != inf as usize {
            let tmp = A[i % m];
            A[i % m] = idx.min(tmp);
            heap.push(Reverse(idx.max(tmp)));
        } else {
            A[i % m] = idx;
        }
        i += 1;
    }
    // println!("{:?}", A);
    for (i, &(t, x)) in ask.iter().enumerate() {
        if t == 2 {
            let x = x % m as i64;
            if i > A[x as usize] {
                println!("{}", ask[A[x as usize] as usize].1);
            } else {
                println!("{}", -1);
            }
        }
    }
}
