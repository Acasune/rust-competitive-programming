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
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize,K:usize,
         AB:[(Usize1,Usize1);N-1],
    }
    let mut dp = vec![inf as usize; N];
    let mut edges = vec![vec![]; N];
    let md = md1000000007 as usize;
    for (a, b) in AB {
        edges[a].push(b);
        edges[b].push(a);
    }
    dp[0] = K;
    let mut deq = VecDeque::new();
    let mut k = K - 1;
    for &v in &edges[0] {
        if dp[v] != inf as usize {
            continue;
        }
        dp[v] = k;
        deq.push_back((v, 0));
        k -= 1;
    }

    while let Some((s, prev)) = deq.pop_front() {
        let mut k = K - 2;
        for &v in &edges[s] {
            if dp[v] != inf as usize {
                continue;
            }
            dp[v] = k;
            deq.push_back((v, s));
            k -= 1;
        }
    }
    let ans = dp.into_iter().fold(1usize, |cum, x| {
        if x == inf as usize {
            cum * 0usize
        } else {
            cum * x % md
        }
    });

    println!("{}", ans);
}
