#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize,K:usize,
    }
    let mut n = (N - 1) * (N - 2) / 2;
    if n < K {
        println!("{}", -1);
        return;
    }
    let mut ans = vec![];
    for i in 1..N {
        ans.push((0, i));
    }
    'outer: for i in 1..N {
        for j in i + 1..N {
            if n == K {
                break 'outer;
            }
            ans.push((i, j));
            n -= 1;
        }
    }
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
