#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,
    }
    let mut memo = vec![String::new(); N + 1];
    memo[1] = String::from("1");

    for i in 2..=N {
        let mut a = memo[i - 1].clone();
        a.push_str(",");
        a.push_str(&(i as u32).to_string());
        a.push_str(",");
        a.push_str(&memo[i - 1]);
        memo[i] = a;
    }
    let ans = memo[N].split(",").collect::<Vec<&str>>();

    print!("{}", ans[0]);
    for i in 1..ans.len() {
        print!(" {}", ans[i]);
    }
    println!("");
}
