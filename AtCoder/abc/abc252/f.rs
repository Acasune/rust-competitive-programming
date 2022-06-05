#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize, L:usize,
        A:[usize;N],
    }
    let sum = A.iter().sum::<usize>();
    let remain = L - sum;
    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    if remain != 0 {
        heap.push(Reverse(remain));
    }

    for a in A {
        heap.push(Reverse(a));
    }
    while let Some(Reverse(a)) = heap.pop() {
        let b = heap.pop();
        if b.is_none() {
        } else {
            let Reverse(b) = b.unwrap();
            ans += a + b;
            heap.push(Reverse(a + b));
        }
    }
    println!("{}", ans);
}
