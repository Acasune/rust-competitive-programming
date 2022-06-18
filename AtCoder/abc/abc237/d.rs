#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

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
        mut S:Chars
    }
    let mut deq = VecDeque::new();
    deq.push_back(N);
    S.reverse();
    for (i, c) in S.into_iter().enumerate() {
        let n = N - i - 1;
        if c == 'R' {
            deq.push_front(n);
        } else {
            deq.push_back(n);
        }
    }
    print!("{}", deq.pop_front().unwrap());
    while let Some(i) = deq.pop_front() {
        print!(" {}", i);
    }
    println!();
}
