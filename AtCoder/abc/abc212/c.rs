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
        N:usize, M:usize,
        mut A:[i64;N],
        mut B:[i64;M]
    }
    let mut av = vec![-inf];
    A.sort();
    for a in A {
        av.push(a);
    }
    av.push(inf);
    let mut ans = inf;
    for b in B {
        let idx = av
            .binary_search_by_key(&(b * 2), |&a| 2 * a + 1)
            .err()
            .unwrap();
        ans = ans.min((b - av[idx]).abs());
        ans = ans.min((b - av[idx - 1]).abs());
    }
    println!("{}", ans);
}
