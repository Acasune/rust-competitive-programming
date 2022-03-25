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

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        Q:usize,
    }
    let mut heap = BinaryHeap::<Reverse<i64>>::new();
    let mut vec = VecDeque::new();
    for _ in 0..Q {
        input! {
            a:usize,
        }
        if a == 1 {
            input! {
                x:i64,
            }
            vec.push_back(x);
        } else if a == 2 {
            if heap.is_empty() {
                let x = vec.pop_front().unwrap();
                println!("{}", x);
            } else {
                let Reverse(x) = heap.pop().unwrap();
                println!("{}", x);
            }
        } else {
            while let Some(x) = vec.pop_front() {
                heap.push(Reverse(x));
            }
        }
    }
}
