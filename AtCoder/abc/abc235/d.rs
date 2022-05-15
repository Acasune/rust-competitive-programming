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
          A:usize,
        N:usize,
    }
    let mut memo = vec![inf as usize; 10_000_000];
    dfs(1, A, &mut memo, 0);
    if memo[N] != inf as usize {
        println!("{}", memo[N]);
    } else {
        println!("{}", -1);
    }
}

fn dfs(s: usize, A: usize, memo: &mut Vec<usize>, cnt: usize) {
    if s >= 10_000_000 || memo[s] <= cnt {
        return;
    }
    let mut x = s;
    for i in 0..x.to_string().len() {
        if memo[x] > cnt + i {
            memo[x] = cnt + i;

            // println!("{}", x);
            dfs(x * A, A, memo, cnt + i + 1);
        }
        if x % 10 == 0 {
            break;
        }
        x = transform(x);
    }
}

fn transform(X: usize) -> usize {
    let mut x = X;
    let y = x % 10;
    x /= 10;
    let z = format!("{}{}", y, x).parse::<usize>().unwrap();
    z
}
