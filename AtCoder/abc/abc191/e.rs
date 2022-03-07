#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        N:usize, M:usize,
        ABC:[(usize,usize,i64);M],
    }
    let mut to = vec![vec![]; N];
    let mut from = vec![vec![]; N];
    for &(a, b, c) in &ABC {
        to[a - 1].push((c, b - 1));
        from[b - 1].push((c, a - 1));
    }
    for i in 0..N {
        let ans = dijkstra(i, N, &to, &from);
        if ans == 1_000_000_000_000i64 {
            println!("{}", -1);
        } else {
            println!("{}", ans);
        }
    }
}

fn dijkstra(s: usize, N: usize, to: &Vec<Vec<(i64, usize)>>, from: &Vec<Vec<(i64, usize)>>) -> i64 {
    let mut memo = vec![1_000_000_000_000i64; N];
    memo[s] = 0;
    let mut que = BinaryHeap::<(i64, usize)>::new();
    que.push((0, s));
    while !que.is_empty() {
        let (cc, s) = que.pop().unwrap();
        let cc = -cc;
        if memo[s] < cc {
            continue;
        }
        for &(c, nxt) in &to[s] {
            if memo[nxt] > memo[s] + c {
                memo[nxt] = memo[s] + c;
                que.push((-c, nxt));
            }
        }
    }
    let mut ret = 1_000_000_000_000i64;
    for &(e, g) in &from[s] {
        ret = ret.min(memo[g] + e);
    }

    return ret;
}
