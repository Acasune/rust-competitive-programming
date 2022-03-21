#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,
       mut A:[usize;N],
    }
    A.sort();
    let upper = A[N - 1];
    let mut bases = HashSet::<usize>::new();
    for i in 0..N {
        let mut a = A[i];
        let mut j = 2;
        while j * j <= a && j <= M {
            if a % j == 0 {
                bases.insert(j);
                while a % j == 0 {
                    a /= j;
                }
            }
            j += 1;
        }
        if a != 1 {
            bases.insert(a);
        }
    }
    let mut memo = vec![false; M + 1];
    for i in bases.into_iter() {
        let mut j = i;
        while j <= M {
            memo[j] = true;
            j += i;
        }
    }
    let mut ans = vec![];
    for i in 1..=M {
        if !memo[i] {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}
