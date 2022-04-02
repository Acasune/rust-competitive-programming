#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,X:Usize1,
        A:[Usize1;N],
    }
    let mut memo = vec![false; N];
    let mut que = VecDeque::<usize>::new();
    // memo[X] = true;
    que.push_back(X);
    while let Some(x) = que.pop_front() {
        // println!("{}", x);
        if !memo[x] {
            memo[x] = true;
            que.push_back(A[x]);
        }
    }
    // println!("{:?}", memo);
    println!("{}", memo.into_iter().filter(|x| *x).count());
}
